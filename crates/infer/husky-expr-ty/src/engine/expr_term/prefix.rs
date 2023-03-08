use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_prefix_expr_term(
        &mut self,
        opr: PrefixOpr,
        opd: ExprIdx,
    ) -> ExprTermResult<LocalTerm> {
        let Some(opd_term) = self.infer_new_expr_term(opd) else {
           return Err(DerivedExprTermError::PrefixOprTermNotInferred.into())
        };
        match opr {
            PrefixOpr::Minus => todo!(),
            PrefixOpr::Not => todo!(),
            PrefixOpr::Tilde => todo!(),
            PrefixOpr::Ref => {
                // let opd_ty = self.infer
                // match
                todo!()
            }
            PrefixOpr::Vector => todo!(),
            PrefixOpr::Slice => todo!(),
            PrefixOpr::CyclicSlice => todo!(),
            PrefixOpr::Array(_) => todo!(),
            PrefixOpr::Option => todo!(),
        }
    }
}
