use super::*;
use husky_expr::*;
use husky_opn_syntax::{BinaryOpr, PrefixOpr};
use husky_print_utils::p;
use salsa::DebugWithDb;

pub(super) struct SignatureTermEngine<'a> {
    db: &'a dyn SignatureDb,
    expr_region_data: &'a ExprRegionData,
    term_menu: &'a TermMenu,
    expr_terms: ExprMap<SignatureTermResult<Term>>,
    term_symbol_region: TermSymbolRegion,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TermSymbolRegion {
    registry: TermSymbolRegistry,
    inherited_symbol_terms: Vec<TermSymbol>,
    current_symbol_terms: Vec<TermSymbol>,
}

impl TermSymbolRegion {
    fn new(parent: Option<&TermSymbolRegion>, symbol_region: &SymbolRegion) -> Self {
        let registry = parent.map_or(Default::default(), |parent| parent.registry.clone());
        let inherited_symbol_terms = symbol_region
            .inherited_symbol_iter()
            .map(|symbol| {
                parent
                    .unwrap()
                    .parent_symbol_term(symbol.parent_symbol_idx())
            })
            .collect();
        Self {
            registry,
            inherited_symbol_terms,
            current_symbol_terms: Default::default(),
        }
    }

    fn parent_symbol_term(&self, parent_symbol_idx: ParentSymbolIdx) -> TermSymbol {
        match parent_symbol_idx {
            ParentSymbolIdx::Inherited(inherited_symbol_idx) => {
                self.inherited_symbol_term(inherited_symbol_idx)
            }
            ParentSymbolIdx::Current(current_symbol_idx) => {
                self.current_symbol_term(current_symbol_idx).unwrap()
            }
        }
    }

    pub fn inherited_symbol_term(&self, inherited_symbol_idx: InheritedSymbolIdx) -> TermSymbol {
        self.inherited_symbol_terms[inherited_symbol_idx.raw()]
    }

    pub fn current_symbol_term(&self, current_symbol_idx: CurrentSymbolIdx) -> Option<TermSymbol> {
        self.current_symbol_terms
            .get(current_symbol_idx.raw())
            .copied()
    }
}

impl<'a> SignatureTermEngine<'a> {
    pub(super) fn new(
        db: &'a dyn SignatureDb,
        expr_region: ExprRegion,
        parent_term_symbol_region: Option<&'a TermSymbolRegion>,
    ) -> Self {
        let toolchain = expr_region.toolchain(db);
        // ad hoc
        let term_menu = db.term_menu(toolchain).as_ref().unwrap();
        let expr_region_data = &expr_region.data(db);
        let mut this = Self {
            db,
            expr_region_data,
            term_menu,
            expr_terms: ExprMap::new(expr_region_data.expr_arena()),
            term_symbol_region: TermSymbolRegion::new(
                parent_term_symbol_region,
                expr_region_data.symbol_region(),
            ),
        };
        this.init_current_symbol_term_symbols();
        this.init_expr_roots();
        this
    }

