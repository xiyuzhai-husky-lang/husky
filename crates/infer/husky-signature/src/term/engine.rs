use super::*;
use husky_expr::*;
use outcome::{Success, *};

pub(crate) struct SignatureTermEngine<'a> {
    db: &'a dyn SignatureDb,
    expr_arena: &'a ExprArena,
    entity_path_expr_arena: &'a EntityPathExprArena,
    term_menu: &'a TermMenu,
    symbol_sheet: &'a SymbolSheet,
    expr_terms: ExprMap<SignatureTermOutcome<Term>>,
    symbol_term_sheet: SymbolTermSheet,
}

pub(crate) struct SymbolTermSheet {}

impl<'a> SignatureTermEngine<'a> {
    pub(crate) fn new(
        db: &'a dyn SignatureDb,
        expr_page: ExprPage,
        parent_signature: Option<Signature>,
    ) -> Self {
        let expr_arena = &expr_page.expr_arena(db);
        let toolchain = expr_page.toolchain(db);
        Self {
            db,
            expr_arena,
            entity_path_expr_arena: expr_page.entity_path_expr_arena(db),
            symbol_sheet: expr_page.symbol_sheet(db),
            // ad hoc
            term_menu: db.term_menu(toolchain).as_ref().unwrap(),
            expr_terms: ExprMap::new(expr_arena),
            symbol_term_sheet: todo!(),
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
        SignatureTermSheet {}
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
                let base = self.symbol_sheet.inherited_symbol_arena().len();
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
                    } => Success(Term::new_application(
                        self.db,
                        self.term_menu.slice_type(),
                        argument,
                    )),
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
