use crate::*;
use husky_ast::AstText;
use husky_expr_syntax::RawExprMap;
use husky_term_pattern::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct TermPatternInferSheet {
    ast_text: Arc<AstText>,
    term_pattern_interner: TermPatternInterner,
    expr_results: RawExprMap<TermPatternInferEntry>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TermPatternInferEntry {
    opt_term: Option<TermPatternInferResult<TermPatternItd>>,
    ty: TermPatternInferResult<TermPatternItd>,
    expectation: Option<TermPatternItd>,
}

impl TermPatternInferSheet {
    pub(crate) fn new(ast_text: Arc<AstText>) -> Self {
        Self {
            ast_text,
            term_pattern_interner: todo!(),
            expr_results: todo!(),
        }
    }

    pub(crate) fn insert_result(
        &mut self,
        expr: RawExprIdx,
        opt_term: Option<TermPatternInferResult<TermPatternItd>>,
        ty: TermPatternInferResult<TermPatternItd>,
        expectation: Option<TermPatternItd>,
    ) {
        self.expr_results.insert_new(
            expr,
            TermPatternInferEntry {
                opt_term,
                ty,
                expectation,
            },
        )
    }

    pub(crate) fn insert_term_infer_result(
        &mut self,
        expr: RawExprIdx,
        term: TermPatternInferResult<TermItd>,
    ) {
        todo!()
    }
    pub(crate) fn expr_opt_term(
        &self,
        expr: RawExprIdx,
    ) -> Option<&TermPatternInferResult<TermPatternItd>> {
        self.expr_results
            .get(expr)
            .map(|entry| entry.opt_term.as_ref())
            .flatten()
    }
}

impl TermPatternInferSheet {
    pub fn ast_text(&self) -> &AstText {
        todo!()
    }

    pub fn expr_ty_result(&self, expr: RawExprIdx) -> &TermPatternInferResult<Ty> {
        todo!()
    }
}
