use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitConversion {
    None,
    Other,
}

/// expect a type that is implicitly convertible to dst
#[derive(Debug, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectImplicitlyConvertible {
    pub(crate) destination: LocalTerm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectImplicitlyConvertibleResolvedOk {
    implicit_conversion: ImplicitConversion,
    expectee: LocalTerm,
    destination: LocalTerm,
}

impl ExpectLocalTermResolvedOk for ExpectImplicitlyConvertibleResolvedOk {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast(resolved_ok: &LocalTermExpectationResolvedOk) -> Self {
        todo!()
    }
}

impl From<ExpectImplicitlyConvertible> for LocalTermExpectation {
    fn from(value: ExpectImplicitlyConvertible) -> Self {
        LocalTermExpectation::ImplicitlyConversion {
            destination: value.destination,
        }
    }
}

impl From<ExpectImplicitlyConvertibleResolvedOk> for LocalTermExpectationResolvedOk {
    fn from(value: ExpectImplicitlyConvertibleResolvedOk) -> Self {
        todo!()
    }
}

impl ExpectLocalTerm for ExpectImplicitlyConvertible {
    type ResolvedOk = ExpectImplicitlyConvertibleResolvedOk;

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
    ) -> Option<LocalTermExpectationResolvedOkM> {
        let table = self.local_term_table();
        match expectee {
            LocalTerm::Resolved(resolved_expectee) => match destination {
                LocalTerm::Resolved(destination) => self.res_to_res(resolved_expectee, destination),
                LocalTerm::Unresolved(dst) => match table[dst].unresolved_term() {
                    UnresolvedTerm::ImplicitSymbol(_) => match level {
                        LocalTermResolveLevel::Weak => None,
                        LocalTermResolveLevel::Strong => Some(LocalTermExpectationResolvedOkM {
                            actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                                implicit_symbol: dst,
                                substitution: resolved_expectee.into(),
                            }],
                            result: Ok(LocalTermExpectationResolvedOk::ImplicitlyConvertible(
                                ExpectImplicitlyConvertibleResolvedOk {
                                    implicit_conversion: ImplicitConversion::None,
                                    expectee,
                                    destination: expectee,
                                },
                            )),
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
    ) -> Option<LocalTermExpectationResolvedOkM> {
        p!(expectee.debug(self.db()), destination.debug(self.db()));
        todo!()
    }

    fn unres_to(
        &self,
        expectee: UnresolvedTermIdx,
        destination: LocalTerm,
        level: LocalTermResolveLevel,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        let table = self.local_term_table();
        match table[expectee].unresolved_term() {
            UnresolvedTerm::ImplicitSymbol(_) => match level {
                LocalTermResolveLevel::Weak => None,
                LocalTermResolveLevel::Strong => Some(LocalTermExpectationResolvedOkM {
                    actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                        implicit_symbol: expectee,
                        substitution: destination,
                    }],
                    result: Ok(LocalTermExpectationResolvedOk::ImplicitlyConvertible(
                        ExpectImplicitlyConvertibleResolvedOk {
                            implicit_conversion: ImplicitConversion::None,
                            expectee: destination.into(),
                            destination: destination.into(),
                        },
                    )),
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
    ) -> Option<LocalTermExpectationResolvedOkM> {
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
    ) -> Option<LocalTermExpectationResolvedOkM> {
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
                                        return Some(LocalTermExpectationResolvedOkM {
                                            result: Err(
                                                OriginalLocalTermExpectationError::Todo.into()
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
