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
    type ResolvedOk = ExpectEqsRefMutApplicationResolvedOk;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct ExpectEqsRefMutApplicationResolvedOk {
    destination: LocalTerm,
    /// T
    inner_ty: LocalTerm,
}

impl ExpectLocalTermResolvedOk for ExpectEqsRefMutApplicationResolvedOk {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast(resolved_ok: &LocalTermExpectationResolvedOk) -> Self {
        todo!()
    }
}

impl From<ExpectEqsRefMutApplicationResolvedOk> for LocalTermExpectationResolvedOk {
    fn from(value: ExpectEqsRefMutApplicationResolvedOk) -> Self {
        LocalTermExpectationResolvedOk::EqsRefMutApplication(value)
    }
}

impl From<ExpectEqsRefMutApplication> for LocalTermExpectation {
    fn from(value: ExpectEqsRefMutApplication) -> Self {
        LocalTermExpectation::EqsRefMutApplication { lifetime: todo!() }
    }
}
// LocalTermExpectationRuleVariant::RefMut { lifetime } => {
//     // ad hoc
//     LocalTermExpectationResolveProgress::Resolved(LocalTermExpectationResolvedOk::Err(
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
//                 LocalTermExpectationResolvedOk::Err(
//                     OriginalLocalTermExpectationError::Todo.into(),
//                 ),
//             )
//         }
//     }
// }
