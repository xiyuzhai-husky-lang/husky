use super::*;
use husky_expr::*;
use husky_opn_syntax::{BinaryOpr, PrefixOpr};
use husky_print_utils::p;
use salsa::DebugWithDb;

pub(super) struct SignatureRawTermEngine<'a> {
    db: &'a dyn SignatureDb,
    expr_region_data: &'a ExprRegionData,
    raw_term_menu: &'a RawTermMenu,
    expr_terms: ExprMap<SignatureRawTermResult<RawTerm>>,
    term_symbol_region: RawTermSymbolRegion,
}

impl<'a> SignatureRawTermEngine<'a> {
    pub(super) fn new(
        db: &'a dyn SignatureDb,
        expr_region: ExprRegion,
        parent_term_symbol_region: Option<&'a RawTermSymbolRegion>,
    ) -> Self {
        let toolchain = expr_region.toolchain(db);
        // ad hoc
        let _entity_path_menu = db.entity_path_menu(toolchain).unwrap();
        let raw_term_menu = db.raw_term_menu(toolchain).unwrap();
        let expr_region_data = &expr_region.data(db);
        let mut this = Self {
            db,
            expr_region_data,
            raw_term_menu,
            expr_terms: ExprMap::new(expr_region_data.expr_arena()),
            term_symbol_region: RawTermSymbolRegion::new(
                parent_term_symbol_region,
                expr_region_data.symbol_region(),
            ),
        };
        this.init_current_symbol_term_symbols();
        this.init_expr_roots();
        this
    }

    fn init_current_symbol_term_symbols(&mut self) {
        for (_idx, symbol) in self
            .expr_region_data
            .symbol_region()
            .indexed_current_symbol_iter()
        {
            let ty = match symbol.variant() {
                CurrentSymbolVariant::ImplicitParameter {
                    implicit_parameter_variant,
                } => match implicit_parameter_variant {
                    CurrentImplicitParameterSymbol::Type { .. } => {
                        Ok(self.raw_term_menu.ty0().into())
                    }
                    CurrentImplicitParameterSymbol::Lifetime { .. } => {
                        Ok(self.raw_term_menu.lifetime_ty().into())
                    }
                    _ => todo!(),
                },
                CurrentSymbolVariant::RegularParameter {
                    pattern_symbol_idx, ..
                } => {
                    let pattern_symbol =
                        &self.expr_region_data.pattern_expr_region()[*pattern_symbol_idx];
                    match pattern_symbol {
                        PatternSymbol::Atom(pattern) => {
                            let ty = self
                                .expr_region_data
                                .symbol_region()
                                .regular_parameter_pattern_ty_constraint(*pattern)
                                .unwrap();
                            match self.infer_new(ty) {
                                Ok(ty) => Ok(ty),
                                Err(_) => Err(RawTermSymbolTypeErrorKind::SignatureRawTermError),
                            }
                        }
                    }
                }
                CurrentSymbolVariant::LetVariable { .. }
                | CurrentSymbolVariant::FrameVariable { .. } => return,
            };
            self.term_symbol_region.new_symbol(self.db, ty)
        }
    }

    fn init_expr_roots(&mut self) {
        for expr_root in self.expr_region_data.roots() {
            match expr_root.kind() {
                ExprRootKind::BlockExpr => return,
                ExprRootKind::SelfType
                | ExprRootKind::Trait
                | ExprRootKind::ReturnType
                | ExprRootKind::FieldType => (),
            }
            self.cache_new(expr_root.expr())
        }
    }

    // infer the term for expr, assuming it hasn't been computed before
    fn infer_new(&mut self, expr_idx: ExprIdx) -> SignatureRawTermResult<RawTerm> {
        let result = self.calc(expr_idx);
        let result_export = match result {
            Ok(term) => Ok(term),
            Err(_) => Err(DerivedSignatureRawTermError::RawTermAbortion.into()),
        };
        self.save(expr_idx, result);
        result_export
    }

