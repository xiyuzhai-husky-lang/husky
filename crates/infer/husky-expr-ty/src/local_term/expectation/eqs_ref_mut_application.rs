use super::*;

/// for some lifetime `'a`, expect the term to be equal to `&mut 'a T`
/// for some T
///
/// T is referred to as the inner type
#[derive(Debug, Clone)]
pub(crate) struct ExpectEqsRefMutApplication {
    pub(crate) lifetime: LocalTerm,
}

impl ExpectLocalTerm for ExpectEqsRefMutApplication {
    type Result = ExpectEqsRefMutApplicationResult;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct ExpectEqsRefMutApplicationResult {
    term: LocalTerm,
    /// T
    inner_ty: LocalTerm,
}

impl From<ExpectEqsRefMutApplicationResult> for LocalTermExpectationResult {
    fn from(value: ExpectEqsRefMutApplicationResult) -> Self {
        LocalTermExpectationResult::OkEqsRefMutApplication(value)
    }
}

impl From<ExpectEqsRefMutApplication> for LocalTermExpectation {
    fn from(value: ExpectEqsRefMutApplication) -> Self {
        LocalTermExpectation::EqsRefMutApplication { lifetime: todo!() }
    }
}
// LocalTermExpectationRuleVariant::RefMut { lifetime } => {
//     // ad hoc
//     LocalTermExpectationResolveProgress::Resolved(LocalTermExpectationResult::Err(
//         OriginalLocalTermExpectationError::Todo.into(),
//     ))
// }

// LocalTermExpectationRuleVariant::RefMut { lifetime } => {
//     match self.local_term_table()[target].unresolved_term() {
//         UnresolvedTerm::ImplicitSymbol(_) => todo!(),
//         UnresolvedTerm::TypeApplication { ty, arguments }
//             if *ty == self.entity_path_menu().ref_mut_ty_path() =>
//         {
//             todo!()
//         }
//         UnresolvedTerm::TypeApplication { ty, arguments } => {
//             LocalTermExpectationResolveProgress::Resolved(
//                 LocalTermExpectationResult::Err(
//                     OriginalLocalTermExpectationError::Todo.into(),
//                 ),
//             )
//         }
//     }
// }
