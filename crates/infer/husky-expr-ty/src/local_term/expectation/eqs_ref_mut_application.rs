use super::*;

/// for some lifetime `'a`, expect the term to be equal to `&mut 'a T`
/// for some T
///
/// T is referred to as the inner type
#[derive(Debug, Clone)]
pub(crate) struct ExpectEqsRefMutApplication {
    pub(crate) lifetime: LocalTerm,
}

impl const ProvideTypeContext for ExpectEqsRefMutApplication {
    fn ty_context(&self) -> TypeContext {
        Default::default()
    }
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

    fn downcast_ref(resolved_ok: &LocalTermExpectationResolvedOk) -> &Self {
        match resolved_ok {
            LocalTermExpectationResolvedOk::EqsRefMutApplication(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl From<ExpectEqsRefMutApplicationResolvedOk> for LocalTermExpectationResolvedOk {
    fn from(value: ExpectEqsRefMutApplicationResolvedOk) -> Self {
        LocalTermExpectationResolvedOk::EqsRefMutApplication(value)
    }
}

impl From<ExpectEqsRefMutApplication> for LocalTermExpectation {
    fn from(value: ExpectEqsRefMutApplication) -> Self {
        LocalTermExpectation::EqsRefMutApplication {
            lifetime: value.lifetime,
        }
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_eqs_ref_mut_application_expectation(
        &self,
        expectee: LocalTerm,
        lifetime: LocalTerm,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        match expectee {
            LocalTerm::Resolved(expectee) => Some(LocalTermExpectationResolvedOkM {
                result: Err(todo!()),
                actions: vec![],
            }),
            LocalTerm::Unresolved(unresolved_expectee) => {
                match self.local_term_table()[unresolved_expectee].unresolved_term() {
                    UnresolvedTerm::ImplicitSymbol(_) => todo!(),
                    UnresolvedTerm::TypeApplication { ty, arguments }
                        if *ty == self.entity_path_menu().ref_mut_ty_path() =>
                    {
                        todo!()
                    }
                    UnresolvedTerm::TypeApplication { ty, arguments } => {
                        Some(LocalTermExpectationResolvedOkM {
                            result: Err(todo!()),
                            actions: vec![],
                        })
                    }
                }
            }
        }
    }
}

// LocalTermExpectationRuleVariant::RefMut { lifetime } => {
//     // ad hoc
//     LocalTermExpectationResolveProgress::Resolved(LocalTermExpectationResolvedOk::Err(
//         todo!(),
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
//                     todo!(),
//                 ),
//             )
//         }
//     }
// }
