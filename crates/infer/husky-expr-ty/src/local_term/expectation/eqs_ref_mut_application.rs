use super::*;

/// for some lifetime `'a`, expect the term to be equal to `&mut 'a T`
/// for some T
///
/// T is referred to as the inner type
#[derive(Debug, Clone)]
pub(crate) struct ExpectRefMut {
    pub(crate) lifetime: LocalTerm,
}

impl ExpectLocalTerm for ExpectRefMut {
    type Result = ExpectRefMutResult;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

pub(crate) enum ExpectRefMutResult {
    ResolvedOk {
        term: LocalTerm,
        /// T
        inner_ty: LocalTerm,
    },
    ResolvedErr(LocalTermExpectationError),
}

impl From<ExpectRefMutResult> for LocalTermExpectationResult {
    fn from(value: ExpectRefMutResult) -> Self {
        todo!()
    }
}

impl From<ExpectRefMut> for LocalTermExpectation {
    fn from(value: ExpectRefMut) -> Self {
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
