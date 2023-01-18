use super::*;
use husky_expr::*;

pub(crate) struct SignatureTermEngine<'a> {
    expr_arena: &'a ExprArena,
    entity_path_expr_arena: &'a EntityPathExprArena,
    expr_terms: ExprMap<SignatureTermResult<Term>>,
}

impl<'a> SignatureTermEngine<'a> {
    pub(crate) fn new(db: &'a dyn SignatureDb, expr_page: ExprPage) -> Self {
        let expr_arena = &expr_page.expr_arena(db);
        Self {
            expr_arena,
            entity_path_expr_arena: expr_page.entity_path_expr_arena(db),
            expr_terms: ExprMap::new(expr_arena),
        }
    }

    pub(crate) fn finish(self) -> SignatureTermSheet {
        todo!()
    }
}
