mod any_derived;
mod any_original;
mod casting;
mod coersion;
mod condition_ty;
mod curry_destination;
mod eqs_category;
mod eqs_function_ty;
mod eqs_ritchie_ty;
mod final_destination;
mod int_ty;
mod subtype;

pub use self::any_derived::*;
pub use self::any_original::*;
pub use self::casting::*;
pub use self::coersion::*;
pub use self::condition_ty::*;
pub use self::curry_destination::*;
pub use self::eqs_category::*;
pub use self::eqs_function_ty::*;
pub use self::eqs_ritchie_ty::*;
pub use self::final_destination::*;
pub use self::int_ty::*;
pub use self::subtype::*;

use super::*;
use husky_print_utils::p;
use idx_arena::{Arena, ArenaIdx};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
#[salsa::debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum Expectation {
    ExplicitlyConvertible(ExpectCasting),
    ImplicitlyConvertible(ExpectCoersion),
    EqsSort(ExpectEqsCategory),
    FrameVariableType,
    EqsExactly(ExpectSubtype),
    EqsFunctionType(ExpectEqsFunctionType),
    EqsRitchieType(ExpectEqsRitchieType),
    AnyOriginal(ExpectAnyOriginal),
    AnyDerived(ExpectAnyDerived),
    ConditionType(ExpectConditionType),
    IntType(ExpectIntType),
    FinalDestination(ExpectFinalDestination),
    CurryDestination(ExpectCurryDestination),
}

impl Expectation {
    /// basically enum version of virtual method dispath
    pub(crate) fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        meta: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        match meta.resolve_progress() {
            ExpectationProgress::Intact | ExpectationProgress::Holed => (),
            ExpectationProgress::Resolved(_) => return AltNone,
        }
        match self {
            Expectation::ExplicitlyConvertible(epn) => epn.resolve(db, terms, meta),
            Expectation::ImplicitlyConvertible(epn) => epn.resolve(db, terms, meta),
            Expectation::EqsSort(epn) => epn.resolve(db, terms, meta),
            Expectation::FrameVariableType => todo!(),
            Expectation::EqsFunctionType(epn) => epn.resolve(db, terms, meta),
            Expectation::EqsRitchieType(epn) => epn.resolve(db, terms, meta),
            Expectation::EqsExactly(epn) => epn.resolve(db, terms, meta),
            Expectation::AnyOriginal(epn) => epn.resolve(db, terms, meta),
            Expectation::AnyDerived(epn) => epn.resolve(db, terms, meta),
            Expectation::IntType(epn) => epn.resolve(db, terms, meta),
            Expectation::FinalDestination(epn) => epn.resolve(db, terms, meta),
            Expectation::CurryDestination(epn) => epn.resolve(db, terms, meta),
            Expectation::ConditionType(epn) => epn.resolve(db, terms, meta),
        }
    }
}

pub trait ExpectFluffyTerm: Into<Expectation> + Clone {
    type Outcome: Clone;

    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome;

    /// final destination of `A1 -> ... -> An` is equal to that of `An`
    ///
    /// final destination of `A1 ... An` is equal to that of `A1`
    ///
    /// final destination of `Sort` is `FinalDestination::Sort`
    ///
    /// final destination of a type path `A` is `FinalDestination::TypePath(A)`
    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination;

    #[inline(always)]
    fn final_destination(&self, engine: &impl FluffyTermEngine<'_>) -> FinalDestination {
        self.final_destination_inner(engine.db(), engine.fluffy_terms())
    }

    #[inline(always)]
    fn disambiguate_ty_path(&self, engine: &impl FluffyTermEngine<'_>) -> TypePathDisambiguation {
        self.disambiguate_ty_path_inner(engine.db(), engine.fluffy_terms())
    }

    /// if ty_path's type is under this expectation, disambiguate whether it's an ontology or constructor
    // final
    #[inline(always)]
    fn disambiguate_ty_path_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> TypePathDisambiguation {
        match self.final_destination_inner(db, terms) {
            FinalDestination::Sort => TypePathDisambiguation::OntologyConstructor,
            FinalDestination::TypeOntology
            | FinalDestination::AnyOriginal
            | FinalDestination::AnyDerived => TypePathDisambiguation::InstanceConstructor,
            FinalDestination::Ritchie(RitchieKind::FnType) => {
                TypePathDisambiguation::InstanceConstructor
            }
            FinalDestination::Ritchie(_) => todo!(),
        }
    }

