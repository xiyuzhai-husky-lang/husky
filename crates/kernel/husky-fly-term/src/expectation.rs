mod any_derived;
mod any_original;
mod casting;
mod coersion;
mod condition_ty;
mod curry_destination;
mod final_destination;
mod function_ty;
mod int_ty;
pub mod num_ty;
mod ritchie_ty;
mod sort;
mod subtype;

pub use self::any_derived::*;
pub use self::any_original::*;
pub use self::casting::*;
pub use self::coersion::*;
pub use self::condition_ty::*;
pub use self::curry_destination::*;
pub use self::final_destination::*;
pub use self::function_ty::*;
pub use self::int_ty::*;
pub use self::num_ty::*;
use self::quary::FlyPlaceError;
pub use self::ritchie_ty::*;
pub use self::sort::*;
pub use self::subtype::*;

use super::*;
use husky_term_prelude::ritchie::RitchieKind;
use idx_arena::ArenaIdx;
use thiserror::Error;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
#[enum_class::from_variants]
pub enum Expectation {
    Casting(ExpectCasting),
    Coersion(ExpectCoersion),
    EqsSort(ExpectSort),
    LoopVariableType,
    EqsExactly(ExpectSubtypeOrEqual),
    EqsFunctionType(ExpectEqsFunctionType),
    EqsRitchieType(ExpectEqsRitchieType),
    AnyOriginal(ExpectAnyOriginal),
    AnyDerived(ExpectAnyDerived),
    ConditionType(ExpectConditionType),
    IntType(ExpectIntType),
    NumType(ExpectNumType),
    FinalDestination(ExpectFinalDestination),
    CurryDestination(ExpectCurryDestination),
}

impl Expectation {
    /// basically enum version of virtual method dispath
    pub(crate) fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        match state.resolve_progress() {
            ExpectationProgress::Intact | ExpectationProgress::Holed => (),
            ExpectationProgress::Resolved(_) => return AltNone,
        }
        match self {
            Expectation::Casting(epn) => epn.resolve(db, terms, state),
            Expectation::Coersion(epn) => epn.resolve(db, terms, state),
            Expectation::EqsSort(epn) => epn.resolve(db, terms, state),
            Expectation::LoopVariableType => todo!(),
            Expectation::EqsFunctionType(epn) => epn.resolve(db, terms, state),
            Expectation::EqsRitchieType(epn) => epn.resolve(db, terms, state),
            Expectation::EqsExactly(epn) => epn.resolve(db, terms, state),
            Expectation::AnyOriginal(epn) => epn.resolve(db, terms, state),
            Expectation::AnyDerived(epn) => epn.resolve(db, terms, state),
            Expectation::IntType(epn) => epn.resolve(db, terms, state),
            Expectation::NumType(epn) => epn.resolve(db, terms, state),
            Expectation::FinalDestination(epn) => epn.resolve(db, terms, state),
            Expectation::CurryDestination(epn) => epn.resolve(db, terms, state),
            Expectation::ConditionType(epn) => epn.resolve(db, terms, state),
        }
    }
}

impl ExpectFlyTerm for Expectation {
    type Outcome = ExpectationOutcome;

    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        todo!()
    }

    fn final_destination_inner(&self, db: &salsa::Db, terms: &FlyTerms) -> FinalDestination {
        todo!()
    }

    fn destination(&self) -> FlyTermDestination {
        todo!()
    }

    fn resolve(
        &self,
        db: &salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        todo!()
    }
}

// maybe make this Copy?
pub trait ExpectFlyTerm: Into<Expectation> + Clone {
    type Outcome: Clone + Into<ExpectationOutcome>;

    /// override this for ExpectAny*
    fn initial_resolve_progress() -> ExpectationProgress {
        ExpectationProgress::Intact
    }

    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome;

    /// final destination of `A1 -> ... -> An` is equal to that of `An`
    ///
    /// final destination of `A1 ... An` is equal to that of `A1`
    ///
    /// final destination of `Sort` is `FinalDestination::Sort`
    ///
    /// final destination of a type path `A` is `FinalDestination::TypePath(A)`
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FlyTerms) -> FinalDestination;

    #[inline(always)]
    fn final_destination(&self, engine: &impl FlyTermEngine<'_>) -> FinalDestination {
        self.final_destination_inner(engine.db(), engine.fly_terms())
    }

    #[inline(always)]
    fn disambiguate_ty_path(&self, engine: &impl FlyTermEngine<'_>) -> TypePathDisambiguation {
        self.disambiguate_ty_path_inner(engine.db(), engine.fly_terms())
    }

    /// if ty_path's type is under this expectation, disambiguate whether it's an ontology or constructor
    // final
    #[inline(always)]
    fn disambiguate_ty_path_inner(
        &self,
        db: &::salsa::Db,
        terms: &FlyTerms,
    ) -> TypePathDisambiguation {
        match self.final_destination_inner(db, terms) {
            FinalDestination::Sort => TypePathDisambiguation::OntologyConstructor,
            FinalDestination::TypeOntology
            | FinalDestination::AnyOriginal
            | FinalDestination::AnyDerived => TypePathDisambiguation::InstanceConstructor,
            FinalDestination::Ritchie(RitchieKind::RITCHIE_TYPE_FN) => {
                TypePathDisambiguation::InstanceConstructor
            }
            FinalDestination::Ritchie(_) => todo!(),
        }
    }

    fn destination(&self) -> FlyTermDestination;

    // fn destination_term_data<'a>(
    //     &self,
    //     db: &'a ::salsa::Db,
    //     fly_terms: &'a FlyTerms,
    // ) ->  FlyTermData<'a>  {
    //     self.destination()
    //         .map(|destination| destination.data_inner(db, fly_terms))
    // }

    /// needs to return option to indicate whether something has been changed
    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect>;
}

