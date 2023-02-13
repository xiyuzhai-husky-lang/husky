use super::*;

/// expect a type that is implicitly convertible to dst
#[derive(Debug, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectImplicitlyConvertible {
    pub(crate) destination: LocalTerm,
}

#[derive(Debug, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectImplicitConvertibleResult {}

impl From<ExpectImplicitlyConvertible> for LocalTermExpectation {
    fn from(value: ExpectImplicitlyConvertible) -> Self {
        LocalTermExpectation::ImplicitlyConversion {
            destination: value.destination,
        }
    }
}

impl From<ExpectImplicitConvertibleResult> for LocalTermExpectationResult {
    fn from(value: ExpectImplicitConvertibleResult) -> Self {
        todo!()
    }
}

impl ExpectLocalTerm for ExpectImplicitlyConvertible {
    type Result = ExpectImplicitConvertibleResult;

    fn destination(&self) -> Option<LocalTerm> {
        Some(self.destination)
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_implicit_conversion_expectation(
        &self,
        expectee: LocalTerm,
        destination: LocalTerm,
        level: LocalTermResolveLevel,
    ) -> Option<LocalTermExpectationResultM> {
        let table = self.local_term_table();
        match expectee {
            LocalTerm::Resolved(expectee) => match destination {
                LocalTerm::Resolved(destination) => self.res_to_res(expectee, destination),
                LocalTerm::Unresolved(dst) => match table[dst].unresolved_term() {
                    UnresolvedTerm::ImplicitSymbol(_) => match level {
                        LocalTermResolveLevel::Weak => None,
                        LocalTermResolveLevel::Strong => Some(LocalTermExpectationResultM {
                            actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                                implicit_symbol: dst,
                                substitution: expectee.into(),
                            }],
                            result: LocalTermExpectationResult::OkImplicitConversion {
                                implicit_conversion: LocalTermImplicitConversion::None,
                                local_term: expectee.into(),
                            },
                        }),
                    },
                    UnresolvedTerm::TypeApplication { ty, arguments } => todo!(),
                },
            },
            LocalTerm::Unresolved(expectee) => self.unres_to(expectee, destination, level),
        }
    }

    fn res_to_res(
        &self,
        expectee: ReducedTerm,
        destination: ReducedTerm,
    ) -> Option<LocalTermExpectationResultM> {
        p!(expectee.debug(self.db()), destination.debug(self.db()));
        todo!()
    }

    fn unres_to(
        &self,
        expectee: UnresolvedTermIdx,
        destination: LocalTerm,
        level: LocalTermResolveLevel,
    ) -> Option<LocalTermExpectationResultM> {
        let table = self.local_term_table();
        match table[expectee].unresolved_term() {
            UnresolvedTerm::ImplicitSymbol(_) => match level {
                LocalTermResolveLevel::Weak => None,
                LocalTermResolveLevel::Strong => Some(LocalTermExpectationResultM {
                    actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                        implicit_symbol: expectee,
                        substitution: destination,
                    }],
                    result: LocalTermExpectationResult::OkImplicitConversion {
                        implicit_conversion: LocalTermImplicitConversion::None,
                        local_term: destination,
                    },
                }),
            },
            UnresolvedTerm::TypeApplication { ty, arguments } => {
                self.unres_ty_app_to(*ty, arguments, destination)
            }
        }
    }

    /// expectation rule effect for implicit conversion from unresolved type application to unresolved expectee
    fn unres_ty_app_to(
        &self,
        ty: TypePath,
        arguments: &[LocalTerm],
        destination: LocalTerm,
    ) -> Option<LocalTermExpectationResultM> {
        match ty {
            ty if ty == self.entity_path_menu().ref_ty_path() => {
                todo!()
            }
            ty if ty == self.entity_path_menu().ref_mut_ty_path() => {
                todo!()
            }
            ty => self.generic_unres_ty_app_to(ty, arguments, destination),
        }
    }

    fn generic_unres_ty_app_to(
        &self,
        ty: TypePath,
        arguments: &[LocalTerm],
        destination: LocalTerm,
    ) -> Option<LocalTermExpectationResultM> {
        match destination {
            LocalTerm::Resolved(destination) => {
                let destination_expansion = self.db().application_expansion(destination);
                match destination_expansion.f() {
                    Term::Literal(_) => todo!(),
                    Term::Symbol(_) => todo!(),
                    Term::Entity(dst_f) => {
                        match dst_f {
                            EntityPath::Module(_) => todo!(),
                            EntityPath::ModuleItem(destination_f) => match destination_f {
                                ModuleItemPath::Type(destination_f) => {
                                    if destination_f != ty {
                                        return Some(LocalTermExpectationResultM {
                                            result: LocalTermExpectationResult::Err(
                                                OriginalLocalTermExpectationError::Todo.into(),
                                            ),
                                            actions: vec![],
                                        });
                                    }
                                    todo!()
                                }
                                ModuleItemPath::Trait(_) => todo!(),
                                ModuleItemPath::Form(_) => todo!(),
                            },
                            EntityPath::AssociatedItem(_) => todo!(),
                            EntityPath::Variant(_) => todo!(),
                        }
                        // let dst_arguments = &dst_expansion.arguments(self.db());
                        // if dst_arguments.len() != arguments.len() {
                        //     return Err(todo!());
                        // }
                        // p!(dst.debug(self.db()));
                        todo!()
                    }
                    Term::Category(_) => todo!(),
                    Term::Universe(_) => todo!(),
                    Term::Curry(_) => todo!(),
                    Term::Ritchie(_) => todo!(),
                    Term::Abstraction(_) => todo!(),
                    Term::Application(_) => todo!(),
                    Term::Subentity(_) => todo!(),
                    Term::AsTraitSubentity(_) => todo!(),
                    Term::TraitConstraint(_) => todo!(),
                }
            }
            LocalTerm::Unresolved(_) => todo!(),
        }
    }
}
