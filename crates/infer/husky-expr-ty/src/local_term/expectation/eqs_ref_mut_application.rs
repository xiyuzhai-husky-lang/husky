use super::*;

/// for some lifetime `'a`, expect the term to be equal to `&mut 'a T`
/// for some T
///
/// T is referred to as the inner type
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectEqsRefMutApplication {
    pub(crate) lifetime: UnresolvedTermIdx,
}

impl const ProvideEntityPathTypeExpectation for ExpectEqsRefMutApplication {
    fn entity_path_ty_expectation(&self) -> EntityPathTypeExpectation {
        todo!()
    }
}

impl ExpectLocalTerm for ExpectEqsRefMutApplication {
    type Outcome = ExpectEqsRefMutApplicationOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct ExpectEqsRefMutApplicationOutcome {
    destination: LocalTerm,
    /// T
    inner_ty: LocalTerm,
}

impl ExpectLocalTermOutcome for ExpectEqsRefMutApplicationOutcome {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast_ref(resolved_ok: &LocalTermExpectationOutcome) -> &Self {
        match resolved_ok {
            LocalTermExpectationOutcome::EqsRefMutApplication(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_eqs_ref_mut_application_expectation(
        &self,
        expectee: LocalTerm,
        expectation: &ExpectEqsRefMutApplication,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee {
            LocalTerm::Resolved(expectee) => Some(LocalTermExpectationEffect {
                result: Err(todo!()),
                actions: vec![],
            }),
            LocalTerm::Unresolved(unresolved_expectee) => {
                match unresolved_terms[unresolved_expectee].unresolved_term() {
                    UnresolvedTerm::ImplicitSymbol(_) => todo!(),
                    UnresolvedTerm::TypeApplication {
                        ty_path: ty,
                        arguments,
                    } if *ty == self.entity_path_menu().ref_mut_ty_path() => {
                        todo!()
                    }
                    UnresolvedTerm::TypeApplication {
                        ty_path: ty,
                        arguments,
                    } => Some(LocalTermExpectationEffect {
                        result: Err(todo!()),
                        actions: vec![],
                    }),
                    UnresolvedTerm::Ritchie {
                        ritchie_kind,
                        parameter_tys,
                        return_ty,
                    } => todo!(),
                }
            }
        }
    }
}

// LocalTermExpectationRuleVariant::RefMut { lifetime } => {
//     // ad hoc
//     LocalTermExpectationResolveProgress::Resolved(LocalTermExpectationOutcome::Err(
//         todo!(),
//     ))
// }

// LocalTermExpectationRuleVariant::RefMut { lifetime } => {
//     match local_term_region[target].unresolved_term() {
//         UnresolvedTerm::ImplicitSymbol(_) => todo!(),
//         UnresolvedTerm::TypeApplication { ty, arguments }
//             if *ty == self.entity_path_menu().ref_mut_ty_path() =>
//         {
//             todo!()
//         }
//         UnresolvedTerm::TypeApplication { ty, arguments } => {
//             LocalTermExpectationResolveProgress::Resolved(
//                 LocalTermExpectationOutcome::Err(
//                     todo!(),
//                 ),
//             )
//         }
//     }
// }
