use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_prefix_expr_term(
        &mut self,
        expr_idx: SemaExprIdx,
        opr: PrefixOpr,
        opd: SemaExprIdx,
    ) -> SemaExprTermResult<FluffyTerm> {
        todo!()
        // let Some(opd_term) = self.infer_expr_term(opd) else {
        //     return Err(DerivedExprTermError::PrefixOprTermNotInferred.into());
        // };
        // match opr {
        //     PrefixOpr::Minus => todo!(),
        //     PrefixOpr::Not => todo!(),
        //     PrefixOpr::Tilde => match self
        //         .expr_ty_info_variant(expr_idx)
        //         .map_err(|_| DerivedExprTermError::AmbiguousTilde)?
        //     {
        //         SemaExprData::Tilde(disambiguation) => match disambiguation {
        //             TildeDisambiguation::BitNot => todo!(),
        //             TildeDisambiguation::Leash => {
        //                 Ok(FluffyTerm::new_leashed(self, expr_idx, opd_term)?)
        //             }
        //         },
        //         _ => unreachable!(),
        //     },
        //     PrefixOpr::Ref => {
        //         // let opd_ty = self.infer
        //         // match
        //         todo!()
        //     }
        //     PrefixOpr::Vector => todo!(),
        //     PrefixOpr::Slice => todo!(),
        //     PrefixOpr::CyclicSlice => todo!(),
        //     PrefixOpr::Array(_) => todo!(),
        //     PrefixOpr::Option => Ok(FluffyTerm::new_application(
        //         self,
        //         expr_idx,
        //         self.term_menu.option_ty_ontology(),
        //         opd_term,
        //     )
        //     .map_err(|e| DerivedExprTermError::OptionApplicationTerm(e))?
        //     .into()),
        // }
    }
}
