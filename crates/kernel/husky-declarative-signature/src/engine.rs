mod pattern_ty;

pub(crate) use self::pattern_ty::*;

use crate::*;
use husky_opr::{BinaryOpr, PrefixOpr};
use husky_print_utils::p;
use husky_syn_expr::*;
use husky_token::{IntegerLikeLiteral, Literal, RangedTokenSheet, TokenSheetData};
use salsa::DebugWithDb;

pub(super) struct DeclarativeTermEngine<'a> {
    db: &'a dyn DeclarativeSignatureDb,
    expr_region_data: &'a SynExprRegionData,
    declarative_term_menu: &'a DeclarativeTermMenu,
    symbol_declarative_term_region: SymbolDeclarativeTermRegion,
    expr_terms: SynExprMap<DeclarativeTermResult2<DeclarativeTerm>>,
    /// todo: change this to ordered
    pattern_expr_ty_infos: SynPatternExprMap<PatternExprDeclarativeTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
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
        let expr_region_data = &syn_expr_region.data(db);
        Self {
            db,
            expr_region_data,
            declarative_term_menu,
            symbol_declarative_term_region: SymbolDeclarativeTermRegion::new(
                parent_term_symbol_region,
                expr_region_data.symbol_region(),
            ),
            expr_terms: SynExprMap::new(expr_region_data.expr_arena()),
            pattern_expr_ty_infos: SynPatternExprMap::new(expr_region_data.pattern_expr_arena()),
            pattern_symbol_ty_infos: SynPatternSymbolMap::new(
                expr_region_data
                    .pattern_expr_region()
                    .pattern_symbol_arena(),
            ),
        }
    }

    fn infer_all(mut self) -> DeclarativeTermRegion {
        self.init_current_symbol_terms();
        self.symbol_declarative_term_region
            .init_self_ty_parameter_and_self_value_parameter(
                self.db,
                self.expr_region_data.path(),
                self.expr_region_data.symbol_region(),
            );
        self.init_expr_roots();
        self.finish()
    }

    fn init_current_symbol_terms(&mut self) {
        let mut current_symbol_indexed_iter = self
            .expr_region_data
            .symbol_region()
            .current_symbol_indexed_iter();
        for (pattern_ty_constraint, symbols) in self
            .expr_region_data
            .symbol_region()
            .pattern_ty_constraints()
        {
            match pattern_ty_constraint {
                PatternTypeConstraint::TemplateTypeParameter => {
                    let (current_symbol_idx, current_symbol) = current_symbol_indexed_iter
                        .next()
                        .expect("ty constraint should match with current symbols");
                    let CurrentSynSymbolVariant::TemplateParameter {
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
                PatternTypeConstraint::ExplicitRegularParameter {
                    pattern_expr_idx: pattern_expr,
                    ty_expr_idx: ty,
                } => self.init_current_symbol_signatures_in_parenate_parameter(
                    *pattern_expr,
                    *ty,
                    *symbols,
                ),
                PatternTypeConstraint::LetVariables { .. }
                | PatternTypeConstraint::FrameVariable => {
                    // need only to compute for decl region
                    return;
                }
                PatternTypeConstraint::ExplicitVariadicParameter { ty } => {
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
        pattern_expr: SynPatternExprIdx,
        ty: SynExprIdx,
        symbols: CurrentSynSymbolIdxRange,
    ) {
        let Ok(ty) = self.infer_new_expr_term(ty) else {
            for symbol in symbols {
                let modifier = self.expr_region_data[symbol].modifier();
                self.symbol_declarative_term_region
                    .add_new_parenate_parameter_symbol_signature(
                        self.db,
                        symbol,
                        modifier,
                        Err(DeclarativeTermSymbolTypeErrorKind::SignatureDeclarativeTermError),
                    )
            }
            return;
        };
        self.infer_pattern_tys_in_parenate_parameter(pattern_expr, ty);
        for symbol in symbols {
            self.infer_current_symbol_signature_in_parenate_parameter(symbol)
        }
    }

    fn infer_current_symbol_signature_in_parenate_parameter(
        &mut self,
        current_symbol_idx: CurrentSynSymbolIdx,
    ) {
        let current_symbol = &self.expr_region_data.symbol_region()[current_symbol_idx];
        match current_symbol.variant() {
            CurrentSynSymbolVariant::ParenateRegularParameter {
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

    fn init_expr_roots(&mut self) {
        for expr_root in self.expr_region_data.roots() {
            match expr_root.kind() {
                ExprRootKind::PropsStructFieldType { .. }
                | ExprRootKind::Trait
                | ExprRootKind::ReturnType
                | ExprRootKind::TupleStructFieldType
                | ExprRootKind::ReturnType
                | ExprRootKind::ExplicitParameterDefaultValue { .. }
                | ExprRootKind::AssociatedTypeTerm => (),
                ExprRootKind::SelfType => {
                    let self_ty_term = self.infer_new_expr_term(expr_root.expr_idx()).ok();
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
            self.cache_new_expr_term(expr_root.expr_idx())
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
            self.expr_region_data.path(),
            self.symbol_declarative_term_region,
            self.expr_terms,
            self.pattern_expr_ty_infos,
            self.pattern_symbol_ty_infos,
        )
    }

    fn save_expr_term(
        &mut self,
        expr_idx: SynExprIdx,
        outcome: DeclarativeTermResult2<DeclarativeTerm>,
    ) {
        self.expr_terms.insert_new(expr_idx, outcome)
    }

    fn calc_expr_term(&mut self, expr_idx: SynExprIdx) -> DeclarativeTermResult2<DeclarativeTerm> {
        match self.expr_region_data.expr_arena()[expr_idx] {
            SynExpr::Literal(token_idx, literal) => match literal {
                Literal::Unit => todo!(),
                Literal::Char(_) => todo!(),
                Literal::String(_) => todo!(),
                Literal::Integer(literal) => match literal {
                    IntegerLikeLiteral::UnspecifiedRegular(i) => Ok(DeclarativeTerm::Literal(
                        UnresolvedTermLiteral::RegularInteger(i).into(),
                    )),
                    IntegerLikeLiteral::UnspecifiedLarge() => todo!(),
                    IntegerLikeLiteral::I8(_) => todo!(),
                    IntegerLikeLiteral::I16(_) => todo!(),
                    IntegerLikeLiteral::I32(_) => todo!(),
                    IntegerLikeLiteral::I64(_) => todo!(),
                    IntegerLikeLiteral::I128(_) => todo!(),
                    IntegerLikeLiteral::ISize(_) => todo!(),
                    IntegerLikeLiteral::R8(_) => todo!(),
                    IntegerLikeLiteral::R16(_) => todo!(),
                    IntegerLikeLiteral::R32(_) => todo!(),
                    IntegerLikeLiteral::R64(_) => todo!(),
                    IntegerLikeLiteral::R128(_) => todo!(),
                    IntegerLikeLiteral::RSize(_) => todo!(),
                    IntegerLikeLiteral::U8(_) => todo!(),
                    IntegerLikeLiteral::U16(_) => todo!(),
                    IntegerLikeLiteral::U32(_) => todo!(),
                    IntegerLikeLiteral::U64(_) => todo!(),
                    IntegerLikeLiteral::U128(_) => todo!(),
                    IntegerLikeLiteral::USize(_) => todo!(),
                },
                Literal::Float(_) => todo!(),
                Literal::TupleIndex(_) => todo!(),
                Literal::Bool(_) => todo!(),
            },
            SynExpr::PrincipalEntityPath {
                item_path_expr: _,
                opt_path,
            } => match opt_path {
                Some(path) => Ok(DeclarativeTerm::EntityPath(match path {
                    PrincipalEntityPath::Module(_) => todo!(),
                    PrincipalEntityPath::ModuleItem(path) => match path {
                        ModuleItemPath::Type(path) => DeclarativeTermEntityPath::Type(path),
                        ModuleItemPath::Trait(path) => path.into(),
                        ModuleItemPath::Fugitive(path) => path.into(),
                    },
                    PrincipalEntityPath::TypeVariant(path) => {
                        DeclarativeTermEntityPath::TypeVariant(path)
                    }
                })),
                None => Err(DerivedDeclarativeTermError2::InvalidEntityPath.into()),
            },
            SynExpr::ScopeResolution {
                parent_expr_idx,
                scope_resolution_token,
                ident_token,
            } => todo!(),
            SynExpr::InheritedSymbol {
                inherited_symbol_idx,
                ..
            } => self
                .symbol_declarative_term_region
                .inherited_symbol_signature(inherited_symbol_idx)
                .term_symbol()
                .map(Into::into)
                .ok_or(DerivedDeclarativeTermError2::InheritedSymbolIsNotValidTerm.into()),
            SynExpr::CurrentSymbol {
                current_symbol_idx, ..
            } => Ok(self
                .symbol_declarative_term_region
                .current_symbol_signature(current_symbol_idx)
                .expect("not none")
                .term_symbol()
                .ok_or(OriginalDeclarativeTermError2::InvalidSymbolForTerm)?
                .into()),
            SynExpr::FrameVarDecl { .. } => unreachable!(),
            SynExpr::SelfType(_) => self
                .symbol_declarative_term_region
                .self_ty_term()
                .ok_or(DerivedDeclarativeTermError2::SelfTypeNotAllowedInThisRegion.into()),
            SynExpr::SelfValue(_) => self
                .symbol_declarative_term_region
                .self_ty_term()
                .ok_or(DerivedDeclarativeTermError2::SelfValueNotAllowedInThisRegion.into()),
            SynExpr::Binary {
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
                    BinaryOpr::Closed(_) => todo!(),
                    BinaryOpr::Shift(_) => todo!(),
                    BinaryOpr::Comparison(_) => todo!(),
                    BinaryOpr::ShortCircuitLogic(_) => todo!(),
                    BinaryOpr::Assign => todo!(),
                    BinaryOpr::AssignClosed(_) => todo!(),
                    BinaryOpr::AssignShift(_) => todo!(),
                    BinaryOpr::ScopeResolution => todo!(),
                    BinaryOpr::Curry => Ok(DeclarativeTermCurry::new_nondependent(
                        self.db,
                        CurryKind::Explicit, // ad hoc
                        Variance::Invariant, // ad hoc
                        lopd,
                        ropd,
                    )
                    .into()),
                    BinaryOpr::As => todo!(),
                    BinaryOpr::Ins => todo!(),
                    BinaryOpr::In => todo!(),
                }
            }
            SynExpr::Be { .. } => todo!(),
            SynExpr::Prefix {
                opr,
                opr_token_idx: _,
                opd,
            } => {
                let Ok(opd) = self.infer_new_expr_term(opd) else {
                    return Err(
                        DerivedDeclarativeTermError2::CannotInferOperandDeclarativeTermInPrefix
                            .into(),
                    );
                };
                let tmpl = match opr {
                    PrefixOpr::Minus => todo!(),
                    PrefixOpr::Not => todo!(),
                    PrefixOpr::Tilde => DeclarativeTerm::LeashOrBitNot(
                        self.expr_region_data.path().toolchain(self.db),
                    ),
                    PrefixOpr::Ref => todo!(),
                    // self.declarative_term_menu.ref_ty_path(),
                    PrefixOpr::Vector => todo!(),
                    PrefixOpr::Slice => todo!(),
                    PrefixOpr::CyclicSlice => todo!(),
                    PrefixOpr::Array(_) => todo!(),
                    PrefixOpr::Option => self.declarative_term_menu.option_ty_path(),
                };
                Ok(DeclarativeTermExplicitApplication::new(self.db, tmpl, opd).into())
            }
            SynExpr::Suffix {
                opd: _,
                opr: _punctuation,
                opr_token_idx: _punctuation_token_idx,
            } => todo!(),
            SynExpr::Field {
                owner: _self_expr,
                dot_token_idx: _,
                ident_token: _,
            } => todo!(),
            SynExpr::MethodApplicationOrCall { .. } => todo!(),
            SynExpr::TemplateInstantiation { .. } => todo!(),
            SynExpr::FunctionApplicationOrCall {
                function,
                ref generic_arguments,
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
                    Some(last_item) => last_item.comma_token_idx().is_some(),
                    None => false,
                };
                let items = items
                    .into_iter()
                    .map(|item| self.infer_new_expr_term(item.expr_idx()))
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
            SynExpr::ExplicitApplication {
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
            SynExpr::NewTuple { ref items, .. } => {
                p!(self.expr_region_data.path().debug(self.db));
                p!(items.len());
                todo!()
            }
            SynExpr::List { ref items, .. } => {
                let items = items
                    .iter()
                    .map(|item| self.infer_new_expr_term(item.expr_idx()))
                    .collect::<DeclarativeTermResult2<Vec<_>>>()?;
                Ok(DeclarativeTermList::new(
                    self.db,
                    self.expr_region_data.path().toolchain(self.db),
                    items,
                )
                .into())
            }
            SynExpr::BoxColonList { ref items, .. } => match items.len() {
                0 => Ok(self.declarative_term_menu.slice_ty_path()),
                _ => todo!(),
            },
            SynExpr::Bracketed { item, .. } => self.infer_new_expr_term(item),
            SynExpr::Block { stmts: _ } => todo!(),
            SynExpr::IndexOrCompositionWithList {
                owner: _,
                lbox_token_idx: _,
                ref items,
                rbox_token_idx: _,
            } => todo!(),
            SynExpr::Err(_) => Err(DerivedDeclarativeTermError2::ExprError.into()),
            SynExpr::Unit {
                lpar_token_idx,
                rpar_token_idx,
            } => Ok(self.declarative_term_menu.unit()),
            SynExpr::EmptyHtmlTag {
                empty_html_bra_idx: langle_token_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            SynExpr::FunctionCall { .. } => todo!(),
            SynExpr::Ritchie {
                ritchie_kind,
                ref parameter_ty_items,
                return_ty_expr,
                ..
            } => {
                let parameter_tys: SmallVec<[_; 2]> = parameter_ty_items
                    .into_iter()
                    .map(|parameter_ty_item| {
                        // todo: support variadic, and keyed??
                        Ok(DeclarativeTermRitchieRegularParameter::new(
                            // todo: handle &mut !!
                            Contract::None,
                            self.infer_new_expr_term(parameter_ty_item.expr_idx())?,
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
        }
    }

    pub(crate) fn current_symbol_term(
        &self,
        symbol: CurrentSynSymbolIdx,
    ) -> Option<SymbolSignature> {
        self.symbol_declarative_term_region
            .current_symbol_signature(symbol)
    }

    pub(crate) fn declarative_term_menu(&self) -> &DeclarativeTermMenu {
        self.declarative_term_menu
    }
}
