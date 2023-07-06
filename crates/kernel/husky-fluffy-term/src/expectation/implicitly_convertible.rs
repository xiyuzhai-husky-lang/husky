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
    parameter_contracted_ty: FluffyTermRitchieParameter,
}

impl ExpectImplicitlyConvertible {
    #[inline(always)]
    pub fn new(parameter_contracted_ty: FluffyTermRitchieParameter) -> Self {
        Self {
            parameter_contracted_ty,
        }
    }

    #[inline(always)]
    pub fn new_const(ty: FluffyTerm) -> Self {
        Self {
            parameter_contracted_ty: FluffyTermRitchieParameter::new(Contract::Const, ty),
        }
    }

    /// this will reduce place type to type
    #[inline(always)]
    pub fn new_pure(engine: &impl FluffyTermEngine, ty: FluffyTerm) -> Self {
        let ty = match ty.data(engine) {
            FluffyTermData::PlaceTypeOntology {
                ty_path: path,
                arguments,
                ..
            } => match arguments.len() {
                0 => TermEntityPath::TypeOntology(path).into(),
                _ => todo!(),
            },
            _ => ty,
        };
        Self {
            parameter_contracted_ty: FluffyTermRitchieParameter::new(Contract::None, ty),
        }
    }

    #[inline(always)]
    pub fn new_pure_unit(engine: &impl FluffyTermEngine) -> Self {
        Self {
            parameter_contracted_ty: FluffyTermRitchieParameter::new(
                Contract::None,
                engine.term_menu().unit_ty_ontology().into(),
            ),
        }
    }

    #[inline(always)]
    pub fn new_pure_bool(engine: &impl FluffyTermEngine) -> Self {
        Self {
            parameter_contracted_ty: FluffyTermRitchieParameter::new(
                Contract::None,
                engine.term_menu().bool_ty_ontology().into(),
            ),
        }
    }

    #[inline(always)]
    pub fn new_move(ty: FluffyTerm) -> Self {
        Self {
            parameter_contracted_ty: FluffyTermRitchieParameter::new(Contract::Move, ty),
        }
    }

    pub(crate) fn try_substitute_unresolved_fluffy_term<'a>(
        &self,
        terms: &'a FluffyTerms,
    ) -> Result<Option<Expectation>, &'a HollowTermResolveError> {
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

    pub fn parameter_contracted_ty(&self) -> FluffyTermRitchieParameter {
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

    fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        meta: &mut ExpectationMeta,
        terms: &mut FluffyTerms,
    ) -> Option<ExpectationEffect> {
        match self.ty().data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ..
            } => self.resolve_convertible_to_ty_ontology(
                db,
                meta,
                terms,
                path,
                refined_path,
                arguments,
            ),
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Hole(_, hole) => {
                meta.set_holed(hole, |meta| HoleConstraint::ImplicitlyConvertibleFrom {
                    target: meta.expectee(),
                })
            }
            FluffyTermData::Category(_) => match self.contract() {
                Contract::None => todo!(),
                Contract::Move => todo!(),
                Contract::BorrowMut => todo!(),
                Contract::Const => {
                    if meta.expectee() == self.ty() {
                        return meta.set_ok(ImplicitConversion::Trivial, smallvec![]);
                    }
                    todo!()
                }
            },
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::PlaceTypeOntology {
                place,
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ..
            } => self.resolve_convertible_to_place_ty_ontology(
                db,
                meta,
                terms,
                place,
                path,
                refined_path,
                arguments,
            ),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => None, // adhoc
            // todo!(),
            FluffyTermData::Symbol { ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }
}

