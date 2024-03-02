//! coersion rules are
//!
pub mod deref;
pub mod holed;
pub mod never;
pub mod reref;
pub mod trival;
pub mod wrap_in_some;

use self::deref::DerefFlyCoersion;
use self::trival::TrivialFlyCoersion;

use super::*;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FlyCoersion {
    Trivial(TrivialFlyCoersion),
    Never,
    WrapInSome,
    PlaceToLeash,
    Deref(DerefFlyCoersion),
}

impl FlyCoersion {
    pub fn place_after_coersion(self) -> FlyQuary {
        match self {
            FlyCoersion::Trivial(slf) => slf.place_after_coersion(),
            FlyCoersion::Deref(slf) => slf.place_after_coersion(),
            FlyCoersion::Never | FlyCoersion::WrapInSome | FlyCoersion::PlaceToLeash => {
                FlyQuary::Transient
            }
        }
    }
}

/// expect a type that is implicitly convertible to type under contract
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db]
pub struct ExpectCoersion {
    contract: TermContract,
    ty_expected: FlyTerm,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpectCoersionOutcome {
    coersion: FlyCoersion,
}

impl ExpectCoersionOutcome {
    pub fn coersion(&self) -> FlyCoersion {
        self.coersion
    }
}

impl ExpectCoersion {
    #[inline(always)]
    pub fn new(contract: TermContract, ty_expected: FlyTerm) -> Self {
        Self {
            contract,
            ty_expected,
        }
    }

    #[inline(always)]
    pub fn new_const(ty: FlyTerm) -> Self {
        Self {
            contract: TermContract::Const,
            ty_expected: ty,
        }
    }

    /// this will reduce place type to type
    #[inline(always)]
    pub fn new_pure(engine: &impl FlyTermEngine, ty: FlyTerm) -> Self {
        let ty = match ty.data(engine) {
            // FlyTermData::TypeOntologyAtPlace {
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
            contract: TermContract::Pure,
            ty_expected: ty,
        }
    }

    #[inline(always)]
    pub fn new_pure_unit(engine: &impl FlyTermEngine) -> Self {
        Self {
            contract: TermContract::Pure,
            ty_expected: engine.term_menu().unit_ty_ontology().into(),
        }
    }

    #[inline(always)]
    pub fn new_pure_bool(engine: &impl FlyTermEngine) -> Self {
        Self {
            contract: TermContract::Pure,
            ty_expected: engine.term_menu().bool_ty_ontology().into(),
        }
    }

    #[inline(always)]
    pub fn new_move(ty: FlyTerm) -> Self {
        Self {
            contract: TermContract::Move,
            ty_expected: ty,
        }
    }

    fn ty_expected(self) -> FlyTerm {
        self.ty_expected
    }
}

impl ExpectFlyTerm for ExpectCoersion {
    type Outcome = ExpectCoersionOutcome;

    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::Coersion(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FlyTerms) -> FinalDestination {
        self.ty_expected().final_destination_inner(db, terms)
    }

    fn destination(&self) -> FlyTermDestination {
        FlyTermDestination::Specific(self.ty_expected())
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        self.resolve_trivial(db, terms, state)?;
        self.resolve_holed(db, terms, state)?;
        self.resolve_never(db, terms, state)?;
        self.resolve_wrap_in_some(db, terms, state)?;
        self.resolve_deref(db, terms, state)?;
        self.resolve_reref(db, terms, state)?;
        match state.resolve_progress() {
            ExpectationProgress::Intact => state.set_err(
                OriginalFlyTermExpectationError::ExpectedCoersion {
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

impl ExpectCoersion {
    fn try_finalize_coersion(
        &self,
        src: FlyTerm,
        dst: FlyTerm,
        coersion: impl Into<FlyCoersion>,
        db: &::salsa::Db,
        terms: &FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        let coersion = coersion.into();
        Self::try_finalize_coersion_aux(
            src,
            dst,
            coersion
                .place_after_coersion()
                .bind(self.contract)
                .map(|()| coersion)
                .map_err(Into::into),
            db,
            terms,
            state,
        )
    }

    fn try_finalize_coersion_aux(
        src: FlyTerm,
        dst: FlyTerm,
        coersion_result: FlyTermExpectationResult<FlyCoersion>,
        db: &::salsa::Db,
        terms: &FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        let src_base_ty_data = src.base_ty_data_inner(db, terms);
        let dst_base_ty_data = dst.base_ty_data_inner(db, terms);
        let outcome_result = coersion_result.map(|coersion| ExpectCoersionOutcome { coersion });
        if src_base_ty_data == dst_base_ty_data {
            return state.set_result(outcome_result, smallvec![]);
        }
        match src_base_ty_data {
            FlyBaseTypeData::TypeOntology {
                ty_path: src_ty_path,
                ty_arguments: src_ty_arguments,
                ..
            } => match dst_base_ty_data {
                FlyBaseTypeData::TypeOntology {
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
                            actions.push(FlyTermResolveAction::AddExpectation {
                                src: state.child_src(),
                                expectee: src_argument_ty,
                                expectation: ExpectSubtypeOrEqual::new(dst_argument_ty).into(),
                            })
                        }
                    }
                    state.set_result(outcome_result, actions)
                }
                _ => AltNone,
            },
            FlyBaseTypeData::Curry { .. } => AltNone,
            FlyBaseTypeData::Hole(_, _) => AltNone,
            FlyBaseTypeData::Category(_) => todo!(),
            FlyBaseTypeData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => {
                p!(src_base_ty_data.debug(db), dst_base_ty_data.debug(db));
                todo!()
            }
            FlyBaseTypeData::Symbol { symbol: term } => AltNone,
            FlyBaseTypeData::Hvar { hvar } => todo!(),
        }
    }
}
