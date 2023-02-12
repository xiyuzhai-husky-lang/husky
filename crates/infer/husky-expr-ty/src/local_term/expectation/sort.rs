use super::*;

#[derive(Debug, Clone)]
pub(crate) struct ExpectSort;

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
        LocalTermExpectation::Sort
    }
}

impl From<ExpectSortResult> for LocalTermExpectationResult {
    fn from(value: ExpectSortResult) -> Self {
        todo!()
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
