use super::*;
use husky_expr::*;
use outcome::{Success, *};

pub(crate) struct SignatureTermEngine<'a> {
    db: &'a dyn SignatureDb,
    expr_arena: &'a ExprArena,
    entity_path_expr_arena: &'a EntityPathExprArena,
    term_menu: &'a TermMenu,
    symbol_page: &'a SymbolPage,
    expr_terms: ExprMap<SignatureTermOutcome<Term>>,
    symbol_term_registry: SymbolTermRegistry,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolTermRegistry {
    next_parameter: usize,
    next_lifetime: usize,
    next_binding: usize,
    inherited_symbol_terms: Vec<Option<Term>>,
    current_symbol_terms: Vec<Term>,
}

impl SymbolTermRegistry {
    fn new(
        parent: Option<&SymbolTermRegistry>,
        symbol_page: &SymbolPage,
        term_menu: &TermMenu,
    ) -> SymbolTermRegistry {
        let mut next_parameter = parent.map_or(0, |parent| parent.next_parameter);
        let mut next_lifetime = parent.map_or(0, |parent| parent.next_lifetime);
        let mut next_binding = parent.map_or(0, |parent| parent.next_binding);
        let inherited_symbol_terms = symbol_page
            .indexed_inherited_symbol_iter()
            .map(|(idx, symbol)| todo!())
            .collect();
        let current_symbol_terms = symbol_page
            .indexed_current_symbol_iter()
            .map(|(idx, symbol)| match symbol.variant() {
                CurrentSymbolVariant::ImplicitParameter {
                    implicit_parameter_variant,
                } => match implicit_parameter_variant {
                    ImplicitParameterVariant::Type { ident_token } => {
                        let ty0 = term_menu.ty0();
                        TermSymbol::new(idx)
                    }
                },
                CurrentSymbolVariant::Parameter { pattern_symbol } => todo!(),
                CurrentSymbolVariant::LetVariable { pattern_symbol } => todo!(),
                CurrentSymbolVariant::FrameVariable(_) => todo!(),
            })
            .collect();
        SymbolTermRegistry {
            next_parameter,
            next_lifetime,
            next_binding,
            inherited_symbol_terms,
            current_symbol_terms,
        }
    }
}

impl<'a> SignatureTermEngine<'a> {
    pub(crate) fn new(
        db: &'a dyn SignatureDb,
        expr_page: ExprPage,
        parent_symbol_term_registry: Option<&'a SymbolTermRegistry>,
    ) -> Self {
        let expr_arena = &expr_page.expr_arena(db);
        let toolchain = expr_page.toolchain(db);
        let symbol_page = expr_page.symbol_page(db);
        // ad hoc
        let term_menu = &db.term_menu(toolchain).as_ref().unwrap();
        Self {
            db,
            expr_arena,
            entity_path_expr_arena: expr_page.entity_path_expr_arena(db),
            symbol_page,
            term_menu,
            expr_terms: ExprMap::new(expr_arena),
            symbol_term_registry: SymbolTermRegistry::new(
                parent_symbol_term_registry,
                symbol_page,
                term_menu,
            ),
        }
    }

    // ask about the term for expr, assuming it hasn't been computed before
    pub(crate) fn query_new(&mut self, expr_idx: ExprIdx) -> Option<Term> {
        let outcome = self.calc(expr_idx);
        let term = outcome.ok_copy();
        self.save(expr_idx, outcome);
        term
    }

    pub(crate) fn finish(self) -> SignatureTermSheet {
        SignatureTermSheet::new(self.symbol_term_registry)
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
                ident,
                token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => {
                let base = self.symbol_page.inherited_symbol_arena().len();
                // let symbol = Term::new_symbol();
                todo!()
            }
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
            Expr::Be { .. } => todo!(),
            Expr::PrefixOpn {
                punctuation,
                punctuation_token_idx,
                opd,
            } => todo!(),
            Expr::SuffixOpn {
                opd,
                punctuation,
                punctuation_token_idx,
            } => todo!(),
            Expr::FunctionCall { .. } => todo!(),
            Expr::Field {
                this_expr,
                dot_token_idx,
                ident_token,
            } => todo!(),
            Expr::MethodCall { .. } => todo!(),
            Expr::TemplateInstantiation { .. } => todo!(),
            Expr::Application { function, argument } => {
                let Some(argument) = self.query_new(argument) else {
                        return Abort(SignatureTermAbortion::CannotInferArgumentTermInApplication)
                    };
                match self.expr_arena[function] {
                    Expr::BoxColon {
                        caller: None,
                        lbox_token_idx,
                        colon_token_idx,
                        rbox_token,
                    } => Success({
                        let db = self.db;
                        let function = self.term_menu.slice_type();
                        TermApplication::new(db, function, argument).into()
                    }),
                    Expr::NewBoxList {
                        caller: None,
                        lbox_token_idx,
                        items,
                        rbox_token_idx,
                    } => {
                        todo!()
                    }
                    _ => {
                        let Some(function) = self.query_new(function) else {
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
            Expr::Bracketed(_) => todo!(),
            Expr::Block { stmts } => todo!(),
            Expr::Err(_) => todo!(),
        }
    }
}
