use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitConversion {
    None,
    Never,
    Other,
}

/// expect a type that is implicitly convertible to dst
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub struct ExpectImplicitlyConvertible {
    pub(crate) destination: LocalTerm,
}

impl ExpectImplicitlyConvertible {
    // todo: redo, take care!
    #[inline(always)]
    pub fn new(parameter_liasoned_ty: LocalTermRitchieParameterLiasonedType) -> Self {
        Self {
            destination: parameter_liasoned_ty.ty(),
        }
    }

    #[inline(always)]
    pub fn new_transient(ty: LocalTerm) -> Self {
        Self { destination: ty }
    }

    #[inline(always)]
    pub fn new_ad_hoc(ty: LocalTerm) -> Self {
        Self { destination: ty }
    }

    pub(crate) fn try_substitute_unresolved_local_term<'a>(
        &self,
        unresolved_terms: &'a UnresolvedTerms,
    ) -> Result<Option<LocalTermExpectation>, &'a LocalTermResolveError> {
        match unresolved_terms.try_reduce_local_term(self.destination)? {
            Some(destination) => Ok(Some(ExpectImplicitlyConvertible { destination }.into())),
            None => Ok(None),
        }
    }
}

impl ExpectLocalTerm for ExpectImplicitlyConvertible {
    type Outcome = ImplicitConversion;

    fn retrieve_outcome(outcome: &LocalTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            LocalTermExpectationOutcome::ImplicitlyConvertible(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn TermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        self.destination.final_destination(db, unresolved_terms)
    }

    fn destination(&self) -> Option<LocalTerm> {
        Some(self.destination)
    }
}

impl ExpectImplicitlyConvertible {
    pub(super) fn resolve(
        &self,
        db: &dyn TermDb,
        src_expr_idx: ExprIdx,
        src: LocalTerm,
        level: LocalTermResolveLevel,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        if src == self.destination {
            return Some(LocalTermExpectationEffect {
                result: Ok(ImplicitConversion::None.into()),
                actions: smallvec![],
            });
        }
        let src_patt = src.pattern_inner(db, unresolved_terms);
        let dst_patt = self.destination.pattern_inner(db, unresolved_terms);
        match dst_patt {
            LocalTermPattern::Literal(_) => todo!(),
            LocalTermPattern::TypeOntology {
                path: dst_path,
                refined_path: dst_refined_path,
                argument_tys: dst_argument_tys,
                ..
            } => match src_patt {
                LocalTermPattern::TypeOntology {
                    refined_path: Right(PreludeTypePath::NEVER),
                    ..
                } => Some(LocalTermExpectationEffect {
                    result: Ok(ImplicitConversion::Never.into()),
                    actions: smallvec![],
                }),
                LocalTermPattern::TypeOntology {
                    refined_path: src_path,
                    argument_tys: src_argument_tys,
                    ..
                } if dst_refined_path == src_path => {
                    if dst_argument_tys.len() != src_argument_tys.len() {
                        p!(src.debug(db), self.destination.debug(db));
                        todo!()
                    }
                    let mut actions = smallvec![];
                    for (src_argument_ty, dst_argument_ty) in
                        std::iter::zip(src_argument_tys.into_iter(), dst_argument_tys.into_iter())
                    {
                        if src_argument_ty != dst_argument_ty {
                            actions.push(TermResolveAction::AddExpectation {
                                src_expr_idx,
                                expectee: src_argument_ty,
                                expectation: ExpectSubtype::new(dst_argument_ty).into(),
                            })
                        }
                    }
                    Some(LocalTermExpectationEffect {
                        result: Ok(ImplicitConversion::None.into()),
                        actions,
                    })
                }
                LocalTermPattern::TypeOntology {
                    path: src_path,
                    refined_path: src_refined_path,
                    argument_tys: src_arguments,
                    ..
                } => Some(LocalTermExpectationEffect {
                    result: Err(OriginalLocalTermExpectationError::TypePathMismatch {
                        expected_path: dst_path,
                        expectee_path: src_path,
                    }
                    .into()),
                    actions: smallvec![],
                }),
                LocalTermPattern::ImplicitSymbol(_, src_implicit_symbol) => match level {
                    LocalTermResolveLevel::Weak => None,
                    LocalTermResolveLevel::Strong => Some(LocalTermExpectationEffect {
                        result: Ok(LocalTermExpectationOutcome::ImplicitlyConvertible(
                            ImplicitConversion::None,
                        )),
                        actions: smallvec![TermResolveAction::SubstituteImplicitSymbol {
                            implicit_symbol: src_implicit_symbol,
                            substitution: self.destination,
                        }],
                    }),
                },
                _ => {
                    p!(src.debug(db), self.destination.debug(db));
                    Some(LocalTermExpectationEffect {
                        result: Err(todo!()),
                        actions: smallvec![],
                    })
                }
            },
            LocalTermPattern::Curry { .. } => todo!(),
            LocalTermPattern::ImplicitSymbol(_, dst_implicit_symbol) => match level {
                LocalTermResolveLevel::Weak => None,
                LocalTermResolveLevel::Strong => Some(LocalTermExpectationEffect {
                    actions: smallvec![TermResolveAction::SubstituteImplicitSymbol {
                        implicit_symbol: dst_implicit_symbol,
                        substitution: src,
                    }],
                    result: Ok(ImplicitConversion::None.into()),
                }),
            },
            LocalTermPattern::Category(_) => todo!(),
            LocalTermPattern::Ritchie { .. } => todo!(),
        }
    }
}
