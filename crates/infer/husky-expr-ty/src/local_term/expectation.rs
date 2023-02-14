mod eqs_exactly;
mod eqs_ref_mut_application;
mod eqs_ritchie_call_ty;
mod eqs_sort;
mod explicitly_convertible;
mod implicitly_convertible;
mod ins_sort;

pub(crate) use eqs_exactly::*;
pub(crate) use eqs_ref_mut_application::*;
pub(crate) use eqs_ritchie_call_ty::*;
pub(crate) use eqs_sort::*;
pub(crate) use explicitly_convertible::*;
pub(crate) use implicitly_convertible::*;
pub(crate) use ins_sort::*;

use super::*;
use husky_print_utils::p;
use idx_arena::Arena;
use thiserror::Error;

pub(crate) trait ExpectLocalTerm: Clone + Into<LocalTermExpectation> {
    type ResolvedOk: ExpectLocalTermResolvedOk;

    fn destination(&self) -> Option<LocalTerm>;
}

pub(crate) trait ExpectLocalTermResolvedOk: Into<LocalTermExpectationResolvedOk> {
    fn destination(&self) -> LocalTerm;

    /// will panic if not right
    fn downcast_ref(resolved_ok: &LocalTermExpectationResolvedOk) -> &Self;
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct LocalTermExpectationEntry {
    src_expr_idx: ExprIdx,
    expectee: LocalTerm,
    expectation: LocalTermExpectation,
    resolve_progress: LocalTermExpectationResolveProgress,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) enum LocalTermExpectationResolvedOk {
    ExplicitlyConvertible(ExpectExplicitlyConvertibleResolvedOk),
    ImplicitlyConvertible(ExpectImplicitlyConvertibleResolvedOk),
    InsSort(ExpectInsSortResolvedOk),
    EqsSort(ExpectEqsSortResolvedOk),
    EqsExactly(ExpectEqsExactlyResolvedOk),
    EqsRefMutApplication(ExpectEqsRefMutApplicationResolvedOk),
    EqsRitchieCallType(ExpectEqsRitchieCallTypeResolvedOk),
}

impl LocalTermExpectationResolvedOk {
    fn resolved(&self) -> Option<ReducedTerm> {
        match self {
            LocalTermExpectationResolvedOk::ExplicitlyConvertible(_) => todo!(),
            LocalTermExpectationResolvedOk::ImplicitlyConvertible(_) => todo!(),
            LocalTermExpectationResolvedOk::InsSort(result) => result.resolved(),
            LocalTermExpectationResolvedOk::EqsSort(_) => todo!(),
            LocalTermExpectationResolvedOk::EqsExactly(result) => result.resolved(),
            LocalTermExpectationResolvedOk::EqsRefMutApplication(_) => todo!(),
            LocalTermExpectationResolvedOk::EqsRitchieCallType(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) enum LocalTermExpectationResolveProgress {
    Unresolved,
    Resolved(LocalTermExpectationResult<LocalTermExpectationResolvedOk>),
}

impl LocalTermExpectationResolveProgress {
    pub(crate) fn resolved_ok<R: ExpectLocalTermResolvedOk>(&self) -> Option<&R> {
        match self {
            LocalTermExpectationResolveProgress::Unresolved => None,
            LocalTermExpectationResolveProgress::Resolved(Ok(resolved_ok)) => {
                Some(R::downcast_ref(resolved_ok))
            }
            LocalTermExpectationResolveProgress::Resolved(Err(_)) => None,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub enum LocalTermExpectationError {
    #[error("original {0}")]
    Original(#[from] OriginalLocalTermExpectationError),
    #[error("derived {0}")]
    Derived(#[from] DerivedLocalTermExpectationError),
}

impl From<TypeError> for LocalTermExpectationError {
    fn from(error: TypeError) -> Self {
        match error {
            TypeError::Original(error) => LocalTermExpectationError::Original(error.into()),
            TypeError::Derived(error) => LocalTermExpectationError::Derived(error.into()),
        }
    }
}

pub type LocalTermExpectationResult<T> = Result<T, LocalTermExpectationError>;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub enum OriginalLocalTermExpectationError {
    #[error("{0}")]
    Type(#[from] OriginalTypeError),
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub enum DerivedLocalTermExpectationError {
    #[error("{0}")]
    Type(#[from] DerivedTypeError),
    #[error("target substitution failure")]
    TargetSubstitutionFailure,
    #[error("duplication")]
    Duplication(LocalTermExpectationRuleIdx),
    #[error("unresolved local term")]
    UnresolvedLocalTerm,
}

impl LocalTermExpectationResolveProgress {
    // it will use derived type error
    pub(crate) fn duplicate(&self, src: LocalTermExpectationRuleIdx) -> Self {
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

    pub(crate) fn reduced_term(&self) -> Option<ReducedTerm> {
        match self {
            LocalTermExpectationResolveProgress::Unresolved
            | LocalTermExpectationResolveProgress::Resolved(Err(_)) => None,
            LocalTermExpectationResolveProgress::Resolved(Ok(result)) => result.resolved(),
        }
    }
}

impl LocalTermExpectationEntry {
    pub(crate) fn variant(&self) -> &LocalTermExpectation {
        &self.expectation
    }

    pub(crate) fn resolve_progress(&self) -> &LocalTermExpectationResolveProgress {
        &self.resolve_progress
    }

    pub(crate) fn expectee(&self) -> LocalTerm {
        self.expectee
    }

    pub fn src_expr_idx(&self) -> ExprIdx {
        self.src_expr_idx
    }

    pub(crate) fn set_resolved(
        &mut self,
        result: LocalTermExpectationResult<LocalTermExpectationResolvedOk>,
    ) {
        self.resolve_progress = LocalTermExpectationResolveProgress::Resolved(result)
    }
}

pub(super) struct LocalTermExpectationResolvedOkM {
    pub(super) result: LocalTermExpectationResult<LocalTermExpectationResolvedOk>,
    pub(super) actions: Vec<TermResolveAction>,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub(crate) struct LocalTermExpectations {
    arena: Arena<LocalTermExpectationEntry>,
    first_unresolved_expectation: usize,
}

impl std::ops::Index<LocalTermExpectationRuleIdx> for LocalTermExpectations {
    type Output = LocalTermExpectationEntry;

    fn index(&self, index: LocalTermExpectationRuleIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl LocalTermExpectations {
    pub(super) fn unresolved_rule_iter(
        &self,
    ) -> impl Iterator<Item = (LocalTermExpectationRuleIdx, &LocalTermExpectationEntry)> {
        self.arena
            .indexed_iter_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::Resolved(_) => false,
            })
    }

    pub(super) fn unresolved_expectation_rule_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (LocalTermExpectationRuleIdx, &mut LocalTermExpectationEntry)> {
        self.arena
            .indexed_iter_mut_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::Resolved(_) => false,
            })
    }

    pub(super) fn alloc_rule(
        &mut self,
        rule: LocalTermExpectationEntry,
    ) -> LocalTermExpectationRuleIdx {
        self.arena.alloc_one(rule)
    }

    pub(super) fn take_effect(
        &mut self,
        rule_idx: LocalTermExpectationRuleIdx,
        effect: LocalTermExpectationResolvedOkM,
    ) -> Option<Vec<TermResolveAction>> {
        self.arena
            .update(rule_idx, |rule| rule.set_resolved(effect.result));
        Some(effect.actions)
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn add_expectation_rule(
        &mut self,
        src_expr_idx: ExprIdx,
        target: LocalTerm,
        expectation: impl ExpectLocalTerm,
    ) -> OptionLocalTermExpectationIdx {
        self.local_term_table_mut()
            .expectation_rules
            .alloc_rule(LocalTermExpectationEntry {
                src_expr_idx,
                expectee: target.into(),
                expectation: expectation.into(),
                resolve_progress: LocalTermExpectationResolveProgress::Unresolved,
            })
            .into()
    }
    pub(super) fn resolve_expectation(
        &self,
        rule: &LocalTermExpectationEntry,
        level: LocalTermResolveLevel,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        let table = self.local_term_table();
        match rule.expectation {
            LocalTermExpectation::ImplicitlyConversion { destination } => {
                self.resolve_implicit_conversion_expectation(rule.expectee, destination, level)
            }
            LocalTermExpectation::EqsSort { smallest_universe } => {
                self.resolve_eqs_sort_expectation(rule.expectee, smallest_universe)
            }
            LocalTermExpectation::FrameVariableType => todo!(),
            LocalTermExpectation::EqsRefMutApplication { lifetime } => {
                self.resolve_eqs_ref_mut_application_expectation(rule.expectee, lifetime)
            }
            LocalTermExpectation::EqsRitchieCallTy => {
                self.resolve_eqs_richie_call_ty(rule.expectee)
            }
            LocalTermExpectation::InsSort { smallest_universe } => {
                self.resolve_ins_sort_expectation(smallest_universe, rule.expectee)
            }
            LocalTermExpectation::EqsExactly { destination } => {
                self.resolve_eqs_exactly(rule.expectee, destination)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) enum LocalTermExpectation {
    ImplicitlyConversion {
        destination: LocalTerm,
    },
    /// expect term to be an instance of Type u for some universe
    InsSort {
        smallest_universe: TermUniverse,
    },
    EqsSort {
        smallest_universe: TermUniverse,
    },
    FrameVariableType,
    EqsRefMutApplication {
        lifetime: LocalTerm,
    },
    EqsExactly {
        destination: LocalTerm,
    },
    EqsRitchieCallTy,
}
