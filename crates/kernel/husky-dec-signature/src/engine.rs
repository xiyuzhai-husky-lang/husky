mod pattern_ty;

pub(crate) use self::pattern_ty::*;

use crate::*;
use husky_print_utils::p;
use husky_syn_expr::*;
use husky_syn_opr::{SynBinaryOpr, SynPrefixOpr};
use husky_vfs::Toolchain;
use salsa::DebugWithDb;

pub(super) struct DecTermEngine<'a> {
    db: &'a ::salsa::Db,
    toolchain: Toolchain,
    syn_expr_region_data: &'a SynExprRegionData,
    dec_term_menu: &'a DecTermMenu,
    symbol_declarative_term_region: DecSvarRegion,
    expr_terms: SynExprMap<DecTermResult2<DecTerm>>,
    /// todo: change this to ordered
    pattern_expr_ty_infos: SynPatternExprMap<PatternExprDeclarativeTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolDeclarativeTypeInfo>,
}

#[salsa::tracked(jar = DecSignatureJar, return_ref)]
pub fn syn_expr_dec_term_region(
    db: &::salsa::Db,
    syn_expr_region: SynExprRegion,
) -> SynExprDecTermRegion {
    let expr_region_data = syn_expr_region.data(db);
    let parent_expr_region = expr_region_data.parent();
    let parent_term_symbol_region =
        parent_expr_region.map(|r| syn_expr_dec_term_region(db, r).dec_symbol_region());
    let engine = DecTermEngine::new(db, syn_expr_region, parent_term_symbol_region);
    engine.infer_all()
}

