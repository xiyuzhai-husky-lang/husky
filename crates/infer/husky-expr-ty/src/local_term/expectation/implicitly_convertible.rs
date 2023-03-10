use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitConversion {
    None,
    Never,
    Other,
}

/// expect a type that is implicitly convertible to dst
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectImplicitlyConvertible {
    pub(crate) destination: LocalTerm,
}

impl ExpectImplicitlyConvertible {
    pub(in super::super) fn try_substitute_unresolved_local_term<'a>(
        &self,
        unresolved_terms: &'a UnresolvedTerms,
    ) -> Result<Option<LocalTermExpectation>, &'a LocalTermResolveError> {
        match unresolved_terms.try_reduce_local_term(self.destination)? {
            Some(destination) => Ok(Some(ExpectImplicitlyConvertible { destination }.into())),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectImplicitlyConvertibleOutcome {
    implicit_conversion: ImplicitConversion,
}

impl ExpectLocalTerm for ExpectImplicitlyConvertible {
    type Outcome = ExpectImplicitlyConvertibleOutcome;

    fn retrieve_outcome(outcome: &LocalTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            LocalTermExpectationOutcome::ImplicitlyConvertible(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        self.destination.final_destination(db, unresolved_terms)
    }

    fn destination(&self) -> Option<LocalTerm> {
        Some(self.destination)
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_implicitly_convertible2(
        &self,
        expectee: LocalTerm,
        destination: LocalTerm,
        level: LocalTermResolveLevel,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee {
            LocalTerm::Resolved(resolved_expectee) => match destination {
                LocalTerm::Resolved(destination) => self
                    .resolved_expectee_implicitly_convertible_to_resolved_destination(
                        resolved_expectee,
                        destination,
                        unresolved_terms,
                    ),
                LocalTerm::Unresolved(destination) => {
                    match unresolved_terms[destination].unresolved_term() {
                        UnresolvedTerm::ImplicitSymbol(_) => match level {
                            LocalTermResolveLevel::Weak => None,
                            LocalTermResolveLevel::Strong => Some(LocalTermExpectationEffect {
                                actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                                    implicit_symbol: destination,
                                    substitution: resolved_expectee.into(),
                                }],
                                result: Ok(LocalTermExpectationOutcome::ImplicitlyConvertible(
                                    ExpectImplicitlyConvertibleOutcome {
                                        implicit_conversion: ImplicitConversion::None,
                                    },
                                )),
                            }),
                        },
                        UnresolvedTerm::TypeApplication { ty_path, arguments } => {
                            let ty_path = *ty_path;
                            match resolved_expectee {
                                Term::Literal(_) => todo!(),
                                Term::Symbol(_) => todo!(),
                                Term::EntityPath(_) => todo!(),
                                Term::Category(_) => todo!(),
                                Term::Universe(_) => todo!(),
                                Term::Curry(_) => todo!(),
                                Term::Ritchie(_) => todo!(),
                                Term::Abstraction(_) => todo!(),
                                Term::Application(resolved_expectee_application) => {
                                    let resolved_expectee_application_expansion =
                                        self.db().term_application_expansion(resolved_expectee);
                                    match resolved_expectee_application_expansion.f() {
                                        Term::EntityPath(TermEntityPath::TypeOntology(f))
                                            if f == ty_path =>
                                        {
                                            todo!()
                                            // match ty_ontology_path_ty_unchecked(self.db(), ty_path) {
                                            //     Ok(_) => todo!(),
                                            //     Err(error) => Some(LocalTermExpectationEffect {
                                            //         result: Err(match error {
                                            //             TypeError::Original(_) => todo!(),
                                            //             TypeError::Derived(error) => LocalTermExpectationError::Derived(
                                            //                 DerivedLocalTermExpectationError::TypePathTypeError { ty_path, error }
                                            //             ),
                                            //         }),
                                            //         actions: vec![],
                                            //     }),
                                            // }
                                        }
                                        _ => todo!(),
                                    }
                                }
                                Term::Subentity(_) => todo!(),
                                Term::AsTraitSubentity(_) => todo!(),
                                Term::TraitConstraint(_) => todo!(),
                            }
                        }
                        UnresolvedTerm::Ritchie {
                            ritchie_kind,
                            parameter_tys,
                            return_ty,
                        } => todo!(),
                    }
                }
            },
            LocalTerm::Unresolved(expectee) => {
                self.unres_to(expectee, destination, level, unresolved_terms)
            }
        }
    }

    fn resolved_expectee_implicitly_convertible_to_resolved_destination(
        &self,
        expectee: Term,
        destination: Term,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee {
            expectee if expectee == destination => Some(LocalTermExpectationEffect {
                result: Ok(ExpectImplicitlyConvertibleOutcome {
                    implicit_conversion: ImplicitConversion::None,
                }
                .into()),
                actions: vec![],
            }),
            expectee if expectee == self.term_menu().never() => Some(LocalTermExpectationEffect {
                result: Ok(ExpectImplicitlyConvertibleOutcome {
                    implicit_conversion: ImplicitConversion::Never,
                }
                .into()),
                actions: vec![],
            }),
            _ => Some(LocalTermExpectationEffect {
                result: Err(todo!()),
                actions: vec![],
            }),
        }
    }

    fn unres_to(
        &self,
        expectee: UnresolvedTermIdx,
        destination: LocalTerm,
        level: LocalTermResolveLevel,
        unresolved_terms: &UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match unresolved_terms[expectee].unresolved_term() {
            UnresolvedTerm::ImplicitSymbol(_) => match level {
                LocalTermResolveLevel::Weak => None,
                LocalTermResolveLevel::Strong => Some(LocalTermExpectationEffect {
                    result: Ok(LocalTermExpectationOutcome::ImplicitlyConvertible(
                        ExpectImplicitlyConvertibleOutcome {
                            implicit_conversion: ImplicitConversion::None,
                        },
                    )),
                    actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                        implicit_symbol: expectee,
                        substitution: destination,
                    }],
                }),
            },
            UnresolvedTerm::TypeApplication {
                ty_path: ty,
                arguments,
            } => self.unresolved_ty_app_expectee_implicitly_convertible_to(
                *ty,
                arguments,
                destination,
                unresolved_terms,
            ),
            UnresolvedTerm::Ritchie {
                ritchie_kind,
                parameter_tys,
                return_ty,
            } => todo!(),
        }
    }

    /// expectation rule effect for implicit conversion from unresolved type application to unresolved expectee
    fn unresolved_ty_app_expectee_implicitly_convertible_to(
        &self,
        ty: TypePath,
        arguments: &[LocalTerm],
        destination: LocalTerm,
        unresolved_terms: &UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match ty {
            ty if ty == self.entity_path_menu().ref_ty_path() => {
                todo!()
            }
            ty if ty == self.entity_path_menu().ref_mut_ty_path() => {
                todo!()
            }
            ty => self.generic_unresolved_ty_app_expectee_implicitly_convertible_to(
                ty,
                arguments,
                destination,
            ),
        }
    }

    fn generic_unresolved_ty_app_expectee_implicitly_convertible_to(
        &self,
        ty_path: TypePath,
        arguments: &[LocalTerm],
        destination: LocalTerm,
    ) -> Option<LocalTermExpectationEffect> {
        match destination {
            LocalTerm::Resolved(destination) => {
                let destination_expansion = self.db().term_application_expansion(destination);
                match destination_expansion.f() {
                    Term::Literal(_) => todo!(),
                    Term::Symbol(_) => todo!(),
                    Term::EntityPath(destination_ty_path) => {
                        match destination_ty_path.ty_ontology() {
                            Some(destination_ty_path) if destination_ty_path == ty_path =>(),
                            Some(destination_ty_path) /* if destination_ty_path!=ty_path */ => {
                                p!(self.path(), destination_ty_path.debug(self.db()), ty_path.debug(self.db()));
                                return Some(LocalTermExpectationEffect {
                                    result: Err(todo!()),
                                    actions: vec![],
                                })
                            },
                            None => todo!(),
                        };
                        let destination_arguments = &destination_expansion.arguments(self.db());
                        if arguments.len() != destination_arguments.len() {
                            return Some(LocalTermExpectationEffect {
                                result: Err(todo!()),
                                actions: vec![],
                            });
                        };
                        // ad hoc
                        return Some(LocalTermExpectationEffect {
                            result: Err(todo!()),
                            actions: vec![],
                        });
                        let ty_path_variances = todo!();
                        let actions = std::iter::zip(
                            arguments.iter().copied(),
                            destination_arguments.iter().copied(),
                        )
                        .map(|(argument, destination)| todo!())
                        .collect();
                        Some(LocalTermExpectationEffect {
                            result: Err(todo!()),
                            actions,
                        })
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

impl ExpectImplicitlyConvertible {
    pub(super) fn resolve_implicitly_convertible(
        &self,
        db: &dyn ExprTypeDb,
        src: LocalTerm,
        level: LocalTermResolveLevel,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        if src == self.destination {
            return Some(LocalTermExpectationEffect {
                result: Ok(ExpectImplicitlyConvertibleOutcome {
                    implicit_conversion: ImplicitConversion::None,
                }
                .into()),
                actions: vec![],
            });
        }
        let src_patt = src.pattern(db, unresolved_terms);
        let dst_patt = self.destination.pattern(db, unresolved_terms);
        match dst_patt {
            LocalTermPattern::Literal(_) => todo!(),
            LocalTermPattern::TypeOntology {
                path: dst_path,
                arguments: dst_arguments,
            } => match src_patt {
                LocalTermPattern::TypeOntology {
                    path: Right(PreludeTypePath::NEVER),
                    ..
                } => Some(LocalTermExpectationEffect {
                    result: Ok(ExpectImplicitlyConvertibleOutcome {
                        implicit_conversion: ImplicitConversion::Never,
                    }
                    .into()),
                    actions: vec![],
                }),
                LocalTermPattern::TypeOntology {
                    path: src_path,
                    arguments: src_arguments,
                } if dst_path == src_path => todo!(),
                LocalTermPattern::TypeOntology {
                    path: src_path,
                    arguments: src_arguments,
                } => {
                    p!(dst_path.debug(db), src_path.debug(db));
                    todo!()
                }
                LocalTermPattern::ImplicitSymbol(_, src_implicit_symbol) => match level {
                    LocalTermResolveLevel::Weak => None,
                    LocalTermResolveLevel::Strong => Some(LocalTermExpectationEffect {
                        actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                            implicit_symbol: src_implicit_symbol,
                            substitution: self.destination,
                        }],
                        result: Ok(LocalTermExpectationOutcome::ImplicitlyConvertible(
                            ExpectImplicitlyConvertibleOutcome {
                                implicit_conversion: ImplicitConversion::None,
                            },
                        )),
                    }),
                },
                _ => {
                    p!(src.debug(db), self.destination.debug(db));
                    Some(LocalTermExpectationEffect {
                        result: Err(todo!()),
                        actions: vec![],
                    })
                }
            },
            LocalTermPattern::Curry {} => todo!(),
            LocalTermPattern::ImplicitSymbol(_, dst_implicit_symbol) => match level {
                LocalTermResolveLevel::Weak => None,
                LocalTermResolveLevel::Strong => Some(LocalTermExpectationEffect {
                    actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                        implicit_symbol: dst_implicit_symbol,
                        substitution: src,
                    }],
                    result: Ok(LocalTermExpectationOutcome::ImplicitlyConvertible(
                        ExpectImplicitlyConvertibleOutcome {
                            implicit_conversion: ImplicitConversion::None,
                        },
                    )),
                }),
            },
            LocalTermPattern::Category(_) => todo!(),
        }
    }
}
