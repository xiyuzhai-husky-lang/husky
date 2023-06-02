mod any_derived;
mod any_original;
mod any_towards_final_destination;
mod eqs_category;
mod eqs_exactly;
mod eqs_function_ty;
mod eqs_ritchie_ty;
mod explicitly_convertible;
mod implicitly_convertible;
mod ins_sort;
mod num_ty;

pub use self::any_derived::*;
pub use self::any_original::*;
pub use self::any_towards_final_destination::*;
pub use self::eqs_category::*;
pub use self::eqs_exactly::*;
pub use self::eqs_function_ty::*;
pub use self::eqs_ritchie_ty::*;
pub use self::explicitly_convertible::*;
pub use self::implicitly_convertible::*;
pub use self::ins_sort::*;
pub use self::num_ty::*;

use super::*;
use husky_print_utils::p;
use husky_ty_expectation::TypePathDisambiguation;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum ExpectationData {
    ExplicitlyConvertible(ExpectExplicitlyConvertible),
    ImplicitlyConvertible(ExpectImplicitlyConvertible),
    /// expect term to be an instance of Type u for some universe
    InsSort(ExpectInsSort),
    EqsSort(ExpectEqsCategory),
    FrameVariableType,
    EqsExactly(ExpectSubtype),
    EqsFunctionType(ExpectEqsFunctionType),
    EqsRitchieType(ExpectEqsRitchieType),
    AnyOriginal(ExpectAnyOriginal),
    AnyDerived(ExpectAnyDerived),
    NumType(ExpectNumType),
    AnyTowardsFinalDestination(ExpectAnyTowardsFinalDestination),
}

pub trait ExpectFluffyTerm: Into<ExpectationData> + Clone {
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
            FinalDestination::Sort => TypePathDisambiguation::Ontology,
            FinalDestination::TypeOntology
            | FinalDestination::AnyOriginal
            | FinalDestination::AnyDerived => TypePathDisambiguation::Constructor,
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
}

/// final destination of `A1 -> ... -> An` is equal to that of `An`
///
/// final destination of `A1 ... An` is equal to that of `A1`
///
/// final destination of `Sort` is `FinalDestination::Sort`
///
/// final destination of a type path `A` is `FinalDestination::TypePath(A)`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum FinalDestination {
    Sort,
    TypeOntology,
    AnyOriginal,
    AnyDerived,
}

pub type FluffyTermExpectationIdx = ArenaIdx<ExpectationEntry>;
pub type OptionFluffyTermExpectationIdx = OptionArenaIdx<ExpectationEntry>;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum FluffyTermExpectationOutcome {
    ExplicitlyConvertible(ExpectExplicitlyConvertibleOutcome),
    ImplicitlyConvertible(ImplicitConversion),
    InsSort(ExpectInsSortOutcome),
    EqsSort(TermUniverse),
    EqsExactly(ExpectSubtypeOutcome),
    EqsFunctionCallType(ExpectEqsFunctionTypeOutcome),
    EqsRitchieCallType(ExpectEqsRitchieTypeOutcome),
    NumType(ExpectNumTypeOutcome),
}

impl FluffyTermExpectationOutcome {
    fn resolved(&self) -> Option<EtherealTerm> {
        match self {
            FluffyTermExpectationOutcome::ExplicitlyConvertible(_) => todo!(),
            FluffyTermExpectationOutcome::ImplicitlyConvertible(_) => todo!(),
            FluffyTermExpectationOutcome::InsSort(result) => result.resolved(),
            FluffyTermExpectationOutcome::EqsSort(_) => todo!(),
            FluffyTermExpectationOutcome::EqsExactly(result) => result.resolved(),
            FluffyTermExpectationOutcome::EqsFunctionCallType(_) => todo!(),
            FluffyTermExpectationOutcome::EqsRitchieCallType(_) => todo!(),
            FluffyTermExpectationOutcome::NumType(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum ExpectationResolveProgress {
    Unresolved,
    Resolved(FluffyTermExpectationResult<FluffyTermExpectationOutcome>),
}

impl ExpectationResolveProgress {
    pub fn outcome<E: ExpectFluffyTerm>(&self) -> Option<&E::Outcome> {
        match self {
            ExpectationResolveProgress::Unresolved => None,
            ExpectationResolveProgress::Resolved(Ok(outcome)) => Some(E::retrieve_outcome(outcome)),
            ExpectationResolveProgress::Resolved(Err(_)) => None,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum FluffyTermExpectationError {
    #[error("original {0}")]
    Original(#[from] OriginalFluffyTermExpectationError),
    #[error("derived {0}")]
    Derived(#[from] DerivedFluffyTermExpectationError),
}

pub type FluffyTermExpectationResult<T> = Result<T, FluffyTermExpectationError>;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum OriginalFluffyTermExpectationError {
    #[error("type path mismatch")]
    TypePathMismatch {
        expected_path: TypePath,
        expectee_path: TypePath,
    },
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
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

impl ExpectationResolveProgress {
    // it will use derived type error
    pub fn duplicate(&self, src: FluffyTermExpectationIdx) -> Self {
        match self {
            ExpectationResolveProgress::Unresolved => ExpectationResolveProgress::Unresolved,
            ExpectationResolveProgress::Resolved(expectation_result) => match expectation_result {
                Ok(expectation_ok) => {
                    ExpectationResolveProgress::Resolved(Ok(expectation_ok.clone()))
                }
                Err(_) => ExpectationResolveProgress::Resolved(Err(
                    DerivedFluffyTermExpectationError::Duplication(src).into(),
                )),
            },
        }
    }

    pub(crate) fn reduced_term(&self) -> Option<EtherealTerm> {
        match self {
            ExpectationResolveProgress::Unresolved
            | ExpectationResolveProgress::Resolved(Err(_)) => None,
            ExpectationResolveProgress::Resolved(Ok(result)) => result.resolved(),
        }
    }
}

pub(super) struct FluffyTermExpectationEffect {
    pub(super) result: FluffyTermExpectationResult<FluffyTermExpectationOutcome>,
    pub(super) actions: SmallVec<[FluffyTermResolveAction; 2]>,
}

impl ExpectationEntry {
    pub(super) fn resolve_expectation(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        idx: FluffyTermExpectationIdx,
        level: FluffyTermResolveLevel,
    ) -> Option<FluffyTermExpectationEffect> {
        match self.data() {
            ExpectationData::ExplicitlyConvertible(ref expectation) => {
                expectation.resolve(db, terms, self.expectee(), level)
            }
            ExpectationData::ImplicitlyConvertible(expectation) => {
                expectation.resolve(db, terms, self.src().child_src(idx), self.expectee(), level)
            }
            ExpectationData::EqsSort(ref expectation) => {
                expectation.resolve(db, self.expectee(), terms)
            }
            ExpectationData::FrameVariableType => todo!(),
            ExpectationData::EqsFunctionType(ref expectation) => {
                expectation.resolve(db, terms, idx, self.expectee())
            }
            ExpectationData::EqsRitchieType(ref expectation) => {
                expectation.resolve(db, terms, idx, self.expectee())
            }
            ExpectationData::InsSort(ref expectation) => {
                expectation.resolve(db, terms, self.expectee())
            }
            ExpectationData::EqsExactly(ref expectation) => {
                expectation.resolve(db, terms, self.expectee())
            }
            ExpectationData::AnyOriginal(_) => None,
            ExpectationData::AnyDerived(_) => None,
            ExpectationData::NumType(expectation) => {
                expectation.resolve(db, terms, self.expectee())
            }
            ExpectationData::AnyTowardsFinalDestination(_) => None,
        }
    }
}
