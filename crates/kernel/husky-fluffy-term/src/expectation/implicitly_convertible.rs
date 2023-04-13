use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitConversion {
    None,
    Never,
    Other,
}

/// expect a type that is implicitly convertible to type under contract
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectImplicitlyConvertible {
    pub(crate) parameter_contracted_ty: FluffyTermRitchieParameterContractedType,
}

impl ExpectImplicitlyConvertible {
    #[inline(always)]
    pub fn new(parameter_contracted_ty: FluffyTermRitchieParameterContractedType) -> Self {
        Self {
            parameter_contracted_ty,
        }
    }

    #[inline(always)]
    pub fn new_const(ty: FluffyTerm) -> Self {
        Self {
            parameter_contracted_ty: FluffyTermRitchieParameterContractedType::new(
                Contract::Const,
                ty,
            ),
        }
    }

    #[inline(always)]
    pub fn new_pure(ty: FluffyTerm) -> Self {
        Self {
            parameter_contracted_ty: FluffyTermRitchieParameterContractedType::new(
                Contract::Pure,
                ty,
            ),
        }
    }

    #[inline(always)]
    pub fn new_move(ty: FluffyTerm) -> Self {
        Self {
            parameter_contracted_ty: FluffyTermRitchieParameterContractedType::new(
                Contract::Move,
                ty,
            ),
        }
    }

    pub(crate) fn try_substitute_unresolved_fluffy_term<'a>(
        &self,
        terms: &'a FluffyTerms,
    ) -> Result<Option<ExpectationData>, &'a HollowTermResolveError> {
        todo!()
        // match terms.try_reduce_fluffy_term(self.expected)? {
        //     Some(destination) => Ok(Some(
        //         ExpectImplicitlyConvertible {
        //             expected: destination,
        //         }
        //         .into(),
        //     )),
        //     None => Ok(None),
        // }
    }

    fn contract(self) -> Contract {
        self.parameter_contracted_ty.contract()
    }

    fn ty(self) -> FluffyTerm {
        self.parameter_contracted_ty.ty()
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
        self.parameter_contracted_ty
            .ty()
            .final_destination_inner(db, terms)
    }

    fn destination(&self) -> Option<FluffyTerm> {
        Some(self.parameter_contracted_ty.ty())
    }
}

