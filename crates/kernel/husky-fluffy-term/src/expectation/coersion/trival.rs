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
        resolve_aux(
            state.expectee(),
            self.ty_expected,
            |_, _| Some(Coersion::Trivial(PlaceCoersion::Todo)),
            db,
            terms,
            state,
        )
    }

    fn resolve_trivial_old(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<ExpectationEffect> {
        let (expectee_place, expectee_base_ty_data) = state.expectee().ty_data_inner(db, terms);
        let (expected_place, expected_base_ty_data) = self.ty_expected.ty_data_inner(db, terms);
        if expectee_base_ty_data == expected_base_ty_data {
            // ad hoc
            // todo: contract
            return state.set_ok(Coersion::Trivial(PlaceCoersion::Todo), smallvec![]);
        }
        if let FluffyBaseTypeData::Hole(_, hole) = expectee_base_ty_data {
            return state.set_holed(hole, |meta| HoleConstraint::CoercibleInto {
                target: self.ty_expected(),
            });
        }
        match expected_base_ty_data {
            FluffyBaseTypeData::TypeOntology {
                ty_path: path,
                refined_ty_path,
                ty_arguments,
                ..
            } => self.resolve_trivial_dst_ty_ontology(
                db,
                state,
                terms,
                path,
                refined_ty_path,
                ty_arguments,
            ),
            FluffyBaseTypeData::Curry { .. } => todo!(),
            FluffyBaseTypeData::Hole(_, hole) => {
                state.set_holed(hole, |meta| HoleConstraint::CoercibleFrom {
                    target: meta.expectee(),
                })
            }
            FluffyBaseTypeData::Category(_) => match self.contract() {
                Contract::None => todo!(),
                Contract::Move => todo!(),
                Contract::Borrow => todo!(),
                Contract::BorrowMut => todo!(),
                Contract::Const => {
                    if state.expectee() == self.ty_expected() {
                        return state.set_ok(Coersion::Trivial(PlaceCoersion::Todo), smallvec![]);
                    }
                    todo!()
                }
                Contract::Leash => todo!(),
            },
            FluffyBaseTypeData::Ritchie { .. } => AltNone,
            FluffyBaseTypeData::Symbol { .. } => todo!(),
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
                    p!(state.expectee().debug(db), self.ty_expected().debug(db));
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
                    Contract::None => state.set_ok(Coersion::Trivial(PlaceCoersion::Todo), actions),
                    Contract::Move => state.set_ok(Coersion::Trivial(PlaceCoersion::Todo), actions),
                    Contract::Borrow => todo!(),
                    Contract::BorrowMut => todo!(),
                    Contract::Const => todo!(),
                    Contract::Leash => todo!(),
                }
            }
            FluffyTermData::TypeOntologyAtPlace {
                place,
                refined_ty_path: src_refined_ty_path,
                ty_arguments: src_argument_tys,
                ..
            } if dst_refined_ty_path == src_refined_ty_path => {
                if dst_ty_arguments.len() != src_argument_tys.len() {
                    p!(state.expectee().debug(db), self.ty_expected().debug(db));
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
                    Contract::None => state.set_ok(Coersion::Trivial(PlaceCoersion::Todo), actions),
                    Contract::Move => state.set_ok(Coersion::Trivial(PlaceCoersion::Todo), actions),
                    Contract::Borrow => todo!(),
                    Contract::BorrowMut => todo!(),
                    Contract::Const => todo!(),
                    Contract::Leash => todo!(),
                }
            }
            FluffyTermData::Hole(_, hole) => {
                state.set_holed(hole, |meta| HoleConstraint::CoercibleInto {
                    target: meta.expectee(),
                })
            }
            _ => AltNone,
        }
    }
}
