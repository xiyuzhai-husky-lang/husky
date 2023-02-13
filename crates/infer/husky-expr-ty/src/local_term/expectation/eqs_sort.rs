use super::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ExpectSort {
    pub(crate) smallest_universe: TermUniverse,
}

impl ExpectLocalTerm for ExpectSort {
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

    fn downcast(resolved_ok: &LocalTermExpectationResolvedOk) -> Self {
        todo!()
    }
}

impl From<ExpectSort> for LocalTermExpectation {
    fn from(value: ExpectSort) -> Self {
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
    pub(super) fn resolve_eq_sort_expectation(
        &self,
        smallest_universe: TermUniverse,
        expectee: LocalTerm,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        todo!()
        // match expectee {
        //     LocalTerm::Resolved(expectee) => {
        //         let expectee_ty = self.db().term_ty(expectee);
        //         match expectee_ty {
        //             Ok(expectee_ty) if expectee_ty == self.reduced_term_menu().ty0() => todo!(),
        //             Ok(expectee_ty) => {
        //                 p!(
        //                     self.path(),
        //                     expectee.debug(self.db()),
        //                     expectee_ty.debug(self.db())
        //                 );
        //                 todo!()
        //             }
        //             Err(_) => todo!(),
        //         }
        //     }
        //     LocalTerm::Unresolved(_) => todo!(),
        // }
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