impl ExpectImplicitlyConvertible {
    fn resolve_convertible_to_ty_ontology(
        &self,
        db: &dyn FluffyTermDb,
        meta: &mut ExpectationMeta,
        terms: &FluffyTerms,
        dst_path: TypePath,
        dst_refined_path: Either<PreludeTypePath, CustomTypePath>,
        dst_argument_tys: &[FluffyTerm],
    ) -> Option<ExpectationEffect> {
        match meta.expectee().data_inner(db, terms) {
            FluffyTermData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::NEVER),
                ..
            } => meta.set_ok(ImplicitConversion::Never, smallvec![]),
            FluffyTermData::TypeOntology {
                refined_ty_path: src_path,
                arguments: src_argument_tys,
                ..
            } if dst_refined_path == src_path => {
                if dst_argument_tys.len() != src_argument_tys.len() {
                    p!(meta.expectee().debug(db), self.ty().debug(db));
                    todo!()
                }
                let mut actions = smallvec![];
                for (src_argument_ty, dst_argument_ty) in std::iter::zip(
                    src_argument_tys.iter().copied(),
                    dst_argument_tys.iter().copied(),
                ) {
                    if src_argument_ty != dst_argument_ty {
                        actions.push(FluffyTermResolveAction::AddExpectation {
                            src: meta.child_src(),
                            expectee: src_argument_ty,
                            expectation: ExpectSubtype::new(dst_argument_ty).into(),
                        })
                    }
                }
                match self.parameter_contracted_ty.contract() {
                    Contract::None => meta.set_ok(ImplicitConversion::Trivial, actions),
                    Contract::Move => meta.set_ok(ImplicitConversion::Trivial, actions),
                    Contract::BorrowMut => todo!(),
                    Contract::Const => todo!(),
                }
            }
            FluffyTermData::TypeOntology {
                ty_path: src_path,
                refined_ty_path: src_refined_path,
                arguments: src_arguments,
                ..
            } => meta.set_err(
                OriginalFluffyTermExpectationError::TypePathMismatch {
                    expected_path: dst_path,
                    expectee_path: src_path,
                },
                smallvec![],
            ),
            FluffyTermData::PlaceTypeOntology {
                place,
                refined_ty_path: src_path,
                arguments: src_argument_tys,
                ..
            } if dst_refined_path == src_path => {
                if dst_argument_tys.len() != src_argument_tys.len() {
                    p!(meta.expectee().debug(db), self.ty().debug(db));
                    todo!()
                }
                let mut actions = smallvec![];
                for (src_argument_ty, dst_argument_ty) in std::iter::zip(
                    src_argument_tys.iter().copied(),
                    dst_argument_tys.iter().copied(),
                ) {
                    if src_argument_ty != dst_argument_ty {
                        actions.push(FluffyTermResolveAction::AddExpectation {
                            src: meta.child_src(),
                            expectee: src_argument_ty,
                            expectation: ExpectSubtype::new(dst_argument_ty).into(),
                        })
                    }
                }
                match self.parameter_contracted_ty.contract() {
                    Contract::None => meta.set_ok(ImplicitConversion::Trivial, actions),
                    Contract::Move => meta.set_ok(ImplicitConversion::Trivial, actions),
                    Contract::BorrowMut => todo!(),
                    Contract::Const => todo!(),
                }
            }
            FluffyTermData::PlaceTypeOntology {
                ty_path: src_path,
                refined_ty_path: src_refined_path,
                arguments: src_arguments,
                ..
            } => {
                // todo: consider `Deref` and `DerefMut`
                meta.set_err(
                    OriginalFluffyTermExpectationError::TypePathMismatch {
                        expected_path: dst_path,
                        expectee_path: src_path,
                    },
                    smallvec![],
                )
            }
            FluffyTermData::Hole(_, hole) => {
                meta.set_holed(hole, |meta| HoleConstraint::ImplicitlyConvertibleTo {
                    target: meta.expectee(),
                })
            }
            _ => {
                p!(
                    meta.expectee().data_inner(db, terms).debug(db),
                    self.ty().data_inner(db, terms).debug(db)
                );
                // Some(FluffyTermExpectationEffect {
                //     result: Err(todo!()),
                //     actions: smallvec![],
                // })
                todo!()
            }
        }
    }

    fn resolve_convertible_to_place_ty_ontology(
        &self,
        db: &dyn FluffyTermDb,
        state: &mut ExpectationMeta,
        fluffy_terms: &FluffyTerms,
        place: Place,
        dst_path: TypePath,
        dst_refined_path: Either<PreludeTypePath, CustomTypePath>,
        dst_argument_tys: &[FluffyTerm],
    ) -> Option<ExpectationEffect> {
        todo!("this could be tricky")
    }
}
