use crate::*;
use husky_ast::AstText;
use husky_expr_syntax::RawExprMap;
use husky_term_pattern::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct TermPatternInferSheet {
    term_pattern_interner: TermPatternInterner,
    expr_results: RawExprMap<TermPatternInferEntry>,
}

impl TermPatternInferSheet {
    pub(crate) fn new(arena: &RawExprArena) -> Self {
        Self {
            term_pattern_interner: Default::default(),
            expr_results: RawExprMap::new(arena),
        }
    }

    pub(crate) fn insert_result(
        &mut self,
        expr: RawExprIdx,
        const_expr: TermPatternInferResult<Option<ConstExprPattern>>,
        ty: TermPatternInferResult<TermPatternItd>,
    ) {
        self.expr_results
            .insert_new(expr, TermPatternInferEntry { const_expr, ty })
    }

    pub(crate) fn insert_term_infer_result(
        &mut self,
        expr: RawExprIdx,
        term: TermPatternInferResult<TermItd>,
    ) {
        todo!()
    }

    pub(crate) fn const_expr(
        &self,
        expr: RawExprIdx,
    ) -> &TermPatternInferResult<Option<ConstExprPattern>> {
        &self.expr_results.get(expr).unwrap().const_expr
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