impl ExpectImplicitlyConvertible {
    pub(super) fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        parent: FluffyTermExpectationIdx,
        expectee: FluffyTerm,
        level: FluffyTermResolveLevel,
    ) -> Option<FluffyTermExpectationEffect> {
        // if expectee == self.expected {
        //     return Some(FluffyTermExpectationEffect {
        //         result: Ok(ImplicitConversion::None.into()),
        //         actions: smallvec![],
        //     });
        // }
        let src_patt = expectee.data_inner(db, terms);
        match self.ty().data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                argument_tys,
            } => self.resolve_ty_ontology(
                db,
                terms,
                expectee,
                parent,
                level,
                src_patt,
                path,
                refined_path,
                argument_tys,
            ),
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Hole(_, hole) => match level {
                FluffyTermResolveLevel::Weak => None,
                FluffyTermResolveLevel::Strong => Some(FluffyTermExpectationEffect {
                    actions: smallvec![FluffyTermResolveAction::SubstituteHole {
                        hole,
                        substitution: expectee,
                    }],
                    result: Ok(ImplicitConversion::None.into()),
                }),
            },
            FluffyTermData::Category(_) => match self.contract() {
                Contract::Pure => todo!(),
                Contract::Move => todo!(),
                Contract::BorrowMut => todo!(),
                Contract::Const => {
                    if expectee == self.ty() {
                        return Some(FluffyTermExpectationEffect {
                            result: Ok(ImplicitConversion::None.into()),
                            actions: smallvec![],
                        });
                    }
                    todo!()
                }
            },
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                argument_tys,
            } => self.resolve_place_ty_ontology(
                db,
                terms,
                expectee,
                parent,
                level,
                src_patt,
                place,
                path,
                refined_path,
                argument_tys,
            ),
        }
    }

    fn resolve_ty_ontology(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
        expectee: FluffyTerm,
        parent: ArenaIdx<ExpectationEntry>,
        level: FluffyTermResolveLevel,
        src_patt: FluffyTermData,
        dst_path: TypePath,
        dst_refined_path: Either<CustomTypePath, PreludeTypePath>,
        dst_argument_tys: &[FluffyTerm],
    ) -> Option<FluffyTermExpectationEffect> {
        todo!("consider contract");
        match src_patt {
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
                    p!(expectee.debug(db), self.ty().debug(db));
                    todo!()
                }
                let mut actions = smallvec![];
                for (src_argument_ty, dst_argument_ty) in std::iter::zip(
                    src_argument_tys.iter().copied(),
                    dst_argument_tys.iter().copied(),
                ) {
                    if src_argument_ty != dst_argument_ty {
                        actions.push(FluffyTermResolveAction::AddExpectation {
                            src: ExpectationSource::ExpectationResolve { parent },
                            expectee: src_argument_ty,
                            expectation: ExpectSubtype::new(dst_argument_ty).into(),
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
                    expected_path: dst_path,
                    expectee_path: src_path,
                }
                .into()),
                actions: smallvec![],
            }),
            FluffyTermData::Hole(_, _) => {
                todo!()
                //     match level {
                //     FluffyTermResolveLevel::Weak => None,
                //     FluffyTermResolveLevel::Strong => Some(FluffyTermExpectationEffect {
                //         result: Ok(FluffyTermExpectationOutcome::ImplicitlyConvertible(
                //             ImplicitConversion::None,
                //         )),
                //         actions: smallvec![FluffyTermResolveAction::SubstituteHole {
                //             hole,
                //             substitution: self.expected,
                //         }],
                //     }),
                // }
            }
            _ => {
                p!(expectee.debug(db), self.ty().debug(db));
                Some(FluffyTermExpectationEffect {
                    result: Err(todo!()),
                    actions: smallvec![],
                })
            }
        }
    }

    fn resolve_place_ty_ontology(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
        expectee: FluffyTerm,
        parent: ArenaIdx<ExpectationEntry>,
        level: FluffyTermResolveLevel,
        src_patt: FluffyTermData,
        place: Place,
        dst_path: TypePath,
        dst_refined_path: Either<CustomTypePath, PreludeTypePath>,
        dst_argument_tys: &[FluffyTerm],
    ) -> Option<FluffyTermExpectationEffect> {
        match src_patt {
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
                    p!(expectee.debug(db), self.ty().debug(db));
                    todo!()
                }
                let mut actions = smallvec![];
                for (src_argument_ty, dst_argument_ty) in std::iter::zip(
                    src_argument_tys.iter().copied(),
                    dst_argument_tys.iter().copied(),
                ) {
                    if src_argument_ty != dst_argument_ty {
                        actions.push(FluffyTermResolveAction::AddExpectation {
                            src: ExpectationSource::ExpectationResolve { parent },
                            expectee: src_argument_ty,
                            expectation: ExpectSubtype::new(dst_argument_ty).into(),
                        })
                    }
                }
                let result = match self.parameter_contracted_ty.contract() {
                    Contract::Pure => match place {
                        Place::Const => todo!(),
                        Place::StackPure { location } => todo!(),
                        Place::ImmutableStackOwned { location } => todo!(),
                        Place::MutableStackOwned { location } => todo!(),
                        Place::Transient => todo!(),
                        Place::Ref { guard } => todo!(),
                        Place::RefMut { guard } => todo!(),
                        Place::Leashed => todo!(),
                        Place::Todo => todo!(),
                    },
                    Contract::Move => todo!(),
                    Contract::BorrowMut => todo!(),
                    Contract::Const => todo!(),
                };
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
                    expected_path: dst_path,
                    expectee_path: src_path,
                }
                .into()),
                actions: smallvec![],
            }),
            FluffyTermData::Hole(_, _) => {
                todo!()
                //     match level {
                //     FluffyTermResolveLevel::Weak => None,
                //     FluffyTermResolveLevel::Strong => Some(FluffyTermExpectationEffect {
                //         result: Ok(FluffyTermExpectationOutcome::ImplicitlyConvertible(
                //             ImplicitConversion::None,
                //         )),
                //         actions: smallvec![FluffyTermResolveAction::SubstituteHole {
                //             hole,
                //             substitution: self.expected,
                //         }],
                //     }),
                // }
            }
            _ => {
                p!(expectee.debug(db), self.ty().debug(db));
                Some(FluffyTermExpectationEffect {
                    result: Err(todo!()),
                    actions: smallvec![],
                })
            }
        }
    }
}
