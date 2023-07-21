use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_prefix_expr_term(
        &mut self,
        expr_idx: ExprIdx,
        opr: PrefixOpr,
        opd: ExprIdx,
    ) -> ExprTermResult<FluffyTerm> {
        let Some(opd_term) = self.infer_new_expr_term(opd) else {
           return Err(DerivedExprTermError::PrefixOprTermNotInferred.into())
        };
        match opr {
            PrefixOpr::Minus => todo!(),
            PrefixOpr::Not => todo!(),
            PrefixOpr::Tilde => match self
                .expr_ty_info_variant(expr_idx)
                .map_err(|_| DerivedExprTermError::AmbiguousTilde)?
            {
                ExprDisambiguation::Tilde(disambiguation) => match disambiguation {
                    TildeDisambiguation::BitNot => todo!(),
                    TildeDisambiguation::Leash => {
                        Ok(FluffyTerm::new_leashed(self, expr_idx, opd_term)?)
                    }
                },
                _ => unreachable!(),
            },
            PrefixOpr::Ref => {
                // let opd_ty = self.infer
                // match
                todo!()
            }
            PrefixOpr::Vector => todo!(),
            PrefixOpr::Slice => todo!(),
            PrefixOpr::CyclicSlice => todo!(),
            PrefixOpr::Array(_) => todo!(),
            PrefixOpr::Option => Ok(FluffyTerm::new_application(
                self,
                expr_idx,
                self.term_menu.option_ty_ontology(),
                opd_term,
            )
            .map_err(|e| DerivedExprTermError::OptionApplicationTerm(e))?
            .into()),
        }
    }
}
