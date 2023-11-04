mod pattern_ty;

pub(crate) use self::pattern_ty::*;

use crate::*;
use husky_print_utils::p;
use husky_syn_expr::*;
use husky_syn_opr::{SynBinaryOpr, SynPrefixOpr};
use husky_token_data::{IntegerLikeLiteralData, LiteralData};
use salsa::DebugWithDb;

pub(super) struct DeclarativeTermEngine<'a> {
    db: &'a dyn DeclarativeSignatureDb,
    syn_expr_region_data: &'a SynExprRegionData,
    declarative_term_menu: &'a DeclarativeTermMenu,
    symbol_declarative_term_region: SymbolDeclarativeTermRegion,
    expr_terms: SynExprMap<DeclarativeTermResult2<DeclarativeTerm>>,
    /// todo: change this to ordered
    pattern_expr_ty_infos: SynPatternExprMap<PatternExprDeclarativeTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolDeclarativeTypeInfo>,
}

#[salsa::tracked(jar = DeclarativeSignatureJar, return_ref)]
pub(crate) fn declarative_term_region(
    db: &dyn DeclarativeSignatureDb,
    syn_expr_region: SynExprRegion,
) -> DeclarativeTermRegion {
    let expr_region_data = syn_expr_region.data(db);
    let parent_expr_region = expr_region_data.parent();
    let parent_term_symbol_region =
        parent_expr_region.map(|r| declarative_term_region(db, r).term_symbol_region());
    let mut engine = DeclarativeTermEngine::new(db, syn_expr_region, parent_term_symbol_region);
    engine.infer_all()
}

impl<'a> DeclarativeTermEngine<'a> {
    fn new(
        db: &'a dyn DeclarativeSignatureDb,
        syn_expr_region: SynExprRegion,
        parent_term_symbol_region: Option<&'a SymbolDeclarativeTermRegion>,
    ) -> Self {
        let toolchain = syn_expr_region.toolchain(db);
        // ad hoc
        let _item_path_menu = db.item_path_menu(toolchain);
        let declarative_term_menu = db.declarative_term_menu(toolchain).unwrap();
        let syn_expr_region_data = &syn_expr_region.data(db);
        Self {
            db,
            syn_expr_region_data,
            declarative_term_menu,
            symbol_declarative_term_region: SymbolDeclarativeTermRegion::new(
                parent_term_symbol_region,
                syn_expr_region_data,
                declarative_term_menu,
            ),
            expr_terms: SynExprMap::new(syn_expr_region_data.expr_arena()),
            pattern_expr_ty_infos: SynPatternExprMap::new(
                syn_expr_region_data.pattern_expr_arena(),
            ),
            pattern_symbol_ty_infos: SynPatternSymbolMap::new(
                syn_expr_region_data
                    .pattern_expr_region()
                    .pattern_symbol_arena(),
            ),
        }
    }

    fn infer_all(mut self) -> DeclarativeTermRegion {
        self.infer_current_symbol_terms();
        self.symbol_declarative_term_region
            .infer_self_ty_parameter_and_self_value_parameter(
                self.db,
                self.syn_expr_region_data.path(),
                self.syn_expr_region_data.symbol_region(),
            );
        self.infer_expr_roots();
        self.finish()
    }

