use super::*;

/// for some lifetime `'a`, expect the term to be equal to `&mut 'a T`
/// for some T
///
/// T is referred to as the inner type
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectEqsRefMutApplication {
    pub(crate) lifetime: UnresolvedTermIdx,
}

impl ExpectLocalTerm for ExpectEqsRefMutApplication {
    type Outcome = ExpectEqsRefMutApplicationOutcome;

    fn retrieve_outcome(outcome: &LocalTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            LocalTermExpectationOutcome::EqsRefMutApplication(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        todo!()
    }

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
                    UnresolvedTerm::TypeOntology {
                        path: ty,
                        arguments,
                    } if *ty == self.entity_path_menu().ref_mut_ty_path() => {
                        todo!()
                    }
                    UnresolvedTerm::TypeOntology {
                        path: ty,
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
