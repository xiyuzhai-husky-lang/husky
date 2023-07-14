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
pub struct ExpectCoersion {
    contract: Contract,
    ty: FluffyTerm,
}

impl ExpectCoersion {
    #[inline(always)]
    pub fn new(contract: Contract, ty: FluffyTerm) -> Self {
        Self { contract, ty }
    }

    #[inline(always)]
    pub fn new_const(ty: FluffyTerm) -> Self {
        Self {
            contract: Contract::Const,
            ty,
        }
    }

    /// this will reduce place type to type
    #[inline(always)]
    pub fn new_pure(engine: &impl FluffyTermEngine, ty: FluffyTerm) -> Self {
        let ty = match ty.data(engine) {
            FluffyTermData::TypeOntologyAtPlace {
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
            contract: Contract::None,
            ty,
        }
    }

    #[inline(always)]
    pub fn new_pure_unit(engine: &impl FluffyTermEngine) -> Self {
        Self {
            contract: Contract::None,
            ty: engine.term_menu().unit_ty_ontology().into(),
        }
    }

    #[inline(always)]
    pub fn new_pure_bool(engine: &impl FluffyTermEngine) -> Self {
        Self {
            contract: Contract::None,
            ty: engine.term_menu().bool_ty_ontology().into(),
        }
    }

    #[inline(always)]
    pub fn new_move(ty: FluffyTerm) -> Self {
        Self {
            contract: Contract::Move,
            ty,
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
        self.contract
    }

    fn ty(self) -> FluffyTerm {
        self.ty
    }
}

impl ExpectFluffyTerm for ExpectCoersion {
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
        self.ty().final_destination_inner(db, terms)
    }

    fn destination(&self) -> Option<FluffyTerm> {
        Some(self.ty())
    }

    fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
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
                state,
                terms,
                path,
                refined_path,
                arguments,
            ),
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Hole(_, hole) => {
                state.set_holed(hole, |meta| HoleConstraint::CoercibleFrom {
                    target: meta.expectee(),
                })
            }
            FluffyTermData::Category(_) => match self.contract() {
                Contract::None => todo!(),
                Contract::Move => todo!(),
                Contract::BorrowMut => todo!(),
                Contract::Const => {
                    if state.expectee() == self.ty() {
                        return state.set_ok(ImplicitConversion::Trivial, smallvec![]);
                    }
                    todo!()
                }
            },
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::TypeOntologyAtPlace {
                place,
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ..
            } => self.resolve_convertible_to_place_ty_ontology(
                db,
                state,
                terms,
                place,
                path,
                refined_path,
                arguments,
            ),
            FluffyTermData::HoleAtPlace {
                place,
                hole_kind,
                hole,
            } => None, // adhoc
            // todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::SymbolAtPlace { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}

impl ExpectCoersion {
    fn resolve_convertible_to_ty_ontology(
        &self,
        db: &dyn FluffyTermDb,
        meta: &mut ExpectationState,
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
                match self.contract() {
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
            FluffyTermData::TypeOntologyAtPlace {
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
                match self.contract() {
                    Contract::None => meta.set_ok(ImplicitConversion::Trivial, actions),
                    Contract::Move => meta.set_ok(ImplicitConversion::Trivial, actions),
                    Contract::BorrowMut => todo!(),
                    Contract::Const => todo!(),
                }
            }
            FluffyTermData::TypeOntologyAtPlace {
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
                meta.set_holed(hole, |meta| HoleConstraint::CoercibleTo {
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
        state: &mut ExpectationState,
        fluffy_terms: &FluffyTerms,
        place: Place,
        dst_path: TypePath,
        dst_refined_path: Either<PreludeTypePath, CustomTypePath>,
        dst_argument_tys: &[FluffyTerm],
    ) -> Option<ExpectationEffect> {
        todo!("this could be tricky")
    }
}
