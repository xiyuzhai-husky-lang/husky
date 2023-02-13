use super::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ExpectSort {
    pub(crate) smallest_universe: TermUniverse,
}

impl ExpectLocalTerm for ExpectSort {
    type Result = ExpectSortResult;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

pub(crate) enum ExpectSortResult {
    ResolvedOk(LocalTerm),
    ResolvedErr(LocalTermExpectationError),
}

impl From<ExpectSort> for LocalTermExpectation {
    fn from(value: ExpectSort) -> Self {
        LocalTermExpectation::EqsSort {
            smallest_universe: value.smallest_universe,
        }
    }
}

impl From<ExpectSortResult> for LocalTermExpectationResult {
    fn from(value: ExpectSortResult) -> Self {
        todo!()
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_eq_sort_expectation(
        &self,
        smallest_universe: TermUniverse,
        expectee: LocalTerm,
    ) -> Option<LocalTermExpectationResultM> {
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
//                 LocalTermExpectationResult::OkSort {
//                     implicit_conversion: LocalTermImplicitConversion::None,
//                     local_term: resolved_term.into(),
//                 },
//             ),
//         },
//         _ => todo!(),
//     },
//     Err(_) => todo!(),
// },