pub enum FlyTermDestination {
    AnyOriginal,
    AnyDerived,
    Specific(FlyTerm),
}

pub type FlyTermExpectationIdx = ArenaIdx<FlyTermExpectationEntry>;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
#[enum_class::from_variants]
pub enum ExpectationOutcome {
    ExplicitlyConvertible(ExpectCastingOutcome),
    Coersion(ExpectCoersionOutcome),
    EqsSort(Universe),
    Subtype(ExpectSubtypeOutcome),
    EqsFunctionCallType(ExpectEqsFunctionTypeOutcome),
    EqsRitchieCallType(ExpectEqsRitchieTypeOutcome),
    IntType(ExpectIntTypeOutcome),
    NumType(ExpectNumTypeOutcome),
    ConditionType(ExpectConditionTypeOutcome),
    CurryDestination(ExpectCurryDestinationOutcome),
    FinalDestination(ExpectFinalDestinationOutcome),
    AnyOriginal(ExpectAnyOriginalOutcome),
    AnyDerived(ExpectAnyDerivedOutcome),
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum ExpectationProgress {
    Intact,
    Holed,
    Resolved(FlyTermExpectationResult<ExpectationOutcome>),
}

impl ExpectationProgress {
    pub fn outcome<E: ExpectFlyTerm>(&self) -> Option<&E::Outcome> {
        match self {
            ExpectationProgress::Intact | ExpectationProgress::Holed => None,
            ExpectationProgress::Resolved(Ok(outcome)) => Some(E::retrieve_outcome(outcome)),
            ExpectationProgress::Resolved(Err(_)) => None,
        }
    }

    pub fn outcome2(&self) -> Option<&ExpectationOutcome> {
        match self {
            ExpectationProgress::Intact | ExpectationProgress::Holed => None,
            ExpectationProgress::Resolved(Ok(outcome)) => Some(outcome),
            ExpectationProgress::Resolved(Err(_)) => None,
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum FlyTermExpectationError {
    #[error("original {0}")]
    Original(#[from] OriginalFlyTermExpectationError),
    #[error("derived {0}")]
    Derived(#[from] DerivedFlyTermExpectationError),
}

pub type FlyTermExpectationResult<T> = Result<T, FlyTermExpectationError>;

impl From<FlyPlaceError> for FlyTermExpectationError {
    fn from(e: FlyPlaceError) -> Self {
        FlyTermExpectationError::Original(e.into())
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalFlyTermExpectationError {
    #[error("type path mismatch for subtyping")]
    TypePathMismatchForSubtyping {
        expected: FlyTerm,
        expectee: FlyTerm,
        expected_path: TypePath,
        expectee_path: TypePath,
    },
    #[error("type path mismatch for coersion")]
    TypePathMismatchForCoersion {
        contract: Contract,
        ty_expected: FlyTerm,
        expectee: FlyTerm,
        expected_path: TypePath,
        expectee_path: TypePath,
    },
    #[error("expected category")]
    ExpectedCategory { expectee: FlyTerm },
    #[error("expected subtype")]
    ExpectedSubtype { expectee: FlyTerm },
    #[error("expected function type")]
    ExpectedFunctionType,
    #[error("ExpectedCoersion")]
    ExpectedCoersion {
        expectee: FlyTerm,
        contract: Contract,
        expected: FlyTerm,
    },
    #[error("ExpectedIntType")]
    ExpectedIntType { expectee: FlyTerm },
    #[error("place")]
    Place(#[from] FlyPlaceError),
    #[error("todo")]
    Todo,
}

#[salsa::debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedFlyTermExpectationError {
    #[error("{term:?} {error}")]
    TermTypeError { term: EthTerm, error: EthTermError },
    #[error("{0}")]
    Type(#[from] EthTermError),
    #[error("target substitution failure")]
    TargetSubstitutionFailure,
    #[error("duplication")]
    Duplication(FlyTermExpectationIdx),
    #[error("unresolved local term")]
    UnresolvedLocalTerm,
    #[error("type path {ty_path:?} type error {error}")]
    TypePathTypeError {
        ty_path: TypePath,
        error: EthTermError,
    },
    #[error("Variance")]
    Variance,
}
