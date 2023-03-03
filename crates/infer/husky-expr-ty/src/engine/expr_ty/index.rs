use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_index_expr(
        &mut self,
        expr_idx: ExprIdx,
        owner: ExprIdx,
        indices: ExprIdxRange,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        todo!()
    }
}
