use super::*;

/// expect a type that is implicitly convertible to dst
#[derive(Debug, Clone, Copy)]
pub(crate) struct ExpectImplicitConversion {
    pub(crate) destination: LocalTerm,
}

pub(crate) struct ExpectImplicitConversionResult {}

impl From<ExpectImplicitConversion> for LocalTermExpectation {
    fn from(value: ExpectImplicitConversion) -> Self {
        LocalTermExpectation::ImplicitlyConversion {
            destination: value.destination,
        }
    }
}

impl From<ExpectImplicitConversionResult> for LocalTermExpectationResult {
    fn from(value: ExpectImplicitConversionResult) -> Self {
        todo!()
    }
}

impl ExpectLocalTerm for ExpectImplicitConversion {
    type Result = ExpectImplicitConversionResult;

    fn destination(&self) -> Option<LocalTerm> {
        Some(self.destination)
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_implicit_conversion(
        &self,
        rule: &LocalTermExpectationEntry,
        dst: LocalTerm,
        table: &LocalTermTable,
        level: LocalTermResolveLevel,
    ) -> Option<LocalTermExpectationResultM> {
        match rule.expectee {
            LocalTerm::Resolved(expectee) => match dst {
                LocalTerm::Resolved(_) => todo!(),
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
            LocalTerm::Unresolved(expectee) => self.to_unres(table, expectee, level, dst),
        }
    }

    fn to_unres(
        &self,
        table: &LocalTermTable,
        expectee: UnresolvedTermIdx,
        level: LocalTermResolveLevel,
        dst: LocalTerm,
    ) -> Option<LocalTermExpectationResultM> {
        match table[expectee].unresolved_term() {
            UnresolvedTerm::ImplicitSymbol(_) => match level {
                LocalTermResolveLevel::Weak => None,
                LocalTermResolveLevel::Strong => Some(LocalTermExpectationResultM {
                    actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                        implicit_symbol: expectee,
                        substitution: dst,
                    }],
                    result: LocalTermExpectationResult::OkImplicitConversion {
                        implicit_conversion: LocalTermImplicitConversion::None,
                        local_term: dst,
                    },
                }),
            },
            UnresolvedTerm::TypeApplication { ty, arguments } => {
                self.from_unres_ty_app_to_unres(*ty, dst)
            }
        }
    }

    /// expectation rule effect for implicit conversion from unresolved type application to unresolved expectee
    fn from_unres_ty_app_to_unres(
        &self,
        ty: TypePath,
        dst: LocalTerm,
    ) -> Option<LocalTermExpectationResultM> {
        match ty {
            ty if ty == self.entity_path_menu().ref_ty_path() => {
                todo!()
            }
            ty if ty == self.entity_path_menu().ref_mut_ty_path() => {
                todo!()
            }
            ty => self.from_generic_unres_ty_app_to_unres(dst, ty),
        }
    }

    fn from_generic_unres_ty_app_to_unres(
        &self,
        dst: LocalTerm,
        ty: TypePath,
    ) -> Option<LocalTermExpectationResultM> {
        match dst {
            LocalTerm::Resolved(dst) => {
                let dst_expansion = self.db().application_expansion(dst);
                match dst_expansion.f() {
                    Term::Literal(_) => todo!(),
                    Term::Symbol(_) => todo!(),
                    Term::Entity(dst_f) => {
                        match dst_f {
                            EntityPath::Module(_) => todo!(),
                            EntityPath::ModuleItem(dst_f) => match dst_f {
                                ModuleItemPath::Type(dst_f) => {
                                    if dst_f != ty {
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
