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
use idx_arena::Arena;
use thiserror::Error;

pub trait ExpectLocalTerm: Into<LocalTermExpectation> + Clone {
    type Outcome: Clone;

    fn retrieve_outcome(outcome: &LocalTermExpectationOutcome) -> &Self::Outcome;

    /// final destination of `A1 -> ... -> An` is equal to that of `An`
    ///
    /// final destination of `A1 ... An` is equal to that of `A1`
    ///
    /// final destination of `Sort` is `FinalDestination::Sort`
    ///
    /// final destination of a type path `A` is `FinalDestination::TypePath(A)`
    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn TermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination;

    /// if ty_path's type is under this expectation, disambiguate whether it's an ontology or constructor
    // final
    #[inline(always)]
    fn disambiguate_ty_path(
        &self,
        db: &dyn TermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> TypePathDisambiguation {
        match self.final_destination(db, unresolved_terms) {
            FinalDestination::Sort => TypePathDisambiguation::Ontology,
            FinalDestination::TypeOntology
            | FinalDestination::AnyOriginal
            | FinalDestination::AnyDerived => TypePathDisambiguation::Constructor,
        }
    }

    fn destination(&self) -> Option<LocalTerm>;

    fn destination_pattern(
        &self,
        db: &dyn TermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> Option<LocalTermPattern> {
        self.destination()
            .map(|destination| destination.pattern(db, unresolved_terms))
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
#[salsa::derive_debug_with_db(db = TermDb)]
pub enum FinalDestination {
    Sort,
    TypeOntology,
    AnyOriginal,
    AnyDerived,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub struct LocalTermExpectationRule {
    src_expr_idx: ExprIdx,
    expectee: LocalTerm,
    expectation: LocalTermExpectation,
    resolve_progress: LocalTermExpectationResolveProgress,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = TermDb)]
#[enum_class::from_variants]
pub enum LocalTermExpectationOutcome {
    ExplicitlyConvertible(ExpectExplicitlyConvertibleOutcome),
    ImplicitlyConvertible(ImplicitConversion),
    InsSort(ExpectInsSortOutcome),
    EqsSort(TermUniverse),
    EqsExactly(ExpectSubtypeOutcome),
    EqsRitchieCallType(ExpectEqsFunctionTypeOutcome),
}

impl LocalTermExpectationOutcome {
    fn resolved(&self) -> Option<Term> {
        match self {
            LocalTermExpectationOutcome::ExplicitlyConvertible(_) => todo!(),
            LocalTermExpectationOutcome::ImplicitlyConvertible(_) => todo!(),
            LocalTermExpectationOutcome::InsSort(result) => result.resolved(),
            LocalTermExpectationOutcome::EqsSort(_) => todo!(),
            LocalTermExpectationOutcome::EqsExactly(result) => result.resolved(),
            LocalTermExpectationOutcome::EqsRitchieCallType(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub enum LocalTermExpectationResolveProgress {
    Unresolved,
    Resolved(LocalTermExpectationResult<LocalTermExpectationOutcome>),
}

impl LocalTermExpectationResolveProgress {
    pub fn outcome<E: ExpectLocalTerm>(&self) -> Option<&E::Outcome> {
        match self {
            LocalTermExpectationResolveProgress::Unresolved => None,
            LocalTermExpectationResolveProgress::Resolved(Ok(outcome)) => {
                Some(E::retrieve_outcome(outcome))
            }
            LocalTermExpectationResolveProgress::Resolved(Err(_)) => None,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub enum LocalTermExpectationError {
    #[error("original {0}")]
    Original(#[from] OriginalLocalTermExpectationError),
    #[error("derived {0}")]
    Derived(#[from] DerivedLocalTermExpectationError),
}

pub type LocalTermExpectationResult<T> = Result<T, LocalTermExpectationError>;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub enum OriginalLocalTermExpectationError {
    #[error("type path mismatch")]
    TypePathMismatch {
        expected_path: TypePath,
        expectee_path: TypePath,
    },
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub enum DerivedLocalTermExpectationError {
    #[error("{term:?} {error}")]
    TermTypeError { term: Term, error: TermError },
    #[error("{0}")]
    Type(#[from] TermError),
    #[error("target substitution failure")]
    TargetSubstitutionFailure,
    #[error("duplication")]
    Duplication(LocalTermExpectationIdx),
    #[error("unresolved local term")]
    UnresolvedLocalTerm,
    #[error("type path {ty_path:?} type error {error}")]
    TypePathTypeError { ty_path: TypePath, error: TermError },
}

impl LocalTermExpectationResolveProgress {
    // it will use derived type error
    pub fn duplicate(&self, src: LocalTermExpectationIdx) -> Self {
        match self {
            LocalTermExpectationResolveProgress::Unresolved => {
                LocalTermExpectationResolveProgress::Unresolved
            }
            LocalTermExpectationResolveProgress::Resolved(expectation_result) => {
                match expectation_result {
                    Ok(expectation_ok) => {
                        LocalTermExpectationResolveProgress::Resolved(Ok(expectation_ok.clone()))
                    }
                    Err(_) => LocalTermExpectationResolveProgress::Resolved(Err(
                        DerivedLocalTermExpectationError::Duplication(src).into(),
                    )),
                }
            }
        }
    }

    pub(crate) fn reduced_term(&self) -> Option<Term> {
        match self {
            LocalTermExpectationResolveProgress::Unresolved
            | LocalTermExpectationResolveProgress::Resolved(Err(_)) => None,
            LocalTermExpectationResolveProgress::Resolved(Ok(result)) => result.resolved(),
        }
    }
}

impl LocalTermExpectationRule {
    pub(crate) fn variant(&self) -> &LocalTermExpectation {
        &self.expectation
    }

    pub fn resolve_progress(&self) -> &LocalTermExpectationResolveProgress {
        &self.resolve_progress
    }

    pub fn original_error(&self) -> Option<&OriginalLocalTermExpectationError> {
        match self.resolve_progress {
            LocalTermExpectationResolveProgress::Resolved(Err(
                LocalTermExpectationError::Original(ref e),
            )) => Some(e),
            _ => None,
        }
    }

    pub(crate) fn expectee(&self) -> LocalTerm {
        self.expectee
    }

    pub fn src_expr_idx(&self) -> ExprIdx {
        self.src_expr_idx
    }

    pub(crate) fn set_resolved(
        &mut self,
        result: LocalTermExpectationResult<LocalTermExpectationOutcome>,
    ) {
        self.resolve_progress = LocalTermExpectationResolveProgress::Resolved(result)
    }
}

pub(super) struct LocalTermExpectationEffect {
    pub(super) result: LocalTermExpectationResult<LocalTermExpectationOutcome>,
    pub(super) actions: SmallVec<[TermResolveAction; 2]>,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct LocalTermExpectations {
    arena: Arena<LocalTermExpectationRule>,
    first_unresolved_expectation: usize,
}

impl std::ops::Index<LocalTermExpectationIdx> for LocalTermExpectations {
    type Output = LocalTermExpectationRule;

    fn index(&self, index: LocalTermExpectationIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl LocalTermExpectations {
    pub(super) fn unresolved_rule_iter(
        &self,
    ) -> impl Iterator<Item = (LocalTermExpectationIdx, &LocalTermExpectationRule)> {
        self.arena
            .indexed_iter_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::Resolved(_) => false,
            })
    }

    pub fn iter(&self) -> impl Iterator<Item = &LocalTermExpectationRule> {
        self.arena.iter()
    }

    pub(super) fn unresolved_indexed_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (LocalTermExpectationIdx, &mut LocalTermExpectationRule)> {
        self.arena
            .indexed_iter_mut_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::Resolved(_) => false,
            })
    }

    pub(super) fn alloc_rule(&mut self, rule: LocalTermExpectationRule) -> LocalTermExpectationIdx {
        self.arena.alloc_one(rule)
    }

    pub(super) fn take_effect(
        &mut self,
        rule_idx: LocalTermExpectationIdx,
        effect: LocalTermExpectationEffect,
    ) -> Option<SmallVec<[TermResolveAction; 2]>> {
        self.arena
            .update(rule_idx, |rule| rule.set_resolved(effect.result));
        Some(effect.actions)
    }
}

impl LocalTermRegion {
    pub fn add_expectation_rule(
        &mut self,
        src_expr_idx: ExprIdx,
        expectee: LocalTerm,
        expectation: impl Into<LocalTermExpectation>,
    ) -> OptionLocalTermExpectationIdx {
        self.expectations
            .alloc_rule(LocalTermExpectationRule {
                src_expr_idx,
                expectee: expectee.into(),
                expectation: expectation.into(),
                resolve_progress: LocalTermExpectationResolveProgress::Unresolved,
            })
            .into()
    }
}

impl LocalTermExpectationRule {
    pub(super) fn resolve_expectation(
        &self,
        db: &dyn TermDb,
        unresolved_terms: &mut UnresolvedTerms,
        level: LocalTermResolveLevel,
    ) -> Option<LocalTermExpectationEffect> {
        match self.expectation {
            LocalTermExpectation::ExplicitlyConvertible(ref expectation) => {
                expectation.resolve(db, unresolved_terms, self.expectee, level)
            }
            LocalTermExpectation::ImplicitlyConvertible(exp) => exp.resolve(
                db,
                self.src_expr_idx,
                self.expectee,
                level,
                unresolved_terms,
            ),
            LocalTermExpectation::EqsSort(ref expectation) => {
                expectation.resolve(db, self.expectee, unresolved_terms)
            }
            LocalTermExpectation::FrameVariableType => todo!(),
            LocalTermExpectation::EqsFunctionType(ref expectation) => {
                expectation.resolve(db, self.src_expr_idx, self.expectee, unresolved_terms)
            }
            LocalTermExpectation::InsSort(ref expectation) => {
                expectation.resolve(db, unresolved_terms, self.expectee)
            }
            LocalTermExpectation::EqsExactly(ref expectation) => {
                expectation.resolve(db, unresolved_terms, self.src_expr_idx, self.expectee)
            }
            LocalTermExpectation::AnyOriginal(_) => None,
            LocalTermExpectation::AnyDerived(_) => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
#[salsa::derive_debug_with_db(db = TermDb)]
#[enum_class::from_variants]
pub enum LocalTermExpectation {
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