    fn init_current_symbol_term_symbols(&mut self) {
        for (idx, symbol) in self
            .expr_region_data
            .symbol_region()
            .indexed_current_symbol_iter()
        {
            let ty = match symbol.variant() {
                CurrentSymbolVariant::ImplicitParameter {
                    implicit_parameter_variant,
                } => match implicit_parameter_variant {
                    ImplicitParameterVariant::Type { .. } => Ok(self.term_menu.ty0().into()),
                    ImplicitParameterVariant::Lifetime => todo!(),
                    _ => todo!(),
                },
                CurrentSymbolVariant::RegularParameter { pattern_symbol_idx } => {
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
                                Err(_) => Err(TermSymbolTypeErrorKind::SignatureTermError),
                            }
                        }
                    }
                }
                CurrentSymbolVariant::LetVariable { .. }
                | CurrentSymbolVariant::FrameVariable(_) => return,
            };
            self.term_symbol_region
                .current_symbol_terms
                .push(self.term_symbol_region.registry.new_symbol(self.db, ty))
        }
    }

    fn init_expr_roots(&mut self) {
        for expr_root in self.expr_region_data.roots() {
            match expr_root.kind() {
                ExprRootKind::BlockExpr => return,
                ExprRootKind::Type
                | ExprRootKind::Trait
                | ExprRootKind::OutputType
                | ExprRootKind::FieldType => (),
            }
            self.cache_new(expr_root.expr())
        }
    }

    // infer the term for expr, assuming it hasn't been computed before
    fn infer_new(&mut self, expr_idx: ExprIdx) -> SignatureTermResult<Term> {
        let result = self.calc(expr_idx);
        let result_export = match result {
            Ok(term) => Ok(term),
            Err(_) => Err(DerivedSignatureTermError::TermAbortion.into()),
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

    fn save(&mut self, expr_idx: ExprIdx, outcome: SignatureTermResult<Term>) {
        self.expr_terms.insert_new(expr_idx, outcome)
    }

    fn calc(&mut self, expr_idx: ExprIdx) -> SignatureTermResult<Term> {
        match self.expr_region_data.expr_arena()[expr_idx] {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => match entity_path {
                Some(entity_path) => Ok(Term::Entity(entity_path)),
                None => Err(DerivedSignatureTermError::InvalidEntityPath.into()),
            },
            Expr::InheritedSymbol {
                ident,
                token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => todo!(),
            Expr::CurrentSymbol {
                current_symbol_idx, ..
            } => Ok(self
                .term_symbol_region
                .current_symbol_term(current_symbol_idx)
                .expect("not none")
                .into()),
            Expr::FrameVarDecl {
                token_idx,
                ident,
                current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => todo!(),
            Expr::BinaryOpn {
                lopd, opr, ropd, ..
            } => {
                let  Ok(lopd) = self.infer_new(lopd) else {
                    return  Err(DerivedSignatureTermError::CannotInferOperandTermInPrefix.into());
                };
                let  Ok(ropd) = self.infer_new(ropd) else {
                    return  Err(DerivedSignatureTermError::CannotInferOperandTermInPrefix.into());
                };
                match opr {
                    BinaryOpr::PureClosed(_) => todo!(),
                    BinaryOpr::Comparison(_) => todo!(),
                    BinaryOpr::ShortcuitLogic(_) => todo!(),
                    BinaryOpr::Assign(_) => todo!(),
                    BinaryOpr::ScopeResolution => todo!(),
                    BinaryOpr::Curry => Ok(TermCurry::new(
                        self.db,
                        {
                            // ad hoc
                            Variance::Invariant
                        },
                        lopd,
                        ropd,
                    )
                    .into()),
                    BinaryOpr::As => todo!(),
                    BinaryOpr::Is => todo!(),
                    BinaryOpr::In => todo!(),
                }
            }
            Expr::Be { .. } => todo!(),
            Expr::PrefixOpn {
                opr,
                opr_token_idx,
                opd,
            } => {
                let  Ok(opd) = self.infer_new(opd) else {
                    return  Err(DerivedSignatureTermError::CannotInferOperandTermInPrefix.into());
                };
                let tmpl = match opr {
                    PrefixOpr::Minus => todo!(),
                    PrefixOpr::Not => todo!(),
                    PrefixOpr::BitNot => todo!(),
                    PrefixOpr::Ref => self.term_menu.eval_ref(),
                    PrefixOpr::Vector => todo!(),
                    PrefixOpr::Slice => todo!(),
                    PrefixOpr::CyclicSlice => todo!(),
                    PrefixOpr::Array(_) => todo!(),
                    PrefixOpr::Option => self.term_menu.option_ty(),
                };
                Ok(TermApplication::new(self.db, tmpl, opd).into())
            }
            Expr::SuffixOpn {
                opd,
                punctuation,
                punctuation_token_idx,
            } => todo!(),
            Expr::FunctionCall { function, .. } => {
                todo!()
            }
            Expr::Field {
                this_expr,
                dot_token_idx,
                ident_token,
            } => todo!(),
            Expr::MethodCall { .. } => todo!(),
            Expr::TemplateInstantiation { .. } => todo!(),
            Expr::ApplicationOrFunctionCall {
                function, argument, ..
            }
            | Expr::Application { function, argument } => {
                let  Ok(argument) = self.infer_new(argument) else {
                        return  Err(DerivedSignatureTermError::CannotInferArgumentTermInApplication.into())
                    };
                match self.expr_region_data.expr_arena()[function] {
                    Expr::BoxColon {
                        caller: None,
                        lbox_token_idx,
                        colon_token_idx,
                        rbox_token,
                    } => Ok(
                        TermApplication::new(self.db, self.term_menu.slice_ty(), argument).into(),
                    ),
                    Expr::NewBoxList {
                        caller: None,
                        items,
                        ..
                    } => match items.len() {
                        0 => Ok(
                            TermApplication::new(self.db, self.term_menu.vec_ty(), argument).into(),
                        ),
                        1 => match self.expr_region_data.expr_arena()[items.start()] {
                            Expr::Literal(_) => todo!(),
                            Expr::EntityPath {
                                entity_path_expr,
                                entity_path,
                            } => todo!(),
                            Expr::InheritedSymbol {
                                ident,
                                token_idx,
                                inherited_symbol_idx,
                                inherited_symbol_kind,
                            } => todo!(),
                            Expr::CurrentSymbol {
                                ident,
                                token_idx,
                                current_symbol_idx,
                                current_symbol_kind,
                            } => todo!(),
                            Expr::FrameVarDecl {
                                token_idx,
                                ident,
                                current_symbol_idx,
                                current_symbol_kind,
                            } => todo!(),
                            Expr::SelfType(_) => todo!(),
                            Expr::SelfValue(_) => todo!(),
                            Expr::BinaryOpn {
                                lopd,
                                opr,
                                opr_token_idx,
                                ropd,
                            } => todo!(),
                            Expr::Be {
                                src,
                                be_token_idx,
                                ref target,
                            } => todo!(),
                            Expr::PrefixOpn {
                                opr,
                                opr_token_idx,
                                opd,
                            } => todo!(),
                            Expr::SuffixOpn {
                                opd,
                                punctuation,
                                punctuation_token_idx,
                            } => todo!(),
                            Expr::ApplicationOrFunctionCall {
                                function,
                                lpar_token_idx,
                                argument,
                                rpar_token_idx,
                            } => todo!(),
                            Expr::FunctionCall {
                                function,
                                ref implicit_arguments,
                                lpar_token_idx,
                                arguments,
                                rpar_token_idx,
                            } => todo!(),
                            Expr::Field {
                                this_expr,
                                dot_token_idx,
                                ident_token,
                            } => todo!(),
                            Expr::MethodCall {
                                this_expr,
                                dot_token_idx,
                                ident_token,
                                ref implicit_arguments,
                                lpar_token_idx,
                                arguments,
                                rpar_token_idx,
                            } => todo!(),
                            Expr::TemplateInstantiation {
                                template,
                                ref implicit_arguments,
                            } => todo!(),
                            Expr::Application { function, argument } => todo!(),
                            Expr::Bracketed {
                                lpar_token_idx,
                                item,
                                rpar_token_idx,
                            } => todo!(),
                            Expr::NewTuple {
                                lpar_token_idx,
                                items,
                                ref commas,
                                rpar_token_idx,
                            } => todo!(),
                            Expr::NewBoxList {
                                caller,
                                lbox_token_idx,
                                items,
                                rbox_token_idx,
                            } => todo!(),
                            Expr::BoxColon {
                                caller,
                                lbox_token_idx,
                                colon_token_idx,
                                rbox_token,
                            } => todo!(),
                            Expr::Block { stmts } => todo!(),
                            Expr::Err(_) => {
                                Err(DerivedSignatureTermError::CannotInferArgumentTermInBoxList
                                    .into())
                            }
                        },
                        _ => todo!(),
                    },
                    _ => {
                        let  Ok(function) = self.infer_new(function) else {
                            return  Err(DerivedSignatureTermError::CannotInferFunctionTermInApplication.into())
                        };
                        todo!()
                    }
                }
            }
            Expr::NewTuple {
                lpar_token_idx,
                items,
                rpar_token_idx,
                ..
            } => {
                p!(self.expr_region_data.path().debug(self.db));
                p!(items.len());
                todo!()
            }
            Expr::NewBoxList {
                caller,
                lbox_token_idx,
                items,
                rbox_token_idx,
            } => todo!(),
            Expr::BoxColon {
                caller,
                lbox_token_idx,
                colon_token_idx,
                rbox_token,
            } => todo!(),
            Expr::Bracketed { item, .. } => self.infer_new(item),
            Expr::Block { stmts } => todo!(),
            Expr::Err(_) => Err(DerivedSignatureTermError::ExprError.into()),
        }
    }

    pub(crate) fn current_symbol_term_symbol(&self, symbol: CurrentSymbolIdx) -> TermSymbol {
        self.term_symbol_region.current_symbol_terms[symbol.raw()]
    }

    pub(crate) fn term_menu(&self) -> &TermMenu {
        self.term_menu
    }
}
