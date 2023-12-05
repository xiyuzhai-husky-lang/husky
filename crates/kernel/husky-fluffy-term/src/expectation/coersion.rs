//! coersion rules are
//!
mod deref;
mod holed;
mod never;
mod place_to_prelude_indirection;
mod trival;
mod wrap_in_some;

use self::deref::DerefCoersion;
use self::trival::PlaceCoersion;

use super::*;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Coersion {
    Trivial(PlaceCoersion),
    Never,
    Other,
    WrapInSome,
    PlaceToLeash,
    Deref(DerefCoersion),
}

/// expect a type that is implicitly convertible to type under contract
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct ExpectCoersion {
    contract: TermContract,
    ty_expected: FluffyTerm,
}

impl ExpectCoersion {
    #[inline(always)]
    pub fn new(contract: TermContract, ty_expected: FluffyTerm) -> Self {
        Self {
            contract,
            ty_expected,
        }
    }

    #[inline(always)]
    pub fn new_const(ty: FluffyTerm) -> Self {
        Self {
            contract: TermContract::Const,
            ty_expected: ty,
        }
    }

    /// this will reduce place type to type
    #[inline(always)]
    pub fn new_pure(engine: &impl FluffyTermEngine, ty: FluffyTerm) -> Self {
        let ty = match ty.data(engine) {
            // FluffyTermData::TypeOntologyAtPlace {
            //     ty_path,
            //     ty_arguments,
            //     ..
            // } => match ty_arguments.len() {
            //     0 => TermEntityPath::TypeOntology(ty_path).into(),
            //     // ad hoc
            //     _ => ty,
            // },
            _ => ty,
        };
        Self {
            contract: TermContract::None,
            ty_expected: ty,
        }
    }

    #[inline(always)]
    pub fn new_pure_unit(engine: &impl FluffyTermEngine) -> Self {
        Self {
            contract: TermContract::None,
            ty_expected: engine.term_menu().unit_ty_ontology().into(),
        }
    }

    #[inline(always)]
    pub fn new_pure_bool(engine: &impl FluffyTermEngine) -> Self {
        Self {
            contract: TermContract::None,
            ty_expected: engine.term_menu().bool_ty_ontology().into(),
        }
    }

    #[inline(always)]
    pub fn new_move(ty: FluffyTerm) -> Self {
        Self {
            contract: TermContract::Move,
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

    fn contract(self) -> TermContract {
        self.contract
    }

    fn ty_expected(self) -> FluffyTerm {
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
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FluffyTerms) -> FinalDestination {
        self.ty_expected().final_destination_inner(db, terms)
    }

    fn destination(&self) -> Option<FluffyTerm> {
        Some(self.ty_expected())
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        self.resolve_trivial(db, terms, state)?;
        self.resolve_holed(db, terms, state)?;
        self.resolve_never(db, terms, state)?;
        self.resolve_wrap_in_some(db, terms, state)?;
        self.resolve_deref(db, terms, state)?;
        self.resolve_place_to_prelude_indirection(db, terms, state)?;
        match state.resolve_progress() {
            ExpectationProgress::Intact => state.set_err(
                OriginalFluffyTermExpectationError::ExpectedCoersion {
                    expectee: state.expectee(),
                    contract: self.contract,
                    expected: self.ty_expected,
                },
                smallvec![],
            ),
            ExpectationProgress::Holed => AltNone,
            ExpectationProgress::Resolved(_) => unreachable!(),
        }
    }
}

// common auxiliary
fn try_finalize_coersion(
    src: FluffyTerm,
    dst: FluffyTerm,
    coersion: impl Into<Coersion>,
    db: &::salsa::Db,
    terms: &FluffyTerms,
    state: &mut ExpectationState,
) -> AltOption<FluffyTermEffect> {
    let src_base_ty_data = src.base_ty_data_inner(db, terms);
    let dst_base_ty_data = dst.base_ty_data_inner(db, terms);
    let coersion = coersion.into();
    if src_base_ty_data == dst_base_ty_data {
        return state.set_ok(coersion, smallvec![]);
    }
    match src_base_ty_data {
        FluffyBaseTypeData::TypeOntology {
            ty_path: src_ty_path,
            ty_arguments: src_ty_arguments,
            ..
        } => match dst_base_ty_data {
            FluffyBaseTypeData::TypeOntology {
                ty_path: dst_ty_path,
                refined_ty_path,
                ty_arguments: dst_ty_arguments,
                ty_ethereal_term,
            } if dst_ty_path == src_ty_path => {
                if dst_ty_arguments.len() != src_ty_arguments.len() {
                    // p!(state.expectee().debug(db), self.ty_expected().debug(db));
                    todo!()
                }
                let mut actions = smallvec![];
                for (src_argument_ty, dst_argument_ty) in std::iter::zip(
                    src_ty_arguments.iter().copied(),
                    dst_ty_arguments.iter().copied(),
                ) {
                    if src_argument_ty != dst_argument_ty {
                        // todo: check variance
                        actions.push(FluffyTermResolveAction::AddExpectation {
                            src: state.child_src(),
                            expectee: src_argument_ty,
                            expectation: ExpectSubtype::new(dst_argument_ty).into(),
                        })
                    }
                }
                state.set_ok(coersion, actions)
            }
            _ => AltNone,
        },
        FluffyBaseTypeData::Curry {
            curry_kind,
            variance,
            parameter_variable,
            parameter_ty,
            return_ty,
            ty_ethereal_term,
        } => todo!(),
        FluffyBaseTypeData::Hole(_, _) => AltNone,
        FluffyBaseTypeData::Category(_) => todo!(),
        FluffyBaseTypeData::Ritchie {
            ritchie_kind,
            parameter_contracted_tys,
            return_ty,
        } => todo!(),
        FluffyBaseTypeData::Symbol { term } => todo!(),
    }
}
