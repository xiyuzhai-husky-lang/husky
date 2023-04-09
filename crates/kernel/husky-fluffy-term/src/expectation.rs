mod any_derived;
mod any_original;
mod eqs_category;
mod eqs_exactly;
mod eqs_function_ty;
mod explicitly_convertible;
mod implicitly_convertible;
mod ins_sort;

pub use self::any_derived::*;
pub use self::any_original::*;
pub use self::eqs_category::*;
pub use self::eqs_exactly::*;
pub use self::eqs_function_ty::*;
pub use self::explicitly_convertible::*;
pub use self::implicitly_convertible::*;
pub use self::ins_sort::*;

use super::*;
use husky_print_utils::p;
use husky_ty_expectation::TypePathDisambiguation;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};
use thiserror::Error;

pub trait ExpectLocalTerm: Into<FluffyTermExpectationData> + Clone {
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

    fn destination_term_data(
        &self,
        db: &dyn TermDb,
        terms: &FluffyTerms,
    ) -> Option<FluffyTermData> {
        todo!()
        // self.destination()
        //     .map(|destination| destination.data_inner(db, porous_terms))
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

pub type FluffyTermExpectationIdx = ArenaIdx<FluffyTermExpectationEntry>;
pub type OptionFluffyTermExpectationIdx = OptionArenaIdx<FluffyTermExpectationEntry>;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum FluffyTermExpectationOutcome {
    ExplicitlyConvertible(ExpectExplicitlyConvertibleOutcome),
    ImplicitlyConvertible(ImplicitConversion),
    InsSort(ExpectInsSortOutcome),
    EqsSort(TermUniverse),
    EqsExactly(ExpectSubtypeOutcome),
    EqsRitchieCallType(ExpectEqsFunctionTypeOutcome),
}

impl FluffyTermExpectationOutcome {
    fn resolved(&self) -> Option<Term> {
        match self {
            FluffyTermExpectationOutcome::ExplicitlyConvertible(_) => todo!(),
            FluffyTermExpectationOutcome::ImplicitlyConvertible(_) => todo!(),
            FluffyTermExpectationOutcome::InsSort(result) => result.resolved(),
            FluffyTermExpectationOutcome::EqsSort(_) => todo!(),
            FluffyTermExpectationOutcome::EqsExactly(result) => result.resolved(),
            FluffyTermExpectationOutcome::EqsRitchieCallType(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum FluffyTermExpectationResolveProgress {
    Unresolved,
    Resolved(FluffyTermExpectationResult<FluffyTermExpectationOutcome>),
}

impl FluffyTermExpectationResolveProgress {
    pub fn outcome<E: ExpectLocalTerm>(&self) -> Option<&E::Outcome> {
        match self {
            FluffyTermExpectationResolveProgress::Unresolved => None,
            FluffyTermExpectationResolveProgress::Resolved(Ok(outcome)) => {
                Some(E::retrieve_outcome(outcome))
            }
            FluffyTermExpectationResolveProgress::Resolved(Err(_)) => None,
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
    TermTypeError { term: Term, error: TermError },
    #[error("{0}")]
    Type(#[from] TermError),
    #[error("target substitution failure")]
    TargetSubstitutionFailure,
    #[error("duplication")]
    Duplication(FluffyTermExpectationIdx),
    #[error("unresolved local term")]
    UnresolvedLocalTerm,
    #[error("type path {ty_path:?} type error {error}")]
    TypePathTypeError { ty_path: TypePath, error: TermError },
}

impl FluffyTermExpectationResolveProgress {
    // it will use derived type error
    pub fn duplicate(&self, src: FluffyTermExpectationIdx) -> Self {
        match self {
            FluffyTermExpectationResolveProgress::Unresolved => {
                FluffyTermExpectationResolveProgress::Unresolved
            }
            FluffyTermExpectationResolveProgress::Resolved(expectation_result) => {
                match expectation_result {
                    Ok(expectation_ok) => {
                        FluffyTermExpectationResolveProgress::Resolved(Ok(expectation_ok.clone()))
                    }
                    Err(_) => FluffyTermExpectationResolveProgress::Resolved(Err(
                        DerivedFluffyTermExpectationError::Duplication(src).into(),
                    )),
                }
            }
        }
    }

    pub(crate) fn reduced_term(&self) -> Option<Term> {
        match self {
            FluffyTermExpectationResolveProgress::Unresolved
            | FluffyTermExpectationResolveProgress::Resolved(Err(_)) => None,
            FluffyTermExpectationResolveProgress::Resolved(Ok(result)) => result.resolved(),
        }
    }
}

pub(super) struct FluffyTermExpectationEffect {
    pub(super) result: FluffyTermExpectationResult<FluffyTermExpectationOutcome>,
    pub(super) actions: SmallVec<[FluffyTermResolveAction; 2]>,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FluffyTermExpectations {
    arena: Arena<FluffyTermExpectationEntry>,
    first_unresolved_expectation: usize,
}

impl std::ops::Index<FluffyTermExpectationIdx> for FluffyTermExpectations {
    type Output = FluffyTermExpectationEntry;

    fn index(&self, index: FluffyTermExpectationIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl FluffyTermExpectations {
    pub(super) fn unresolved_rule_iter(
        &self,
    ) -> impl Iterator<Item = (FluffyTermExpectationIdx, &FluffyTermExpectationEntry)> {
        self.arena
            .indexed_iter_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                FluffyTermExpectationResolveProgress::Unresolved => true,
                FluffyTermExpectationResolveProgress::Resolved(_) => false,
            })
    }

    pub fn iter(&self) -> impl Iterator<Item = &FluffyTermExpectationEntry> {
        self.arena.iter()
    }

    pub(super) fn unresolved_indexed_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (FluffyTermExpectationIdx, &mut FluffyTermExpectationEntry)> {
        self.arena
            .indexed_iter_mut_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                FluffyTermExpectationResolveProgress::Unresolved => true,
                FluffyTermExpectationResolveProgress::Resolved(_) => false,
            })
    }

    pub(super) fn alloc_rule(
        &mut self,
        rule: FluffyTermExpectationEntry,
    ) -> FluffyTermExpectationIdx {
        self.arena.alloc_one(rule)
    }

    pub(super) fn take_effect(
        &mut self,
        rule_idx: FluffyTermExpectationIdx,
        effect: FluffyTermExpectationEffect,
    ) -> Option<SmallVec<[FluffyTermResolveAction; 2]>> {
        self.arena
            .update(rule_idx, |rule| rule.set_resolved(effect.result));
        Some(effect.actions)
    }
}

impl FluffyTermRegion {
    pub fn add_expectation_rule(
        &mut self,
        src: ExpectationSource,
        expectee: FluffyTerm,
        expectation: impl Into<FluffyTermExpectationData>,
    ) -> OptionFluffyTermExpectationIdx {
        todo!()
        // self.expectations
        //     .alloc_rule(FluffyTermExpectationRule {
        //         src,
        //         expectee: expectee.into(),
        //         expectation: expectation.into(),
        //         resolve_progress: FluffyTermExpectationResolveProgress::Unresolved,
        //     })
        //     .into()
    }
}

impl FluffyTermExpectationEntry {
    pub(super) fn resolve_expectation(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        idx: FluffyTermExpectationIdx,
        level: FluffyTermResolveLevel,
    ) -> Option<FluffyTermExpectationEffect> {
        match self.data() {
            FluffyTermExpectationData::ExplicitlyConvertible(ref expectation) => {
                expectation.resolve(db, terms, self.expectee(), level)
            }
            FluffyTermExpectationData::ImplicitlyConvertible(exp) => {
                exp.resolve(db, terms, idx, self.expectee(), level)
            }
            FluffyTermExpectationData::EqsSort(ref expectation) => {
                expectation.resolve(db, self.expectee(), terms)
            }
            FluffyTermExpectationData::FrameVariableType => todo!(),
            FluffyTermExpectationData::EqsFunctionType(ref expectation) => {
                expectation.resolve(db, terms, idx, self.expectee())
            }
            FluffyTermExpectationData::InsSort(ref expectation) => {
                expectation.resolve(db, terms, self.expectee())
            }
            FluffyTermExpectationData::EqsExactly(ref expectation) => {
                expectation.resolve(db, terms, self.src(), self.expectee())
            }
            FluffyTermExpectationData::AnyOriginal(_) => None,
            FluffyTermExpectationData::AnyDerived(_) => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum FluffyTermExpectationData {
    ExplicitlyConvertible(ExpectExplicitlyConvertible),
    ImplicitlyConvertible(ExpectImplicitlyConvertible),
    /// expect term to be an instance of Type u for some universe
    InsSort(ExpectInsSort),
    EqsSort(ExpectEqsCategory),
    FrameVariableType,
    EqsExactly(ExpectSubtype),
    EqsFunctionType(ExpectEqsFunctionType),
    AnyOriginal(ExpectAnyOriginal),
    AnyDerived(ExpectAnyDerived),
}

impl std::ops::Index<FluffyTermExpectationIdx> for FluffyTermRegion {
    type Output = FluffyTermExpectationEntry;

    fn index(&self, index: FluffyTermExpectationIdx) -> &Self::Output {
        todo!()
    }
}
