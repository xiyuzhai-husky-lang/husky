use super::*;
use husky_expr::*;
use husky_opn_syntax::{BinaryOpr, PrefixOpr};
use husky_print_utils::p;
use outcome::{Success, *};
use salsa::DebugWithDb;

pub(crate) struct SignatureTermEngine<'a> {
    db: &'a dyn SignatureDb,
    path: ExprPath,
    expr_arena: &'a ExprArena,
    pattern_expr_region: &'a PatternExprRegion,
    entity_path_expr_arena: &'a EntityPathExprArena,
    term_menu: &'a TermMenu,
    symbol_region: &'a SymbolRegion,
    expr_terms: ExprMap<SignatureTermOutcome<Term>>,
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
            .indexed_inherited_symbol_iter()
            .map(|(idx, symbol)| todo!())
            .collect();
        Self {
            registry,
            inherited_symbol_terms,
            current_symbol_terms: Default::default(),
        }
    }

    fn current_symbol_term(&self, current_symbol_idx: CurrentSymbolIdx) -> TermSymbol {
        self.current_symbol_terms[current_symbol_idx.raw()]
    }
}

impl<'a> SignatureTermEngine<'a> {
    pub(crate) fn new(
        db: &'a dyn SignatureDb,
        expr_region: ExprRegion,
        parent_term_symbol_region: Option<&'a TermSymbolRegion>,
    ) -> Self {
        let expr_arena = &expr_region.expr_arena(db);
        let toolchain = expr_region.toolchain(db);
        let symbol_region = expr_region.symbol_region(db);
        // ad hoc
        let term_menu = db.term_menu(toolchain).as_ref().unwrap();
        let mut this = Self {
            db,
            path: expr_region.path(db),
            expr_arena,
            pattern_expr_region: expr_region.pattern_expr_region(db),
            entity_path_expr_arena: expr_region.entity_path_expr_arena(db),
            symbol_region,
            term_menu,
            expr_terms: ExprMap::new(expr_arena),
            term_symbol_region: TermSymbolRegion::new(parent_term_symbol_region, symbol_region),
        };
        this.init_current_symbol_term_symbols();
        this
    }

    fn init_current_symbol_term_symbols(&mut self) {
        let current_symbol_terms = self
            .symbol_region
            .indexed_current_symbol_iter()
            .map(|(idx, symbol)| {
                let ty = match symbol.variant() {
                    CurrentSymbolVariant::ImplicitParameter {
                        implicit_parameter_variant,
                    } => match implicit_parameter_variant {
                        ImplicitParameterVariant::Type { .. } => Ok(self.term_menu.ty0()),
                    },
                    CurrentSymbolVariant::Parameter { pattern_symbol } => {
                        let pattern_symbol = &self.pattern_expr_region[*pattern_symbol];
                        match pattern_symbol {
                            PatternSymbol::Atom(_) => todo!(),
                        }
                    }
                    CurrentSymbolVariant::LetVariable { pattern_symbol } => todo!(),
                    CurrentSymbolVariant::FrameVariable(_) => todo!(),
                };
                self.term_symbol_region.registry.new_symbol(self.db, ty)
            })
            .collect();
        self.term_symbol_region.current_symbol_terms = current_symbol_terms;
    }

    // ask about the term for expr, assuming it hasn't been computed before
    pub(crate) fn query_new(&mut self, expr_idx: ExprIdx) -> SignatureTermOutcome<Term> {
        let outcome = self.calc(expr_idx);
        let term = match outcome {
            Success(term) => Success(term),
            Failure(_) => todo!(),
            Abort(_) => Abort(SignatureTermAbortion::TermAbortion),
        };
        self.save(expr_idx, outcome);
        term
    }

    pub(crate) fn finish(self) -> SignatureTermSheet {
        SignatureTermSheet::new(self.term_symbol_region)
    }

    fn save(&mut self, expr_idx: ExprIdx, outcome: SignatureTermOutcome<Term>) {
        self.expr_terms.insert_new(expr_idx, outcome)
    }

    fn calc(&mut self, expr_idx: ExprIdx) -> SignatureTermOutcome<Term> {
        match self.expr_arena[expr_idx] {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => match entity_path {
                Some(entity_path) => Success(Term::Entity(entity_path)),
                None => Abort(SignatureTermAbortion::InvalidEntityPath),
            },
            Expr::InheritedSymbol {
                ident,
                token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => todo!(),
            Expr::CurrentSymbol {
                current_symbol_idx, ..
            } => Success(
                self.term_symbol_region
                    .current_symbol_term(current_symbol_idx)
                    .into(),
            ),
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
                let Success(lopd) = self.query_new(lopd) else {
                    return Abort(SignatureTermAbortion::CannotInferOperandTermInPrefix);
                };
                let Success(ropd) = self.query_new(ropd) else {
                    return Abort(SignatureTermAbortion::CannotInferOperandTermInPrefix);
                };
                match opr {
                    BinaryOpr::PureClosed(_) => todo!(),
                    BinaryOpr::Comparison(_) => todo!(),
                    BinaryOpr::ShortcuitLogic(_) => todo!(),
                    BinaryOpr::Assign(_) => todo!(),
                    BinaryOpr::ScopeResolution => todo!(),
                    BinaryOpr::Curry => Success(TermCurry::new(self.db, lopd, ropd).into()),
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
                let Success(opd) = self.query_new(opd) else {
                    return Abort(SignatureTermAbortion::CannotInferOperandTermInPrefix);
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
                Success(TermApplication::new(self.db, tmpl, opd).into())
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
                let Success(argument) = self.query_new(argument) else {
                        return Abort(SignatureTermAbortion::CannotInferArgumentTermInApplication)
                    };
                match self.expr_arena[function] {
                    Expr::BoxColon {
                        caller: None,
                        lbox_token_idx,
                        colon_token_idx,
                        rbox_token,
                    } => Success(
                        TermApplication::new(self.db, self.term_menu.slice_ty(), argument).into(),
                    ),
                    Expr::NewBoxList {
                        caller: None,
                        items,
                        ..
                    } => match items.len() {
                        0 => Success(
                            TermApplication::new(self.db, self.term_menu.vec_ty(), argument).into(),
                        ),
                        1 => match self.expr_arena[items.start()] {
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
                                Abort(SignatureTermAbortion::CannotInferArgumentTermInBoxList)
                            }
                        },
                        _ => todo!(),
                    },
                    _ => {
                        let Success(function) = self.query_new(function) else {
                            return Abort(SignatureTermAbortion::CannotInferFunctionTermInApplication)
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
                p!(self.path.debug(self.db));
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
            Expr::Bracketed { item, .. } => self.query_new(item),
            Expr::Block { stmts } => todo!(),
            Expr::Err(_) => Abort(SignatureTermAbortion::ExprError),
        }
    }

    pub(crate) fn current_symbol_term_symbol(&self, symbol: CurrentSymbolIdx) -> TermSymbol {
        self.term_symbol_region.current_symbol_terms[symbol.raw()]
    }

    pub(crate) fn term_menu(&self) -> &TermMenu {
        self.term_menu
    }
}
