use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_prefix_expr_term(
        &mut self,
        expr_idx: ExprIdx,
        opr: PrefixOpr,
        opd: ExprIdx,
    ) -> ExprTermResult<LocalTerm> {
        let Some(opd_term) = self.infer_new_expr_term(opd) else {
           return Err(DerivedExprTermError::PrefixOprTermNotInferred.into())
        };
        match opr {
            PrefixOpr::Minus => todo!(),
            PrefixOpr::Not => todo!(),
            PrefixOpr::Tilde => match self
                .expr_disambiguation(expr_idx)
                .map_err(|_| DerivedExprTermError::AmbiguousTilde)?
            {
                ExprDisambiguation::Tilde(disambiguation) => match disambiguation {
                    TildeDisambiguation::BitNot => todo!(),
                    TildeDisambiguation::Leash => match opd_term {
                        LocalTerm::Resolved(opd_term) => Ok(TermApplication::new(
                            self.db,
                            self.term_menu.leash_ty_ontology(),
                            opd_term,
                        )
                        .map_err(|e| DerivedExprTermError::TildeApplicationTerm(e))?
                        .into()),
                        LocalTerm::Unresolved(_) => todo!(),
                    },
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
            PrefixOpr::Option => Ok(LocalTerm::new_application(
                self.db,
                &mut self.local_term_region,
                expr_idx,
                self.term_menu.leash_ty_ontology(),
                opd_term,
            )
            .map_err(|e| DerivedExprTermError::OptionApplicationTerm(e))?
            .into()),
        }
    }
}
