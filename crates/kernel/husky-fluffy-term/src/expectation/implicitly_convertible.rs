use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitConversion {
    Trivial,
    Never,
    Other,
}

/// expect a type that is implicitly convertible to type under contract
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectImplicitlyConvertible {
    parameter_contracted_ty: FluffyTermRitchieParameterContractedType,
}

impl ExpectImplicitlyConvertible {
    #[inline(always)]
    pub fn new(parameter_contracted_ty: FluffyTermRitchieParameterContractedType) -> Self {
        Self {
            parameter_contracted_ty,
        }
    }

    #[inline(always)]
    pub fn new_const(engine: &impl FluffyTermEngine, ty: FluffyTerm) -> Self {
        #[cfg(test)]
        match ty.data(engine) {
            FluffyTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                arguments: argument_tys,
            } => unreachable!(),
            _ => (),
        }
        Self {
            parameter_contracted_ty: FluffyTermRitchieParameterContractedType::new(
                Contract::Const,
                ty,
            ),
        }
    }

    /// this will reduce place type to type
    #[inline(always)]
    pub fn new_pure(engine: &impl FluffyTermEngine, ty: FluffyTerm) -> Self {
        let ty = match ty.data(engine) {
            FluffyTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                arguments: argument_tys,
            } => match argument_tys.len() {
                0 => TermEntityPath::TypeOntology(path).into(),
                _ => todo!(),
            },
            _ => ty,
        };
        Self {
            parameter_contracted_ty: FluffyTermRitchieParameterContractedType::new(
                Contract::Pure,
                ty,
            ),
        }
    }

    #[inline(always)]
    pub fn new_pure_unit(engine: &impl FluffyTermEngine) -> Self {
        Self {
            parameter_contracted_ty: FluffyTermRitchieParameterContractedType::new(
                Contract::Pure,
                engine.term_menu().unit_ty_ontology().into(),
            ),
        }
    }

    #[inline(always)]
    pub fn new_pure_bool(engine: &impl FluffyTermEngine) -> Self {
        Self {
            parameter_contracted_ty: FluffyTermRitchieParameterContractedType::new(
                Contract::Pure,
                engine.term_menu().bool_ty_ontology().into(),
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

    pub fn parameter_contracted_ty(&self) -> FluffyTermRitchieParameterContractedType {
        self.parameter_contracted_ty
    }
}

impl ExpectFluffyTerm for ExpectImplicitlyConvertible {
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
        match self.ty().data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                arguments: argument_tys,
            } => self.resolve_convertible_to_ty_ontology(
                db,
                terms,
                parent,
                level,
                expectee,
                path,
                refined_path,
                argument_tys,
            ),
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Hole(_, hole) => match level {
                FluffyTermResolveLevel::Weak => None,
                FluffyTermResolveLevel::Strong => Some(FluffyTermExpectationEffect {
                    actions: smallvec![FluffyTermResolveAction::FillHole {
                        hole,
                        term: expectee,
                    }],
                    result: Ok(ImplicitConversion::Trivial.into()),
                }),
            },
            FluffyTermData::Category(_) => match self.contract() {
                Contract::Pure => todo!(),
                Contract::Move => todo!(),
                Contract::BorrowMut => todo!(),
                Contract::Const => {
                    if expectee == self.ty() {
                        return Some(FluffyTermExpectationEffect {
                            result: Ok(ImplicitConversion::Trivial.into()),
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
                arguments: argument_tys,
            } => self.resolve_convertible_to_place_ty_ontology(
                db,
                terms,
                parent,
                level,
                expectee,
                place,
                path,
                refined_path,
                argument_tys,
            ),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }

    fn resolve_convertible_to_ty_ontology(
        &self,
        db: &dyn FluffyTermDb,
        fluffy_terms: &FluffyTerms,
        parent: ArenaIdx<ExpectationEntry>,
        level: FluffyTermResolveLevel,
        expectee: FluffyTerm,
        dst_path: TypePath,
        dst_refined_path: Either<CustomTypePath, PreludeTypePath>,
        dst_argument_tys: &[FluffyTerm],
    ) -> Option<FluffyTermExpectationEffect> {
        match expectee.data_inner(db, fluffy_terms) {
            FluffyTermData::TypeOntology {
                refined_path: Right(PreludeTypePath::NEVER),
                ..
            } => Some(FluffyTermExpectationEffect {
                result: Ok(ImplicitConversion::Never.into()),
                actions: smallvec![],
            }),
            FluffyTermData::TypeOntology {
                refined_path: src_path,
                arguments: src_argument_tys,
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
                    Contract::Pure => Ok(ImplicitConversion::Trivial.into()),
                    Contract::Move => Ok(ImplicitConversion::Trivial.into()),
                    Contract::BorrowMut => todo!(),
                    Contract::Const => todo!(),
                };
                Some(FluffyTermExpectationEffect { result, actions })
            }
            FluffyTermData::TypeOntology {
                path: src_path,
                refined_path: src_refined_path,
                arguments: src_arguments,
                ..
            } => Some(FluffyTermExpectationEffect {
                result: Err(OriginalFluffyTermExpectationError::TypePathMismatch {
                    expected_path: dst_path,
                    expectee_path: src_path,
                }
                .into()),
                actions: smallvec![],
            }),
            FluffyTermData::PlaceTypeOntology {
                place,
                refined_path: src_path,
                arguments: src_argument_tys,
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
                    Contract::Pure => Ok(ImplicitConversion::Trivial.into()),
                    Contract::Move => todo!(),
                    Contract::BorrowMut => todo!(),
                    Contract::Const => todo!(),
                };
                Some(FluffyTermExpectationEffect { result, actions })
            }
            FluffyTermData::PlaceTypeOntology {
                path: src_path,
                refined_path: src_refined_path,
                arguments: src_arguments,
                ..
            } => {
                // todo: consider `Deref` and `DerefMut`
                Some(FluffyTermExpectationEffect {
                    result: Err(OriginalFluffyTermExpectationError::TypePathMismatch {
                        expected_path: dst_path,
                        expectee_path: src_path,
                    }
                    .into()),
                    actions: smallvec![],
                })
            }
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
                p!(
                    expectee.data_inner(db, fluffy_terms).debug(db),
                    self.ty().data_inner(db, fluffy_terms).debug(db)
                );
                Some(FluffyTermExpectationEffect {
                    result: Err(todo!()),
                    actions: smallvec![],
                })
            }
        }
    }

    fn resolve_convertible_to_place_ty_ontology(
        &self,
        db: &dyn FluffyTermDb,
        fluffy_terms: &FluffyTerms,
        parent: ArenaIdx<ExpectationEntry>,
        level: FluffyTermResolveLevel,
        expectee: FluffyTerm,
        place: Place,
        dst_path: TypePath,
        dst_refined_path: Either<CustomTypePath, PreludeTypePath>,
        dst_argument_tys: &[FluffyTerm],
    ) -> Option<FluffyTermExpectationEffect> {
        todo!("this could be tricky")
    }
}