impl<'a> DecTermEngine<'a> {
    fn new(
        db: &'a ::salsa::Db,
        syn_expr_region: SynExprRegion,
        parent_term_symbol_region: Option<&'a DecSvarRegion>,
    ) -> Self {
        let toolchain = syn_expr_region.toolchain(db);
        // ad hoc
        let _item_path_menu = item_path_menu(db, toolchain);
        let dec_term_menu = db.dec_term_menu(toolchain).unwrap();
        let syn_expr_region_data = &syn_expr_region.data(db);
        Self {
            db,
            toolchain,
            syn_expr_region_data,
            dec_term_menu,
            symbol_declarative_term_region: DecSvarRegion::new(
                parent_term_symbol_region,
                syn_expr_region_data,
                dec_term_menu,
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

    fn infer_all(mut self) -> SynExprDecTermRegion {
        self.infer_current_syn_symbol_terms();
        self.symbol_declarative_term_region
            .infer_self_ty_parameter_and_self_value_parameter(
                self.db,
                self.toolchain,
                self.syn_expr_region_data.path(),
                self.syn_expr_region_data.symbol_region(),
            );
        self.infer_expr_roots();
        self.finish()
    }

    fn infer_current_syn_symbol_terms(&mut self) {
        let mut current_syn_symbol_indexed_iter = self
            .syn_expr_region_data
            .symbol_region()
            .indexed_current_syn_symbols();
        for (pattern_ty_constraint, symbols) in self
            .syn_expr_region_data
            .symbol_region()
            .pattern_ty_constraints()
        {
            match pattern_ty_constraint {
                SyndicateTypeConstraint::TemplateTypeParameter => {
                    let (current_syn_symbol_idx, current_syn_symbol) =
                        current_syn_symbol_indexed_iter
                            .next()
                            .expect("ty constraint should match with current symbols");
                    let CurrentSynSymbolData::TemplateParameter {
                        syn_attrs,
                        annotated_variance_token,
                        template_parameter_variant,
                    } = current_syn_symbol.data()
                    else {
                        unreachable!()
                    };
                    let attrs = DeclarativeTemplateSymbolAttrs::from_attrs(syn_attrs.iter().map(
                        |syn_attr| match syn_attr {
                            TemplateSymbolSynAttr::Phantom(_, _) => {
                                DeclarativeTemplateSymbolAttr::Phantom
                            }
                            TemplateSymbolSynAttr::Runtime(_, _) => {
                                DeclarativeTemplateSymbolAttr::Runtime
                            }
                        },
                    ));
                    let variance = annotated_variance_token.map(|vt| vt.into());
                    let (name, (ty, term_symbol)) = match *template_parameter_variant {
                        CurrentTemplateParameterSynSymbolVariant::Lifetime { label_token } => (
                            label_token.label().into(),
                            DecSvar::new_lifetime(
                                self.db,
                                self.toolchain,
                                self.dec_term_menu,
                                &mut self.symbol_declarative_term_region.symbol_registry_mut(),
                                attrs,
                                variance,
                            ),
                        ),
                        CurrentTemplateParameterSynSymbolVariant::Place { label_token } => (
                            label_token.label().into(),
                            DecSvar::new_place(
                                self.db,
                                self.toolchain,
                                self.dec_term_menu,
                                &mut self.symbol_declarative_term_region.symbol_registry_mut(),
                                attrs,
                                variance,
                            ),
                        ),
                        CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. } => (
                            ident_token.ident().into(),
                            DecSvar::new_ty(
                                self.db,
                                self.toolchain,
                                self.dec_term_menu,
                                &mut self.symbol_declarative_term_region.symbol_registry_mut(),
                                attrs,
                                variance,
                            ),
                        ),
                        CurrentTemplateParameterSynSymbolVariant::Constant {
                            ident_token,
                            ty_expr_idx,
                        } => {
                            let ty = self.infer_new_expr_term(ty_expr_idx).map_err(Into::into);
                            (
                                ident_token.ident().into(),
                                (
                                    ty,
                                    DecSvar::new_const(
                                        self.db,
                                        self.toolchain,
                                        attrs,
                                        ty,
                                        &mut self
                                            .symbol_declarative_term_region
                                            .symbol_registry_mut(),
                                    ),
                                ),
                            )
                        }
                        _ => todo!(),
                    };
                    self.symbol_declarative_term_region
                        .add_new_template_parameter_symbol_signature(
                            self.db,
                            symbols.start(),
                            ty,
                            term_symbol,
                            name,
                        )
                }
                SyndicateTypeConstraint::OrdinaryParenateParameter {
                    syn_pattern_root,
                    ty_expr_idx: ty,
                } => self.init_current_syn_symbol_signatures_in_parenate_parameter(
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
                        .add_new_field_variable_symbol_signature(
                            self.db,
                            symbols.start(),
                            ty,
                            ident_token.ident().into(),
                        )
                }
                SyndicateTypeConstraint::LetPattern { .. }
                | SyndicateTypeConstraint::LoopVariable => {
                    // need only to compute for decl region
                    return;
                }
                SyndicateTypeConstraint::VariadicParenateParameter { ident_token, ty } => {
                    let ty = self.infer_new_expr_term(*ty).map_err(|_| todo!());
                    let symbol = DecSvar::new_ephem(
                        self.db,
                        self.toolchain,
                        ty,
                        &mut self.symbol_declarative_term_region.symbol_registry_mut(),
                    );
                    self.symbol_declarative_term_region
                        .add_new_template_parameter_symbol_signature(
                            self.db,
                            symbols.start(),
                            ty,
                            symbol,
                            ident_token.ident().into(),
                        )
                }
            }
        }
    }

    /// explicit parameters are infered in this crate;
    ///
    /// let variables, be variables and match variables are infered in `husky-expr-ty`
    fn init_current_syn_symbol_signatures_in_parenate_parameter(
        &mut self,
        parenate_syn_pattern_expr_root: ParenateSynPatternExprRoot,
        ty: SynExprIdx,
        symbols: CurrentSynSymbolIdxRange,
    ) {
        let Ok(ty) = self.infer_new_expr_term(ty) else {
            for symbol in symbols {
                let current_syn_symbol = &self.syn_expr_region_data[symbol];
                let name = current_syn_symbol.name();
                let modifier = current_syn_symbol.modifier();
                self.symbol_declarative_term_region
                    .add_new_parenate_parameter_symbol_signature(
                        self.db,
                        symbol,
                        modifier,
                        Err(DecTermSymbolTypeErrorKind::CannotInferTypeExprTerm(
                            self.syn_expr_region_data.path(),
                        )),
                        name,
                    )
            }
            return;
        };
        self.infer_pattern_tys_in_parenate_parameter(parenate_syn_pattern_expr_root, ty);
        for symbol in symbols {
            self.infer_current_syn_symbol_signature_in_parenate_parameter(symbol)
        }
    }

    fn infer_current_syn_symbol_signature_in_parenate_parameter(
        &mut self,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
    ) {
        let current_syn_symbol = &self.syn_expr_region_data.symbol_region()[current_syn_symbol_idx];
        match *current_syn_symbol.data() {
            CurrentSynSymbolData::ParenateRegularParameter {
                ident,
                pattern_symbol_idx,
            } => {
                let base_ty = self.pattern_symbol_ty_infos[pattern_symbol_idx].base_ty();
                self.symbol_declarative_term_region
                    .add_new_parenate_parameter_symbol_signature(
                        self.db,
                        current_syn_symbol_idx,
                        current_syn_symbol.modifier(),
                        Ok(base_ty),
                        ident.into(),
                    )
            }
            _ => unreachable!("this function is only used for explicit parameters"),
        }
    }

    fn infer_expr_roots(&mut self) {
        for expr_root in self.syn_expr_region_data.syn_expr_roots() {
            match expr_root.kind() {
                // omit props struct field because they are inferred for field variable
                SynExprRootKind::PropsStructFieldType { .. } => continue,
                SynExprRootKind::Trait
                | SynExprRootKind::ReturnType
                | SynExprRootKind::TupleStructFieldType
                | SynExprRootKind::ExplicitParameterDefaultValue { .. }
                | SynExprRootKind::TypeAliasTypeTerm
                | SynExprRootKind::AssocTypeTerm => (),
                SynExprRootKind::SelfType => {
                    let self_ty_term = self.infer_new_expr_term(expr_root.syn_expr_idx()).ok();
                    self.symbol_declarative_term_region
                        .set_self_ty(self_ty_term);
                    continue;
                }
                SynExprRootKind::BlockExpr
                | SynExprRootKind::LetStmtType
                | SynExprRootKind::LetStmtInitialValue
                | SynExprRootKind::HtmlArgumentExpr
                | SynExprRootKind::ReturnExpr
                | SynExprRootKind::Condition
                | SynExprRootKind::ConstantImplicitParameterType
                | SynExprRootKind::ExplicitParameterType
                | SynExprRootKind::FieldBindInitialValue { .. }
                | SynExprRootKind::Snippet
                | SynExprRootKind::ValExpr
                | SynExprRootKind::EvalExpr => continue,
            }
            self.cache_new_expr_term(expr_root.syn_expr_idx())
        }
    }

    // infer the term for expr, assuming it hasn't been computed before
    fn infer_new_expr_term(&mut self, expr_idx: SynExprIdx) -> DecTermResult2<DecTerm> {
        let result = self.calc_expr_term(expr_idx);
        let result_export = match result {
            Ok(term) => Ok(term),
            Err(_) => Err(DerivedDecTermError2::DecTermAbortion.into()),
        };
        self.save_expr_term(expr_idx, result);
        result_export
    }

    // cache the term for expr, assuming it hasn't been computed before
    fn cache_new_expr_term(&mut self, expr_idx: SynExprIdx) {
        let result = self.calc_expr_term(expr_idx);
        self.save_expr_term(expr_idx, result)
    }

    pub(crate) fn finish(self) -> SynExprDecTermRegion {
        SynExprDecTermRegion::new(
            self.syn_expr_region_data.path(),
            self.symbol_declarative_term_region,
            self.expr_terms,
            self.pattern_expr_ty_infos,
            self.pattern_symbol_ty_infos,
        )
    }

    #[track_caller]
    fn save_expr_term(&mut self, expr_idx: SynExprIdx, outcome: DecTermResult2<DecTerm>) {
        self.expr_terms.insert_new(expr_idx, outcome)
    }

    fn calc_expr_term(&mut self, expr_idx: SynExprIdx) -> DecTermResult2<DecTerm> {
        let db = self.db;
        match self.syn_expr_region_data.expr_arena()[expr_idx] {
            SynExprData::Literal(token_idx, literal) => {
                Ok(DecTerm::from_literal_token_data(literal, db))
            }
            SynExprData::PrincipalEntityPath { opt_path, .. } => match opt_path {
                Some(path) => Ok(DecTerm::EntityPath(match path {
                    PrincipalEntityPath::Module(_) => todo!(),
                    PrincipalEntityPath::MajorItem(path) => match path {
                        MajorItemPath::Type(path) => DecItemPath::Type(path),
                        MajorItemPath::Trait(path) => path.into(),
                        MajorItemPath::Fugitive(path) => path.into(),
                    },
                    PrincipalEntityPath::TypeVariant(path) => DecItemPath::TypeVariant(path),
                })),
                None => Err(DerivedDecTermError2::InvalidEntityPath.into()),
            },
            SynExprData::AssocItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
            } => todo!(),
            SynExprData::InheritedSynSymbol {
                inherited_syn_symbol_idx,
                ..
            } => self
                .symbol_declarative_term_region
                .inherited_syn_symbol_signature(inherited_syn_symbol_idx)
                .term_symbol()
                .map(Into::into)
                .ok_or(DerivedDecTermError2::InheritedSynSymbolIsNotValidTerm.into()),
            SynExprData::CurrentSynSymbol {
                current_syn_symbol_idx,
                ..
            } => Ok(self
                .symbol_declarative_term_region
                .current_parameter_symbol_signature(current_syn_symbol_idx)
                .expect("not none")
                .term_symbol()
                .ok_or(OriginalDecTermError2::InvalidSymbolForTerm)?
                .into()),
            SynExprData::FrameVarDecl { .. } => unreachable!(),
            SynExprData::SelfType(_) => self
                .symbol_declarative_term_region
                .self_ty()
                .ok_or(DerivedDecTermError2::SelfTypeNotAllowedInThisRegion.into()),
            SynExprData::SelfValue(_) => self
                .symbol_declarative_term_region
                .self_ty()
                .ok_or(DerivedDecTermError2::SelfValueNotAllowedInThisRegion.into()),
            SynExprData::Binary {
                lopd, opr, ropd, ..
            } => {
                let Ok(lopd) = self.infer_new_expr_term(lopd) else {
                    return Err(DerivedDecTermError2::CannotInferOperandDecTermInPrefix.into());
                };
                let Ok(ropd) = self.infer_new_expr_term(ropd) else {
                    return Err(DerivedDecTermError2::CannotInferOperandDecTermInPrefix.into());
                };
                match opr {
                    SynBinaryOpr::Closed(_) => todo!(),
                    SynBinaryOpr::Shift(_) => todo!(),
                    SynBinaryOpr::Comparison(_) => todo!(),
                    SynBinaryOpr::ShortCircuitLogic(_) => todo!(),
                    SynBinaryOpr::AssignOrDefEq => todo!(),
                    SynBinaryOpr::AssignClosed(_) => todo!(),
                    SynBinaryOpr::AssignShift(_) => todo!(),
                    SynBinaryOpr::ScopeResolution => todo!(),
                    SynBinaryOpr::CurryType => Ok(DecCurry::new_nondependent(
                        self.db,
                        self.toolchain,
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
                    return Err(DerivedDecTermError2::CannotInferOperandDecTermInPrefix.into());
                };
                let tmpl = match opr {
                    SynPrefixOpr::Minus => todo!(),
                    SynPrefixOpr::Not => todo!(),
                    SynPrefixOpr::Tilde => {
                        DecTerm::LeashOrBitNot(self.syn_expr_region_data.path().toolchain(self.db))
                    }
                    SynPrefixOpr::Ref => todo!(),
                    SynPrefixOpr::Option => self.dec_term_menu.option_ty_path(),
                };
                Ok(DecApplication::new(self.db, tmpl, opd).into())
            }
            SynExprData::Suffix { .. } => todo!(),
            SynExprData::Field { .. } => todo!(),
            SynExprData::MethodApplicationOrCall { .. } => todo!(),
            SynExprData::TemplateInstantiation {
                template,
                ref template_arguments,
            } => {
                let db = self.db;
                p!(
                    template_arguments.langle_regional_token_idx(),
                    self.syn_expr_region_data.path().debug(db),
                    template_arguments.rangle_regional_token_idx()
                );
                todo!();
                // let mut template_term = self.infer_new_expr_term(template)?;
                // for arg in template_arguments.arguments() {
                //     template_term = DecTermExplicitApplication::new(
                //         self.db,
                //         template_term,
                //         self.infer_new_expr_term(arg.syn_expr_idx())?,
                //     )
                //     .into()
                // }
                // Ok(template_term)
            }
            SynExprData::FunctionApplicationOrCall {
                function,
                ref template_arguments,
                ref items,
                ..
            } => {
                let Ok(function) = self.infer_new_expr_term(function) else {
                    return Err(
                        DerivedDecTermError2::CannotInferArgumentDecTermInApplication.into(),
                    );
                };
                let template_arguments = match template_arguments {
                    Some(template_arguments) => template_arguments
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
                    .collect::<DecTermResult2<_>>()?;
                Ok(DecApplicationOrRitchieCall::new(
                    function,
                    template_arguments,
                    items,
                    extra_comma,
                    self.db,
                )
                .into())
            }
            SynExprData::ExplicitApplication {
                function_expr_idx: function,
                argument_expr_idx: argument,
            } => {
                let Ok(argument) = self.infer_new_expr_term(argument) else {
                    Err(DerivedDecTermError2::CannotInferArgumentDecTermInApplication)?
                };
                let Ok(function) = self.infer_new_expr_term(function) else {
                    Err(DerivedDecTermError2::CannotInferFunctionDecTermInApplication)?
                };
                Ok(DecApplication::new(self.db, function, argument).into())
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
                    .collect::<DecTermResult2<Vec<_>>>()?;
                Ok(DecList::new(
                    self.db,
                    self.syn_expr_region_data.path().toolchain(self.db),
                    items,
                )
                .into())
            }
            SynExprData::BoxColonList { ref items, .. } => match items.len() {
                0 => Ok(self.dec_term_menu.slice_ty_path()),
                _ => todo!(),
            },
            SynExprData::Bracketed { item, .. } => self.infer_new_expr_term(item),
            SynExprData::Block { stmts: _ } => todo!(),
            SynExprData::IndexOrCompositionWithList { .. } => todo!(),
            SynExprData::Err(ref e) => Err(DerivedDecTermError2::ExprError.into()),
            SynExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => match place_label_regional_token {
                Some(_) => todo!(),
                None => match self.symbol_declarative_term_region.self_place() {
                    Some(place) => Ok(DecApplication::new(
                        self.db,
                        self.dec_term_menu.at_ty_path(),
                        place.into(),
                    )
                    .into()),
                    None => todo!(),
                },
            },
            SynExprData::Unit { .. } => Ok(self.dec_term_menu.unit()),
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
                            TermContract::Pure,
                            self.infer_new_expr_term(parameter_ty_item.syn_expr_idx())?,
                        )
                        .into())
                    })
                    .collect::<DecTermResult2<SmallVec<_>>>()?;
                let return_ty = match return_ty_expr {
                    Some(return_ty_expr) => self.infer_new_expr_term(return_ty_expr)?,
                    None => self.dec_term_menu.unit(),
                };
                Ok(DecRitchie::new(self.db, ritchie_kind, parameter_tys, return_ty).into())
            }
            SynExprData::Sorry { .. } => todo!(),
            SynExprData::Todo { .. } => todo!(),
            SynExprData::Unreachable { .. } => todo!(),
        }
    }
}
