use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitConversion {
    None,
    Never,
    Other,
}

/// expect a type that is implicitly convertible to dst
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectImplicitlyConvertible {
    pub(crate) expected: FluffyTerm,
}

impl ExpectImplicitlyConvertible {
    // todo: redo, take care!
    #[inline(always)]
    pub fn new(parameter_contracted_ty: FluffyTermRitchieParameterContractedType) -> Self {
        Self {
            expected: parameter_contracted_ty.ty(),
        }
    }

    #[inline(always)]
    pub fn new_transient(ty: FluffyTerm) -> Self {
        Self { expected: ty }
    }

    #[inline(always)]
    pub fn new_ad_hoc(ty: FluffyTerm) -> Self {
        Self { expected: ty }
    }

    pub(crate) fn try_substitute_unresolved_local_term<'a>(
        &self,
        porous_terms: &'a FluffyTerms,
    ) -> Result<Option<FluffyTermExpectation>, &'a FluffyTermResolveError> {
        match porous_terms.try_reduce_local_term(self.expected)? {
            Some(destination) => Ok(Some(
                ExpectImplicitlyConvertible {
                    expected: destination,
                }
                .into(),
            )),
            None => Ok(None),
        }
    }
}

impl ExpectLocalTerm for ExpectImplicitlyConvertible {
    type Outcome = ImplicitConversion;

    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            FluffyTermExpectationOutcome::ImplicitlyConvertible(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        self.expected.final_destination_inner(db, region)
    }

    fn destination(&self) -> Option<FluffyTerm> {
        Some(self.expected)
    }
}

impl ExpectImplicitlyConvertible {
    pub(super) fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        src: HollowTermSource,
        expectee: FluffyTerm,
        level: FluffyTermResolveLevel,
    ) -> Option<FluffyTermExpectationEffect> {
        if expectee == self.expected {
            return Some(FluffyTermExpectationEffect {
                result: Ok(ImplicitConversion::None.into()),
                actions: smallvec![],
            });
        }
        let src_patt = expectee.data_inner(db, terms);
        let dst_patt = self.expected.data_inner(db, terms);
        match dst_patt {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path: dst_path,
                refined_path: dst_refined_path,
                argument_tys: dst_argument_tys,
                ..
            } => match src_patt {
                FluffyTermData::TypeOntology {
                    refined_path: Right(PreludeTypePath::NEVER),
                    ..
                } => Some(FluffyTermExpectationEffect {
                    result: Ok(ImplicitConversion::Never.into()),
                    actions: smallvec![],
                }),
                FluffyTermData::TypeOntology {
                    refined_path: src_path,
                    argument_tys: src_argument_tys,
                    ..
                } if dst_refined_path == src_path => {
                    if dst_argument_tys.len() != src_argument_tys.len() {
                        p!(expectee.debug(db), self.expected.debug(db));
                        todo!()
                    }
                    let mut actions = smallvec![];
                    for (src_argument_ty, dst_argument_ty) in
                        std::iter::zip(src_argument_tys.into_iter(), dst_argument_tys.into_iter())
                    {
                        if src_argument_ty != dst_argument_ty {
                            actions.push(FluffyTermResolveAction::AddExpectation {
                                src,
                                expectee: *src_argument_ty,
                                expectation: ExpectSubtype::new(*dst_argument_ty).into(),
                            })
                        }
                    }
                    Some(FluffyTermExpectationEffect {
                        result: Ok(ImplicitConversion::None.into()),
                        actions,
                    })
                }
                FluffyTermData::TypeOntology {
                    path: src_path,
                    refined_path: src_refined_path,
                    argument_tys: src_arguments,
                    ..
                } => Some(FluffyTermExpectationEffect {
                    result: Err(OriginalFluffyTermExpectationError::TypePathMismatch {
                        expected_path: *dst_path,
                        expectee_path: *src_path,
                    }
                    .into()),
                    actions: smallvec![],
                }),
                FluffyTermData::Hole(_, src_implicit_symbol) => match level {
                    FluffyTermResolveLevel::Weak => None,
                    FluffyTermResolveLevel::Strong => Some(FluffyTermExpectationEffect {
                        result: Ok(FluffyTermExpectationOutcome::ImplicitlyConvertible(
                            ImplicitConversion::None,
                        )),
                        actions: smallvec![FluffyTermResolveAction::SubstituteHole {
                            hole: *src_implicit_symbol,
                            substitution: self.expected,
                        }],
                    }),
                },
                _ => {
                    p!(src.debug(db), self.expected.debug(db));
                    Some(FluffyTermExpectationEffect {
                        result: Err(todo!()),
                        actions: smallvec![],
                    })
                }
            },
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Hole(_, dst_implicit_symbol) => match level {
                FluffyTermResolveLevel::Weak => None,
                FluffyTermResolveLevel::Strong => Some(FluffyTermExpectationEffect {
                    actions: smallvec![FluffyTermResolveAction::SubstituteHole {
                        hole: dst_implicit_symbol,
                        substitution: src,
                    }],
                    result: Ok(ImplicitConversion::None.into()),
                }),
            },
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
        }
    }
}
