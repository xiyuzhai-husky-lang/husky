use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Coersion {
    Trivial,
    Never,
    Other,
    WrapInSome,
}

/// expect a type that is implicitly convertible to type under contract
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub struct ExpectCoersion {
    contract: Contract,
    ty_expected: FluffyTerm,
}

impl ExpectCoersion {
    #[inline(always)]
    pub fn new(contract: Contract, ty_expected: FluffyTerm) -> Self {
        Self {
            contract,
            ty_expected,
        }
    }

    #[inline(always)]
    pub fn new_const(ty: FluffyTerm) -> Self {
        Self {
            contract: Contract::Const,
            ty_expected: ty,
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
            ty_expected: ty,
        }
    }

    #[inline(always)]
    pub fn new_pure_unit(engine: &impl FluffyTermEngine) -> Self {
        Self {
            contract: Contract::None,
            ty_expected: engine.term_menu().unit_ty_ontology().into(),
        }
    }

    #[inline(always)]
    pub fn new_pure_bool(engine: &impl FluffyTermEngine) -> Self {
        Self {
            contract: Contract::None,
            ty_expected: engine.term_menu().bool_ty_ontology().into(),
        }
    }

    #[inline(always)]
    pub fn new_move(ty: FluffyTerm) -> Self {
        Self {
            contract: Contract::Move,
            ty_expected: ty,
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
        self.ty_expected
    }
}

impl ExpectFluffyTerm for ExpectCoersion {
    type Outcome = Coersion;

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
        if self.ty_expected == state.expectee() {
            // ad hoc
            // todo: contract
            state.set_ok(Coersion::Trivial, smallvec![])
        } else {
            match self.ty().data_inner(db, terms) {
                FluffyTermData::Literal(_) => todo!(),
                FluffyTermData::TypeOntology {
                    ty_path: path,
                    refined_ty_path,
                    arguments,
                    ..
                } => self.resolve_convertible_to_ty_ontology(
                    db,
                    state,
                    terms,
                    path,
                    refined_ty_path,
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
                    Contract::Borrow => todo!(),
                    Contract::BorrowMut => todo!(),
                    Contract::Const => {
                        if state.expectee() == self.ty() {
                            return state.set_ok(Coersion::Trivial, smallvec![]);
                        }
                        todo!()
                    }
                    Contract::Leash => todo!(),
                },
                FluffyTermData::Ritchie { .. } => state.set_err(
                    OriginalFluffyTermExpectationError::ExpectedCoersion {
                        expectee: state.expectee(),
                        expected: self.ty_expected,
                    },
                    smallvec![],
                ),
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
}

impl ExpectCoersion {
    fn resolve_convertible_to_ty_ontology(
        &self,
        db: &dyn FluffyTermDb,
        state: &mut ExpectationState,
        terms: &FluffyTerms,
        dst_path: TypePath,
        dst_refined_ty_path: Either<PreludeTypePath, CustomTypePath>,
        dst_ty_arguments: &[FluffyTerm],
    ) -> Option<ExpectationEffect> {
        if let Left(PreludeTypePath::Option) = dst_refined_ty_path && dst_ty_arguments[0] == state.expectee() {
            debug_assert_eq!(dst_ty_arguments.len() ,1);
            state.set_ok(Coersion::WrapInSome, smallvec![])
        } else {
            match state.expectee().data_inner(db, terms) {
                FluffyTermData::TypeOntology {
                    refined_ty_path: Left(PreludeTypePath::NEVER),
                    ..
                } => state.set_ok(Coersion::Never, smallvec![]),
                FluffyTermData::TypeOntology {
                    refined_ty_path: src_path,
                    arguments: src_argument_tys,
                    ..
                } if dst_refined_ty_path == src_path => {
                    if dst_ty_arguments.len() != src_argument_tys.len() {
                        p!(state.expectee().debug(db), self.ty().debug(db));
                        todo!()
                    }
                    let mut actions = smallvec![];
                    for (src_argument_ty, dst_argument_ty) in std::iter::zip(
                        src_argument_tys.iter().copied(),
                        dst_ty_arguments.iter().copied(),
                    ) {
                        if src_argument_ty != dst_argument_ty {
                            actions.push(FluffyTermResolveAction::AddExpectation {
                                src: state.child_src(),
                                expectee: src_argument_ty,
                                expectation: ExpectSubtype::new(dst_argument_ty).into(),
                            })
                        }
                    }
                    match self.contract() {
                        Contract::None => state.set_ok(Coersion::Trivial, actions),
                        Contract::Move => state.set_ok(Coersion::Trivial, actions),
                        Contract::Borrow => todo!(),
                        Contract::BorrowMut => todo!(),
                        Contract::Const => todo!(),
                        Contract::Leash => todo!(),
                    }
                }
                FluffyTermData::TypeOntology {
                    ty_path: src_path,
                    refined_ty_path: src_refined_path,
                    arguments: src_arguments,
                    ..
                } => state.set_err(
                    OriginalFluffyTermExpectationError::TypePathMismatchForCoersion  {
                        contract:self.contract,
                        ty_expected: self.ty_expected,
                        expectee:state.expectee(),
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
                } if dst_refined_ty_path == src_path => {
                    if dst_ty_arguments.len() != src_argument_tys.len() {
                        p!(state.expectee().debug(db), self.ty().debug(db));
                        todo!()
                    }
                    let mut actions = smallvec![];
                    for (src_argument_ty, dst_argument_ty) in std::iter::zip(
                        src_argument_tys.iter().copied(),
                        dst_ty_arguments.iter().copied(),
                    ) {
                        if src_argument_ty != dst_argument_ty {
                            actions.push(FluffyTermResolveAction::AddExpectation {
                                src: state.child_src(),
                                expectee: src_argument_ty,
                                expectation: ExpectSubtype::new(dst_argument_ty).into(),
                            })
                        }
                    }
                    match self.contract() {
                        Contract::None => state.set_ok(Coersion::Trivial, actions),
                        Contract::Move => state.set_ok(Coersion::Trivial, actions),
                        Contract::Borrow => todo!(),
                        Contract::BorrowMut => todo!(),
                        Contract::Const => todo!(),
                        Contract::Leash => todo!(),
                    }
                }
                FluffyTermData::TypeOntologyAtPlace {
                    ty_path: src_path,
                    refined_ty_path: src_refined_path,
                    arguments: src_arguments,
                    ..
                } => {
                    // todo: consider `Deref` and `DerefMut`
                    state.set_err(
                        OriginalFluffyTermExpectationError::TypePathMismatchForCoersion {
                            contract: todo!(),
                            ty_expected: todo!(),
                            expectee: todo!(),
                            expected_path: dst_path,
                            expectee_path: src_path,
                        },
                        smallvec![],
                    )
                }
                FluffyTermData::Hole(_, hole) => {
                    state.set_holed(hole, |meta| HoleConstraint::CoercibleTo {
                        target: meta.expectee(),
                    })
                }
                _ => {
                    p!(
                        state.expectee().data_inner(db, terms).debug(db),
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
