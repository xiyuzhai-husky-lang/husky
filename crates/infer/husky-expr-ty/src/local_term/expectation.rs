mod eqs_ref_mut_application;
mod eqs_ritchie_call_ty;
mod eqs_sort;
mod explicit_convertible;
mod implicit_convertible;
mod ins_sort;

pub(crate) use eqs_ref_mut_application::*;
pub(crate) use eqs_ritchie_call_ty::*;
pub(crate) use eqs_sort::*;
pub(crate) use explicit_convertible::*;
pub(crate) use implicit_convertible::*;
pub(crate) use ins_sort::*;

use super::*;
use husky_print_utils::p;
use idx_arena::Arena;
use thiserror::Error;

pub(crate) trait ExpectLocalTerm: Clone + Into<LocalTermExpectation> {
    type Result: Into<LocalTermExpectationResult>;

    fn destination(&self) -> Option<LocalTerm>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct LocalTermExpectationEntry {
    src_expr_idx: ExprIdx,
    expectee: LocalTerm,
    expectation: LocalTermExpectation,
    resolve_progress: LocalTermExpectationResolveProgress,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LocalTermExpectationResult {
    OkNoExpectation {
        implicit_conversion: LocalTermImplicitConversion,
        local_term: LocalTerm,
    },
    OkExplicitConversion {
        implicit_conversion: LocalTermImplicitConversion,
        local_term: LocalTerm,
    },
    OkImplicitConversion {
        implicit_conversion: LocalTermImplicitConversion,
        local_term: LocalTerm,
    },
    OkInsSort(ExpectInsSortResult),
    OkEqsSort {
        implicit_conversion: LocalTermImplicitConversion,
        local_term: LocalTerm,
    },
    Err(LocalTermExpectationError),
}
impl LocalTermExpectationResult {
    fn reduced_term(&self) -> Option<ReducedTerm> {
        match self {
            LocalTermExpectationResult::OkNoExpectation { .. } => todo!(),
            LocalTermExpectationResult::OkExplicitConversion { .. } => todo!(),
            LocalTermExpectationResult::OkImplicitConversion {
                implicit_conversion,
                local_term,
            } => todo!(),
            LocalTermExpectationResult::Err(_) => todo!(),
            LocalTermExpectationResult::OkInsSort(result) => result.reduced_term(),
            LocalTermExpectationResult::OkEqsSort {
                implicit_conversion,
                local_term,
            } => todo!(),
        }
    }

    fn duplicate(&self, src: LocalTermExpectationRuleIdx) -> LocalTermExpectationResult {
        match self {
            LocalTermExpectationResult::OkNoExpectation { .. } => todo!(),
            LocalTermExpectationResult::OkImplicitConversion {
                implicit_conversion,
                local_term,
            } => LocalTermExpectationResult::OkImplicitConversion {
                implicit_conversion: *implicit_conversion,
                local_term: *local_term,
            },
            LocalTermExpectationResult::Err(_) => LocalTermExpectationResult::Err(
                DerivedLocalTermExpectationError::Duplication(src).into(),
            ),
            LocalTermExpectationResult::OkExplicitConversion {
                implicit_conversion,
                local_term,
            } => LocalTermExpectationResult::OkExplicitConversion {
                implicit_conversion: *implicit_conversion,
                local_term: *local_term,
            },
            LocalTermExpectationResult::OkInsSort(result) => result.clone().into(),
            LocalTermExpectationResult::OkEqsSort {
                implicit_conversion,
                local_term,
            } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LocalTermExpectationResolveProgress {
    Unresolved,
    Resolved(LocalTermExpectationResult),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalTermExpectationError {
    #[error("original {0}")]
    Original(#[from] OriginalLocalTermExpectationError),
    #[error("derived {0}")]
    Derived(#[from] DerivedLocalTermExpectationError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalLocalTermExpectationError {
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedLocalTermExpectationError {
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
            LocalTermExpectationResolveProgress::Resolved(expectation_resolved) => {
                LocalTermExpectationResolveProgress::Resolved(expectation_resolved.duplicate(src))
            }
        }
    }

    pub(crate) fn reduced_term(&self) -> Option<ReducedTerm> {
        match self {
            LocalTermExpectationResolveProgress::Unresolved => None,
            LocalTermExpectationResolveProgress::Resolved(result) => result.reduced_term(),
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

    pub(crate) fn target(&self) -> LocalTerm {
        self.expectee
    }

    pub fn src_expr_idx(&self) -> ExprIdx {
        self.src_expr_idx
    }

    pub(crate) fn set_resolved(&mut self, result: LocalTermExpectationResult) {
        self.resolve_progress = LocalTermExpectationResolveProgress::Resolved(result)
    }
}

pub(super) struct LocalTermExpectationResultM {
    pub(super) result: LocalTermExpectationResult,
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
        effect: LocalTermExpectationResultM,
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
    ) -> Option<LocalTermExpectationResultM> {
        let table = self.local_term_table();
        match rule.expectation {
            LocalTermExpectation::ImplicitlyConversion { destination } => {
                self.resolve_implicit_conversion_expectation(rule.expectee, destination, level)
            }
            LocalTermExpectation::EqsSort { smallest_universe } => {
                todo!()
            }
            LocalTermExpectation::FrameVariableType => todo!(),
            LocalTermExpectation::EqsRefMutApplication { lifetime } => todo!(),
            LocalTermExpectation::EqsRitchieCallTy => todo!(),
            LocalTermExpectation::InsSort { smallest_universe } => {
                self.resolve_ins_sort_expectation(smallest_universe, rule.expectee)
            }
            LocalTermExpectation::EqsSort { smallest_universe } => {
                self.resolve_eq_sort_expectation(smallest_universe, rule.expectee)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
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
    EqsRitchieCallTy,
}
