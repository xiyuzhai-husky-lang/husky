use super::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ExpectEqsSort {
    pub(crate) smallest_universe: TermUniverse,
}

impl const ProvideTypeContext for ExpectEqsSort {
    fn ty_context(&self) -> TypeContext {
        Default::default()
    }
}

impl ExpectLocalTerm for ExpectEqsSort {
    type ResolvedOk = ExpectEqsSortResolvedOk;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExpectEqsSortResolvedOk {
    destination: LocalTerm,
}

impl ExpectLocalTermResolvedOk for ExpectEqsSortResolvedOk {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast_ref(resolved_ok: &LocalTermExpectationResolvedOk) -> &Self {
        match resolved_ok {
            LocalTermExpectationResolvedOk::EqsSort(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl From<ExpectEqsSort> for LocalTermExpectation {
    fn from(value: ExpectEqsSort) -> Self {
        LocalTermExpectation::EqsSort {
            smallest_universe: value.smallest_universe,
        }
    }
}

impl From<ExpectEqsSortResolvedOk> for LocalTermExpectationResolvedOk {
    fn from(value: ExpectEqsSortResolvedOk) -> Self {
        todo!()
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_eqs_sort_expectation(
        &self,
        expectee: LocalTerm,
        smallest_universe: TermUniverse,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        match expectee {
            LocalTerm::Resolved(resolved_expectee) => {
                match resolved_expectee.term() {
                    Term::Category(cat) => Some(match cat.universe() >= smallest_universe {
                        true => LocalTermExpectationResolvedOkM {
                            result: Ok(LocalTermExpectationResolvedOk::EqsSort(
                                ExpectEqsSortResolvedOk {
                                    destination: expectee,
                                },
                            )),
                            actions: vec![],
                        },
                        false => LocalTermExpectationResolvedOkM {
                            result: Err(todo!()),
                            actions: vec![],
                        },
                    }),
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

// LocalTermExpectationRuleVariant::Sort => match db.term_ty(resolved_term.term()) {
//     Ok(term_ty) => match term_ty.term() {
//         Term::Category(cat) => match cat.universe().raw() {
//             0 => todo!(),
//             _ => LocalTermExpectationResolveProgress::Resolved(
//                 LocalTermExpectationResolvedOk::OkSort {
//                     implicit_conversion: LocalTermImplicitConversion::None,
//                     local_term: resolved_term.into(),
//                 },
//             ),
//         },
//         _ => todo!(),
//     },
//     Err(_) => todo!(),
// },
