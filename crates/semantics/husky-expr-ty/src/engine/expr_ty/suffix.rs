use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_suffix_expr_ty(
        &mut self,
        opd: ExprIdx,
        opr: SuffixOpr,
    ) -> ExprTypeResult<FluffyTerm> {
        // match opr {
        //     SuffixOpr::Incr => todo!(),
        //     SuffixOpr::Decr => todo!(),
        //     SuffixOpr::Unveil => todo!(),
        // }

        Err(OriginalExprTypeError::TodoSuffix.into())
    }
}
