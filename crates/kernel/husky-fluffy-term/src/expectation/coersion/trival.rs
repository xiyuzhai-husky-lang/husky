use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlaceCoersion {
    Todo,
}

impl ExpectCoersion {
    pub(super) fn resolve_trivial(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<ExpectationEffect> {
        if self.ty_expected == state.expectee() {
            // ad hoc
            // todo: contract
            state.set_ok(Coersion::Place(PlaceCoersion::Todo), smallvec![])
        } else {
            match self.ty().data_inner(db, terms) {
                FluffyTermData::Literal(_) => todo!(),
                FluffyTermData::TypeOntology {
                    ty_path: path,
                    refined_ty_path,
                    arguments,
                    ..
                } => self.resolve_trivial_dst_ty_ontology(
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
                            return state.set_ok(Coersion::Place(PlaceCoersion::Todo), smallvec![]);
                        }
                        todo!()
                    }
                    Contract::Leash => todo!(),
                },
                FluffyTermData::Ritchie { .. } => AltNone,
                FluffyTermData::TypeOntologyAtPlace {
                    place,
                    ty_path: path,
                    refined_ty_path: refined_path,
                    ty_arguments: arguments,
                    ..
                } => todo!(),
                FluffyTermData::HoleAtPlace {
                    place,
                    hole_kind,
                    hole,
                } => AltNone, // adhoc
                // todo!(),
                FluffyTermData::Symbol { .. } => todo!(),
                FluffyTermData::SymbolAtPlace { .. } => todo!(),
                FluffyTermData::Variable { ty } => todo!(),
                FluffyTermData::TypeVariant { path } => todo!(),
            }
        }
    }

    fn resolve_trivial_dst_ty_ontology(
        &self,
        db: &dyn FluffyTermDb,
        state: &mut ExpectationState,
        fluffy_terms: &FluffyTerms,
        dst_path: TypePath,
        dst_refined_ty_path: Either<PreludeTypePath, CustomTypePath>,
        dst_ty_arguments: &[FluffyTerm],
    ) -> AltOption<ExpectationEffect> {
        match state.expectee().data_inner(db, fluffy_terms) {
            // for two path leading types with the same leading type path.
            // coersion is through subtyping of the arguments
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
                    Contract::None => state.set_ok(Coersion::Place(PlaceCoersion::Todo), actions),
                    Contract::Move => state.set_ok(Coersion::Place(PlaceCoersion::Todo), actions),
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
                OriginalFluffyTermExpectationError::TypePathMismatchForCoersion {
                    contract: self.contract,
                    ty_expected: self.ty_expected,
                    expectee: state.expectee(),
                    expected_path: dst_path,
                    expectee_path: src_path,
                },
                smallvec![],
            ),
            FluffyTermData::TypeOntologyAtPlace {
                place,
                refined_ty_path: src_refined_ty_path,
                ty_arguments: src_argument_tys,
                ..
            } if dst_refined_ty_path == src_refined_ty_path => {
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
                    Contract::None => state.set_ok(Coersion::Place(PlaceCoersion::Todo), actions),
                    Contract::Move => state.set_ok(Coersion::Place(PlaceCoersion::Todo), actions),
                    Contract::Borrow => todo!(),
                    Contract::BorrowMut => todo!(),
                    Contract::Const => todo!(),
                    Contract::Leash => todo!(),
                }
            }
            FluffyTermData::Hole(_, hole) => {
                state.set_holed(hole, |meta| HoleConstraint::CoercibleTo {
                    target: meta.expectee(),
                })
            }
            _ => AltNone,
        }
    }
}