    // cache the term for expr, assuming it hasn't been computed before
    fn cache_new(&mut self, expr_idx: ExprIdx) {
        let result = self.calc(expr_idx);
        self.save(expr_idx, result)
    }

    pub(crate) fn finish(self) -> SignatureTermRegion {
        SignatureTermRegion::new(
            self.expr_region_data.path(),
            self.term_symbol_region,
            self.expr_terms,
        )
    }

    fn save(&mut self, expr_idx: ExprIdx, outcome: SignatureRawTermResult<RawTerm>) {
        self.expr_terms.insert_new(expr_idx, outcome)
    }

    fn calc(&mut self, expr_idx: ExprIdx) -> SignatureRawTermResult<RawTerm> {
        match self.expr_region_data.expr_arena()[expr_idx] {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath {
                entity_path_expr: _,
                path: entity_path,
            } => match entity_path {
                Some(entity_path) => Ok(RawTerm::EntityPath(match entity_path {
                    EntityPath::Module(_) => todo!(),
                    EntityPath::ModuleItem(path) => match path {
                        ModuleItemPath::Type(path) => {
                            /* ad hoc */
                            RawTermEntityPath::Type(path)
                        }
                        ModuleItemPath::Trait(path) => path.into(),
                        ModuleItemPath::Form(path) => path.into(),
                    },
                    EntityPath::AssociatedItem(_) => todo!(),
                    EntityPath::Variant(_) => todo!(),
                })),
                None => Err(DerivedSignatureRawTermError::InvalidEntityPath.into()),
            },
            Expr::InheritedSymbol {
                ident: _,
                token_idx: _,
                inherited_symbol_idx: _,
                inherited_symbol_kind: _,
            } => todo!(),
            Expr::CurrentSymbol {
                current_symbol_idx, ..
            } => Ok(self
                .term_symbol_region
                .current_symbol_term(current_symbol_idx)
                .expect("not none")
                .into()),
            Expr::FrameVarDecl { .. } => unreachable!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => todo!(),
            Expr::Binary {
                lopd, opr, ropd, ..
            } => {
                let  Ok(lopd) = self.infer_new(lopd) else {
                    return  Err(DerivedSignatureRawTermError::CannotInferOperandRawTermInPrefix.into());
                };
                let  Ok(ropd) = self.infer_new(ropd) else {
                    return  Err(DerivedSignatureRawTermError::CannotInferOperandRawTermInPrefix.into());
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
                    BinaryOpr::Curry => Ok(RawTermCurry::new(
                        self.db,
                        CurryKind::Explicit, // ad hoc
                        Variance::Invariant, // ad hoc
                        None,                // ad hoc
                        lopd,
                        ropd,
                    )
                    .into()),
                    BinaryOpr::As => todo!(),
                    BinaryOpr::Ins => todo!(),
                    BinaryOpr::In => todo!(),
                }
            }
            Expr::Be { .. } => todo!(),
            Expr::Prefix {
                opr,
                opr_token_idx: _,
                opd,
            } => {
                let Ok(opd) = self.infer_new(opd) else {
                    return  Err(DerivedSignatureRawTermError::CannotInferOperandRawTermInPrefix.into());
                };
                let tmpl = match opr {
                    PrefixOpr::Minus => todo!(),
                    PrefixOpr::Not => todo!(),
                    PrefixOpr::Tilde => {
                        RawTerm::LeashOrBitNot(self.expr_region_data.path().toolchain(self.db))
                    }
                    PrefixOpr::Ref => self.raw_term_menu.ref_ty_path(),
                    PrefixOpr::Vector => todo!(),
                    PrefixOpr::Slice => todo!(),
                    PrefixOpr::CyclicSlice => todo!(),
                    PrefixOpr::Array(_) => todo!(),
                    PrefixOpr::Option => self.raw_term_menu.option_ty_path(),
                };
                Ok(RawTermExplicitApplication::new(self.db, tmpl, opd).into())
            }
            Expr::Suffix {
                opd: _,
                opr: _punctuation,
                opr_token_idx: _punctuation_token_idx,
            } => todo!(),
            Expr::Field {
                owner: _self_expr,
                dot_token_idx: _,
                ident_token: _,
            } => todo!(),
            Expr::MethodCall { .. } => todo!(),
            Expr::TemplateInstantiation { .. } => todo!(),
            Expr::ExplicitApplicationOrRitchieCall {
                function,
                ref implicit_arguments,
                items,
                ref commas,
                ..
            } => {
                let Ok(function) = self.infer_new(function) else {
                    return  Err(
                        DerivedSignatureRawTermError::CannotInferArgumentRawTermInApplication.into()
                    )
                };
                let implicit_arguments = match implicit_arguments {
                    Some(implicit_arguments) => implicit_arguments
                        .arguments()
                        .into_iter()
                        .map(|_| todo!())
                        .collect(),
                    None => vec![],
                };
                assert!(items.len() <= commas.len() + 1);
                assert!(items.len() >= commas.len());
                let extra_comma = items.len() == commas.len();
                let items = items
                    .into_iter()
                    .map(|item| self.infer_new(item))
                    .collect::<SignatureRawTermResult<_>>()?;
                Ok(RawTermExplicitApplicationOrRitchieCall::new(
                    self.db,
                    function,
                    implicit_arguments,
                    items,
                    extra_comma,
                )
                .into())
            }
            Expr::ExplicitApplication { function, argument } => {
                let Ok(argument) = self.infer_new(argument) else {
                    Err(DerivedSignatureRawTermError::CannotInferArgumentRawTermInApplication)?
                };
                match self.expr_region_data.expr_arena()[function] {
                    Expr::BoxColonList { items, .. } => match items.len() {
                        0 => Ok(RawTermExplicitApplication::new(
                            self.db,
                            self.raw_term_menu.slice_ty_path(),
                            argument,
                        )
                        .into()),
                        _ => todo!(),
                    },
                    Expr::List { items, .. } => match items.len() {
                        0 => Ok(RawTermExplicitApplication::new(
                            self.db,
                            self.raw_term_menu.list(),
                            argument,
                        )
                        .into()),
                        1 => match self.expr_region_data.expr_arena()[items.start()] {
                            Expr::Literal(_) => todo!(),
                            Expr::Err(_) => {
                                Err(DerivedSignatureRawTermError::CannotInferArrayLength)?
                            }
                            ref _expr => {
                                Err(OriginalSignatureRawTermError::ExpectedLiteralForArrayLength)?
                            }
                        },
                        _ => todo!(),
                    },
                    _ => {
                        let  Ok(_function) = self.infer_new(function) else {
                            return  Err(DerivedSignatureRawTermError::CannotInferFunctionRawTermInApplication.into())
                        };
                        todo!()
                    }
                }
            }
            Expr::NewTuple { items, .. } => {
                p!(self.expr_region_data.path().debug(self.db));
                p!(items.len());
                todo!()
            }
            Expr::List { items, .. } => {
                let items = items
                    .into_iter()
                    .map(|item| self.infer_new(item))
                    .collect::<SignatureRawTermResult<Vec<_>>>()?;
                Ok(RawTermList::new(
                    self.db,
                    self.expr_region_data.path().toolchain(self.db),
                    items,
                )
                .into())
            }
            Expr::BoxColonList { .. } => todo!(),
            Expr::Bracketed { item, .. } => self.infer_new(item),
            Expr::Block { stmts: _ } => todo!(),
            Expr::IndexOrCompositionWithList {
                owner: _,
                lbox_token_idx: _,
                items: _indices,
                rbox_token_idx: _,
            } => todo!(),
            Expr::Err(_) => Err(DerivedSignatureRawTermError::ExprError.into()),
        }
    }

    pub(crate) fn current_symbol_term_symbol(
        &self,
        symbol: CurrentSymbolIdx,
    ) -> Option<RawTermSymbol> {
        self.term_symbol_region.current_symbol_term(symbol)
    }

    pub(crate) fn raw_term_menu(&self) -> &RawTermMenu {
        self.raw_term_menu
    }
}