    fn destination(&self) -> Option<FluffyTerm>;

    fn destination_term_data<'a>(
        &self,
        db: &'a dyn FluffyTermDb,
        fluffy_terms: &'a FluffyTerms,
    ) -> Option<FluffyTermData<'a>> {
        self.destination()
            .map(|destination| destination.data_inner(db, fluffy_terms))
    }

    /// needs to return option to indicate whether something has been changed
    fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect>;
}

pub type FluffyTermExpectationIdx = ArenaIdx<FluffyTermExpectationEntry>;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum FluffyTermExpectationOutcome {
    ExplicitlyConvertible(ExpectExplicitlyConvertibleOutcome),
    ImplicitlyConvertible(Coersion),
    EqsSort(TermUniverse),
    Subtype(ExpectSubtypeOutcome),
    EqsFunctionCallType(ExpectEqsFunctionTypeOutcome),
    EqsRitchieCallType(ExpectEqsRitchieTypeOutcome),
    IntType(ExpectIntTypeOutcome),
    ConditionType(ExpectConditionTypeOutcome),
}

impl FluffyTermExpectationOutcome {
    fn resolved(&self) -> Option<EtherealTerm> {
        match self {
            FluffyTermExpectationOutcome::ExplicitlyConvertible(_) => todo!(),
            FluffyTermExpectationOutcome::ImplicitlyConvertible(_) => todo!(),
            FluffyTermExpectationOutcome::EqsSort(_) => todo!(),
            FluffyTermExpectationOutcome::Subtype(result) => result.resolved(),
            FluffyTermExpectationOutcome::EqsFunctionCallType(_) => todo!(),
            FluffyTermExpectationOutcome::EqsRitchieCallType(_) => todo!(),
            FluffyTermExpectationOutcome::IntType(_) => todo!(),
            FluffyTermExpectationOutcome::ConditionType(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub enum ExpectationProgress {
    Intact,
    Holed,
    Resolved(FluffyTermExpectationResult<FluffyTermExpectationOutcome>),
}

impl ExpectationProgress {
    pub fn outcome<E: ExpectFluffyTerm>(&self) -> Option<&E::Outcome> {
        match self {
            ExpectationProgress::Intact | ExpectationProgress::Holed => None,
            ExpectationProgress::Resolved(Ok(outcome)) => Some(E::retrieve_outcome(outcome)),
            ExpectationProgress::Resolved(Err(_)) => None,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub enum FluffyTermExpectationError {
    #[error("original {0}")]
    Original(#[from] OriginalFluffyTermExpectationError),
    #[error("derived {0}")]
    Derived(#[from] DerivedFluffyTermExpectationError),
}

pub type FluffyTermExpectationResult<T> = Result<T, FluffyTermExpectationError>;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub enum OriginalFluffyTermExpectationError {
    #[error("type path mismatch for subtyping")]
    TypePathMismatchForSubtyping {
        expected: FluffyTerm,
        expectee: FluffyTerm,
        expected_path: TypePath,
        expectee_path: TypePath,
    },
    #[error("type path mismatch for coersion")]
    TypePathMismatchForCoersion {
        contract: Contract,
        ty_expected: FluffyTerm,
        expectee: FluffyTerm,
        expected_path: TypePath,
        expectee_path: TypePath,
    },
    #[error("expected category")]
    ExpectedCategory { expectee: FluffyTerm },
    #[error("expected subtype")]
    ExpectedSubtype { expectee: FluffyTerm },
    #[error("expected function type")]
    ExpectedFunctionType,
    #[error("ExpectedCoersion")]
    ExpectedCoersion {
        expectee: FluffyTerm,
        contract: Contract,
        expected: FluffyTerm,
    },
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub enum DerivedFluffyTermExpectationError {
    #[error("{term:?} {error}")]
    TermTypeError {
        term: EtherealTerm,
        error: EtherealTermError,
    },
    #[error("{0}")]
    Type(#[from] EtherealTermError),
    #[error("target substitution failure")]
    TargetSubstitutionFailure,
    #[error("duplication")]
    Duplication(FluffyTermExpectationIdx),
    #[error("unresolved local term")]
    UnresolvedLocalTerm,
    #[error("type path {ty_path:?} type error {error}")]
    TypePathTypeError {
        ty_path: TypePath,
        error: EtherealTermError,
    },
}
