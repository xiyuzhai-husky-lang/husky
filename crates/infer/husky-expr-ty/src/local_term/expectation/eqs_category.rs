use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExpectEqsCategory {
    pub(crate) smallest_universe: TermUniverse,
}

impl ExpectEqsCategory {
    pub(crate) fn new_expect_eqs_ty_kind() -> Self {
        Self {
            smallest_universe: TermUniverse::new(1),
        }
    }
}

impl ExpectLocalTerm for ExpectEqsCategory {
    type Outcome = ExpectEqsSortOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        FinalDestination::Sort
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExpectEqsSortOutcome {
    destination: LocalTerm,
}

impl ExpectLocalTermOutcome for ExpectEqsSortOutcome {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast_ref(resolved_ok: &LocalTermExpectationOutcome) -> &Self {
        match resolved_ok {
            LocalTermExpectationOutcome::EqsSort(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_eqs_sort_expectation(
        &self,
        expectee: LocalTerm,
        expectation: &ExpectEqsCategory,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee {
            LocalTerm::Resolved(resolved_expectee) => {
                match resolved_expectee {
                    Term::Category(cat) => {
                        Some(match cat.universe() >= expectation.smallest_universe {
                            true => LocalTermExpectationEffect {
                                result: Ok(LocalTermExpectationOutcome::EqsSort(
                                    ExpectEqsSortOutcome {
                                        destination: expectee,
                                    },
                                )),
                                actions: vec![],
                            },
                            false => LocalTermExpectationEffect {
                                result: Err(todo!()),
                                actions: vec![],
                            },
                        })
                    }
                    _ => {
                        p!(self.path());
                        p!(resolved_expectee.debug(self.db()));
                        todo!()
                    }
                }
                // let expectee_ty = self.db().term_ty(expectee);
                // match expectee_ty {
                //     Ok(expectee_ty) if expectee_ty == self.reduced_term_menu().ty0() => todo!(),
                //     Ok(expectee_ty) => {
                //         p!(
                //             self.path(),
                //             expectee.debug(self.db()),
                //             expectee_ty.debug(self.db())
                //         );
                //         todo!()
                //     }
                //     Err(_) => todo!(),
                // }
            }
            LocalTerm::Unresolved(_) => None,
        }
    }
}

// LocalTermExpectationRuleVariant::Sort => match db.term_ty(resolved_term) {
//     Ok(term_ty) => match term_ty {
//         Term::Category(cat) => match cat.universe().raw() {
//             0 => todo!(),
//             _ => LocalTermExpectationResolveProgress::Resolved(
//                 LocalTermExpectationOutcome::OkSort {
//                     implicit_conversion: LocalTermImplicitConversion::None,
//                     local_term: resolved_term.into(),
//                 },
//             ),
//         },
//         _ => todo!(),
//     },
//     Err(_) => todo!(),
// },