    fn infer_current_symbol_terms(&mut self) {
        let mut current_symbol_indexed_iter = self
            .syn_expr_region_data
            .symbol_region()
            .current_symbol_indexed_iter();
        for (pattern_ty_constraint, symbols) in self
            .syn_expr_region_data
            .symbol_region()
            .pattern_ty_constraints()
        {
            match pattern_ty_constraint {
                SyndicateTypeConstraint::TemplateTypeParameter => {
                    let (current_symbol_idx, current_symbol) = current_symbol_indexed_iter
                        .next()
                        .expect("ty constraint should match with current symbols");
                    let SynCurrentSymbolVariant::TemplateParameter {
                        syn_attrs,
                        annotated_variance_token,
                        template_parameter_variant,
                    } = current_symbol.variant()
                    else {
                        unreachable!()
                    };
                    let attrs = DeclarativeTemplateSymbolAttrs::from_attrs(syn_attrs.iter().map(
                        |syn_attr| match syn_attr {
                            TemplateSymbolSynAttr::Phantom(_, _) => todo!(),
                        },
                    ));
                    let variance = annotated_variance_token.map(|vt| vt.into());
                    let (ty, term_symbol) = match template_parameter_variant {
                        CurrentTemplateParameterSynSymbolVariant::Lifetime { label_token } => {
                            DeclarativeTermSymbol::new_lifetime(
                                self.db,
                                self.declarative_term_menu,
                                &mut self.symbol_declarative_term_region.symbol_registry_mut(),
                                attrs,
                                variance,
                            )
                        }
                        CurrentTemplateParameterSynSymbolVariant::Place { label_token } => {
                            DeclarativeTermSymbol::new_place(
                                self.db,
                                self.declarative_term_menu,
                                &mut self.symbol_declarative_term_region.symbol_registry_mut(),
                                attrs,
                                variance,
                            )
                        }
                        CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. } => {
                            DeclarativeTermSymbol::new_ty(
                                self.db,
                                self.declarative_term_menu,
                                &mut self.symbol_declarative_term_region.symbol_registry_mut(),
                                attrs,
                                variance,
                            )
                        }
                        CurrentTemplateParameterSynSymbolVariant::Constant {
                            ident_token,
                            ty_expr_idx,
                        } => {
                            let ty = self.infer_new_expr_term(*ty_expr_idx).map_err(Into::into);
                            if ty.is_err() {
                                p!(
                                    ident_token.debug(self.db),
                                    self.syn_expr_region_data.path().debug(self.db)
                                );
                                p!(self.syn_expr_region_data[*ty_expr_idx].debug(self.db));
                                p!(self.expr_terms[*ty_expr_idx].debug(self.db));
                                todo!()
                            }
                            (
                                ty,
                                DeclarativeTermSymbol::new_const(
                                    self.db,
                                    attrs,
                                    ty,
                                    &mut self.symbol_declarative_term_region.symbol_registry_mut(),
                                ),
                            )
                        }
                        _ => todo!(),
                    };
                    // let term_symbol = self.symbol_registry.new_symbol(db, ty);
                    self.symbol_declarative_term_region
                        .add_new_template_parameter_symbol_signature(
                            self.db,
                            symbols.start(),
                            ty,
                            term_symbol,
                        )
                }
                SyndicateTypeConstraint::ExplicitRegularParameter {
                    syn_pattern_root,
                    ty_expr_idx: ty,
                } => self.init_current_symbol_signatures_in_parenate_parameter(
                    *syn_pattern_root,
                    *ty,
                    *symbols,
                ),
                SyndicateTypeConstraint::FieldVariable {
                    ident_token,
                    ty_expr_idx,
                } => {
                    let ty = self.infer_new_expr_term(*ty_expr_idx).map_err(Into::into);
                    self.symbol_declarative_term_region
                        .add_new_field_variable_symbol_signature(self.db, symbols.start(), ty)
                }
                SyndicateTypeConstraint::LetPattern { .. }
                | SyndicateTypeConstraint::FrameVariable => {
                    // need only to compute for decl region
                    return;
                }
                SyndicateTypeConstraint::ExplicitVariadicParameter { ty } => {
                    let ty = self.infer_new_expr_term(*ty).map_err(|_| todo!());
                    let symbol = DeclarativeTermSymbol::new_ephem(
                        self.db,
                        ty,
                        &mut self.symbol_declarative_term_region.symbol_registry_mut(),
                    );
                    self.symbol_declarative_term_region
                        .add_new_template_parameter_symbol_signature(
                            self.db,
                            symbols.start(),
                            ty,
                            symbol,
                        )
                }
            }
        }
    }

    /// explicit parameters are infered in this crate;
    ///
    /// let variables, be variables and match variables are infered in `husky-expr-ty`
    fn init_current_symbol_signatures_in_parenate_parameter(
        &mut self,
        syn_pattern_root: SynPatternRoot,
        ty: SynExprIdx,
        symbols: SynCurrentSymbolIdxRange,
    ) {
        let Ok(ty) = self.infer_new_expr_term(ty) else {
            for symbol in symbols {
                let modifier = self.syn_expr_region_data[symbol].modifier();
                self.symbol_declarative_term_region
                    .add_new_parenate_parameter_symbol_signature(
                        self.db,
                        symbol,
                        modifier,
                        Err(DeclarativeTermSymbolTypeErrorKind::CannotInferTypeExprTerm(
                            self.syn_expr_region_data.path(),
                        )),
                    )
            }
            return;
        };
        self.infer_pattern_tys_in_parenate_parameter(syn_pattern_root, ty);
        for symbol in symbols {
            self.infer_current_symbol_signature_in_parenate_parameter(symbol)
        }
    }

    fn infer_current_symbol_signature_in_parenate_parameter(
        &mut self,
        current_symbol_idx: SynCurrentSymbolIdx,
    ) {
        let current_symbol = &self.syn_expr_region_data.symbol_region()[current_symbol_idx];
        match current_symbol.variant() {
            SynCurrentSymbolVariant::ParenateRegularParameter {
                ident,
                pattern_symbol_idx,
            } => {
                let base_ty = self.pattern_symbol_ty_infos[pattern_symbol_idx].base_ty();
                self.symbol_declarative_term_region
                    .add_new_parenate_parameter_symbol_signature(
                        self.db,
                        current_symbol_idx,
                        current_symbol.modifier(),
                        Ok(base_ty),
                    )
            }
            _ => unreachable!("this function is only used for explicit parameters"),
        }
    }

    fn infer_expr_roots(&mut self) {
        for expr_root in self.syn_expr_region_data.roots() {
            match expr_root.kind() {
                // omit props struct field because they are inferred for field variable
                ExprRootKind::PropsStructFieldType { .. } => continue,
                ExprRootKind::Trait
                | ExprRootKind::ReturnType
                | ExprRootKind::TupleStructFieldType
                | ExprRootKind::ReturnType
                | ExprRootKind::ExplicitParameterDefaultValue { .. }
                | ExprRootKind::AssociatedTypeTerm => (),
                ExprRootKind::SelfType => {
                    let self_ty_term = self.infer_new_expr_term(expr_root.syn_expr_idx()).ok();
                    self.symbol_declarative_term_region
                        .set_self_ty_term(self_ty_term);
                    continue;
                }
                ExprRootKind::BlockExpr
                | ExprRootKind::LetStmtType
                | ExprRootKind::LetStmtInitialValue
                | ExprRootKind::HtmlArgumentExpr
                | ExprRootKind::ReturnExpr
                | ExprRootKind::Condition
                | ExprRootKind::ConstantImplicitParameterType
                | ExprRootKind::ExplicitParameterType
                | ExprRootKind::FieldBindInitialValue { .. }
                | ExprRootKind::Snippet
                | ExprRootKind::ValExpr
                | ExprRootKind::EvalExpr => continue,
                // ad hoc
                ExprRootKind::Traits => (),
            }
            self.cache_new_expr_term(expr_root.syn_expr_idx())
        }
    }

    // infer the term for expr, assuming it hasn't been computed before
    fn infer_new_expr_term(
        &mut self,
        expr_idx: SynExprIdx,
    ) -> DeclarativeTermResult2<DeclarativeTerm> {
        let result = self.calc_expr_term(expr_idx);
        let result_export = match result {
            Ok(term) => Ok(term),
            Err(_) => Err(DerivedDeclarativeTermError2::DeclarativeTermAbortion.into()),
        };
        self.save_expr_term(expr_idx, result);
        result_export
    }

    // cache the term for expr, assuming it hasn't been computed before
    fn cache_new_expr_term(&mut self, expr_idx: SynExprIdx) {
        let result = self.calc_expr_term(expr_idx);
        self.save_expr_term(expr_idx, result)
    }

    pub(crate) fn finish(self) -> DeclarativeTermRegion {
        DeclarativeTermRegion::new(
            self.syn_expr_region_data.path(),
            self.symbol_declarative_term_region,
            self.expr_terms,
            self.pattern_expr_ty_infos,
            self.pattern_symbol_ty_infos,
        )
    }

    #[track_caller]
    fn save_expr_term(
        &mut self,
        expr_idx: SynExprIdx,
        outcome: DeclarativeTermResult2<DeclarativeTerm>,
    ) {
        self.expr_terms.insert_new(expr_idx, outcome)
    }

    fn calc_expr_term(&mut self, expr_idx: SynExprIdx) -> DeclarativeTermResult2<DeclarativeTerm> {
        match self.syn_expr_region_data.expr_arena()[expr_idx] {
            SynExprData::Literal(token_idx, literal) => match literal {
                LiteralData::Unit => todo!(),
                LiteralData::Char(_) => todo!(),
                LiteralData::String(_) => todo!(),
                LiteralData::Integer(literal) => match literal {
                    IntegerLikeLiteralData::UnspecifiedRegular(i) => Ok(DeclarativeTerm::Literal(
                        UnresolvedTermLiteral::RegularInteger(i).into(),
                    )),
                    IntegerLikeLiteralData::UnspecifiedLarge() => todo!(),
                    IntegerLikeLiteralData::I8(_) => todo!(),
                    IntegerLikeLiteralData::I16(_) => todo!(),
                    IntegerLikeLiteralData::I32(_) => todo!(),
                    IntegerLikeLiteralData::I64(_) => todo!(),
                    IntegerLikeLiteralData::I128(_) => todo!(),
                    IntegerLikeLiteralData::ISize(_) => todo!(),
                    IntegerLikeLiteralData::R8(_) => todo!(),
                    IntegerLikeLiteralData::R16(_) => todo!(),
                    IntegerLikeLiteralData::R32(_) => todo!(),
                    IntegerLikeLiteralData::R64(_) => todo!(),
                    IntegerLikeLiteralData::R128(_) => todo!(),
                    IntegerLikeLiteralData::RSize(_) => todo!(),
                    IntegerLikeLiteralData::U8(_) => todo!(),
                    IntegerLikeLiteralData::U16(_) => todo!(),
                    IntegerLikeLiteralData::U32(_) => todo!(),
                    IntegerLikeLiteralData::U64(_) => todo!(),
                    IntegerLikeLiteralData::U128(_) => todo!(),
                    IntegerLikeLiteralData::USize(_) => todo!(),
                },
                LiteralData::Float(_) => todo!(),
                LiteralData::TupleIndex(_) => todo!(),
                LiteralData::Bool(_) => todo!(),
            },
            SynExprData::PrincipalEntityPath { opt_path, .. } => match opt_path {
                Some(path) => Ok(DeclarativeTerm::EntityPath(match path {
                    PrincipalEntityPath::Module(_) => todo!(),
                    PrincipalEntityPath::MajorItem(path) => match path {
                        MajorItemPath::Type(path) => DeclarativeTermEntityPath::Type(path),
                        MajorItemPath::Trait(path) => path.into(),
                        MajorItemPath::Fugitive(path) => path.into(),
                    },
                    PrincipalEntityPath::TypeVariant(path) => {
                        DeclarativeTermEntityPath::TypeVariant(path)
                    }
                })),
                None => Err(DerivedDeclarativeTermError2::InvalidEntityPath.into()),
            },
            SynExprData::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
            } => todo!(),
            SynExprData::InheritedSymbol {
                inherited_symbol_idx,
                ..
            } => self
                .symbol_declarative_term_region
                .inherited_symbol_signature(inherited_symbol_idx)
                .term_symbol()
                .map(Into::into)
                .ok_or(DerivedDeclarativeTermError2::InheritedSymbolIsNotValidTerm.into()),
            SynExprData::CurrentSymbol {
                current_symbol_idx, ..
            } => Ok(self
                .symbol_declarative_term_region
                .current_symbol_signature(current_symbol_idx)
                .expect("not none")
                .term_symbol()
                .ok_or(OriginalDeclarativeTermError2::InvalidSymbolForTerm)?
                .into()),
            SynExprData::FrameVarDecl { .. } => unreachable!(),
            SynExprData::SelfType(_) => self
                .symbol_declarative_term_region
                .self_ty_term()
                .ok_or(DerivedDeclarativeTermError2::SelfTypeNotAllowedInThisRegion.into()),
            SynExprData::SelfValue(_) => self
                .symbol_declarative_term_region
                .self_ty_term()
                .ok_or(DerivedDeclarativeTermError2::SelfValueNotAllowedInThisRegion.into()),
            SynExprData::Binary {
                lopd, opr, ropd, ..
            } => {
                let Ok(lopd) = self.infer_new_expr_term(lopd) else {
                    return Err(
                        DerivedDeclarativeTermError2::CannotInferOperandDeclarativeTermInPrefix
                            .into(),
                    );
                };
                let Ok(ropd) = self.infer_new_expr_term(ropd) else {
                    return Err(
                        DerivedDeclarativeTermError2::CannotInferOperandDeclarativeTermInPrefix
                            .into(),
                    );
                };
                match opr {
                    SynBinaryOpr::Closed(_) => todo!(),
                    SynBinaryOpr::Shift(_) => todo!(),
                    SynBinaryOpr::Comparison(_) => todo!(),
                    SynBinaryOpr::ShortCircuitLogic(_) => todo!(),
                    SynBinaryOpr::Assign => todo!(),
                    SynBinaryOpr::AssignClosed(_) => todo!(),
                    SynBinaryOpr::AssignShift(_) => todo!(),
                    SynBinaryOpr::ScopeResolution => todo!(),
                    SynBinaryOpr::CurryType => Ok(DeclarativeTermCurry::new_nondependent(
                        self.db,
                        CurryKind::Explicit, // ad hoc
                        Variance::Invariant, // ad hoc
                        lopd,
                        ropd,
                    )
                    .into()),
                    SynBinaryOpr::As => todo!(),
                    SynBinaryOpr::Ins => todo!(),
                    SynBinaryOpr::In => todo!(),
                }
            }
            SynExprData::Be { .. } => todo!(),
            SynExprData::Prefix { opr, opd, .. } => {
                let Ok(opd) = self.infer_new_expr_term(opd) else {
                    return Err(
                        DerivedDeclarativeTermError2::CannotInferOperandDeclarativeTermInPrefix
                            .into(),
                    );
                };
                let tmpl = match opr {
                    SynPrefixOpr::Minus => todo!(),
                    SynPrefixOpr::Not => todo!(),
                    SynPrefixOpr::Tilde => DeclarativeTerm::LeashOrBitNot(
                        self.syn_expr_region_data.path().toolchain(self.db),
                    ),
                    SynPrefixOpr::Ref => todo!(),
                    SynPrefixOpr::Option => self.declarative_term_menu.option_ty_path(),
                };
                Ok(DeclarativeTermExplicitApplication::new(self.db, tmpl, opd).into())
            }
            SynExprData::Suffix { .. } => todo!(),
            SynExprData::Field { .. } => todo!(),
            SynExprData::MethodApplicationOrCall { .. } => todo!(),
            SynExprData::TemplateInstantiation { .. } => todo!(),
            SynExprData::FunctionApplicationOrCall {
                function,
                template_arguments: ref generic_arguments,
                ref items,
                ..
            } => {
                let Ok(function) = self.infer_new_expr_term(function) else {
                    return Err(
                        DerivedDeclarativeTermError2::CannotInferArgumentDeclarativeTermInApplication.into()
                    );
                };
                let generic_arguments = match generic_arguments {
                    Some(generic_arguments) => generic_arguments
                        .arguments()
                        .into_iter()
                        .map(|_| todo!())
                        .collect(),
                    None => vec![],
                };
                let extra_comma = match items.last() {
                    Some(last_item) => last_item.comma_regional_token_idx().is_some(),
                    None => false,
                };
                let items = items
                    .into_iter()
                    .map(|item| self.infer_new_expr_term(item.syn_expr_idx()))
                    .collect::<DeclarativeTermResult2<_>>()?;
                Ok(DeclarativeTermExplicitApplicationOrRitchieCall::new(
                    self.db,
                    function,
                    generic_arguments,
                    items,
                    extra_comma,
                )
                .into())
            }
            SynExprData::ExplicitApplication {
                function_expr_idx: function,
                argument_expr_idx: argument,
            } => {
                let Ok(argument) = self.infer_new_expr_term(argument) else {
                    Err(DerivedDeclarativeTermError2::CannotInferArgumentDeclarativeTermInApplication)?
                };
                let Ok(function) = self.infer_new_expr_term(function) else {
                    Err(DerivedDeclarativeTermError2::CannotInferFunctionDeclarativeTermInApplication)?
                };
                Ok(DeclarativeTermExplicitApplication::new(self.db, function, argument).into())
            }
            SynExprData::NewTuple { ref items, .. } => {
                p!(self.syn_expr_region_data.path().debug(self.db));
                p!(items.len());
                todo!()
            }
            SynExprData::List { ref items, .. } => {
                let items = items
                    .iter()
                    .map(|item| self.infer_new_expr_term(item.syn_expr_idx()))
                    .collect::<DeclarativeTermResult2<Vec<_>>>()?;
                Ok(DeclarativeTermList::new(
                    self.db,
                    self.syn_expr_region_data.path().toolchain(self.db),
                    items,
                )
                .into())
            }
            SynExprData::BoxColonList { ref items, .. } => match items.len() {
                0 => Ok(self.declarative_term_menu.slice_ty_path()),
                _ => todo!(),
            },
            SynExprData::Bracketed { item, .. } => self.infer_new_expr_term(item),
            SynExprData::Block { stmts: _ } => todo!(),
            SynExprData::IndexOrCompositionWithList { .. } => todo!(),
            SynExprData::Err(ref e) => Err(DerivedDeclarativeTermError2::ExprError.into()),
            SynExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => match place_label_regional_token {
                Some(_) => todo!(),
                None => match self.symbol_declarative_term_region.self_place() {
                    Some(place) => Ok(DeclarativeTermExplicitApplication::new(
                        self.db,
                        self.declarative_term_menu.at_ty_path(),
                        place.into(),
                    )
                    .into()),
                    None => todo!(),
                },
            },
            SynExprData::Unit { .. } => Ok(self.declarative_term_menu.unit()),
            SynExprData::EmptyHtmlTag {
                empty_html_bra_idx: langle_token_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            SynExprData::FunctionCall { .. } => todo!(),
            SynExprData::Ritchie {
                ritchie_kind,
                ref parameter_ty_items,
                return_ty_syn_expr_idx: return_ty_expr,
                ..
            } => {
                let parameter_tys: SmallVec<[_; 2]> = parameter_ty_items
                    .into_iter()
                    .map(|parameter_ty_item| {
                        // todo: support variadic, and keyed??
                        Ok(DeclarativeRitchieRegularParameter::new(
                            // todo: handle &mut !!
                            Contract::None,
                            self.infer_new_expr_term(parameter_ty_item.syn_expr_idx())?,
                        )
                        .into())
                    })
                    .collect::<DeclarativeTermResult2<SmallVec<_>>>()?;
                let return_ty = match return_ty_expr {
                    Some(return_ty_expr) => self.infer_new_expr_term(return_ty_expr)?,
                    None => self.declarative_term_menu.unit(),
                };
                Ok(
                    DeclarativeTermRitchie::new(self.db, ritchie_kind, parameter_tys, return_ty)
                        .into(),
                )
            }
            SynExprData::Sorry {
                regional_token_idx: token_idx,
            } => todo!(),
            SynExprData::Todo {
                regional_token_idx: token_idx,
            } => todo!(),
            SynExprData::Unreachable { regional_token_idx } => todo!(),
        }
    }

    pub(crate) fn current_symbol_term(
        &self,
        symbol: SynCurrentSymbolIdx,
    ) -> Option<SymbolSignature> {
        self.symbol_declarative_term_region
            .current_symbol_signature(symbol)
    }

    pub(crate) fn declarative_term_menu(&self) -> &DeclarativeTermMenu {
        self.declarative_term_menu
    }
}
