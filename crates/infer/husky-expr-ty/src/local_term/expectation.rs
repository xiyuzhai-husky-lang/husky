mod explicit_conversion;
mod implicit_conversion;
mod ref_mut;
mod ritchie_call;
mod sort;
mod ty;

pub(crate) use explicit_conversion::*;
pub(crate) use implicit_conversion::*;
pub(crate) use ref_mut::*;
pub(crate) use ritchie_call::*;
pub(crate) use sort::*;
pub(crate) use ty::*;

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
    OkSort {
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
            LocalTermExpectationResult::OkSort {
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
            LocalTermExpectationResult::OkSort {
                implicit_conversion,
                local_term,
            } => LocalTermExpectationResult::OkSort {
                implicit_conversion: *implicit_conversion,
                local_term: *local_term,
            },
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
            LocalTermExpectation::AsBool => match rule.expectee {
                LocalTerm::Resolved(_) => todo!(),
                LocalTerm::Unresolved(expectee) => match level {
                    LocalTermResolveLevel::Weak => None,
                    LocalTermResolveLevel::Strong => match table[expectee].unresolved_term() {
                        UnresolvedTerm::ImplicitSymbol(_) => {
                            match table[expectee].unresolved_term() {
                                UnresolvedTerm::ImplicitSymbol(implicit_symbol) => {
                                    let src_expr_idx = rule.src_expr_idx();
                                    p!(self.expr_region_data()[src_expr_idx]);
                                    todo!()
                                }
                                _ => unreachable!(),
                            };
                            todo!()
                        }
                        UnresolvedTerm::TypeApplication { ty, arguments } => {
                            Some(LocalTermExpectationResultM {
                                result: LocalTermExpectationResult::Err(
                                    OriginalLocalTermExpectationError::Todo.into(),
                                ),
                                actions: vec![],
                            })
                        }
                    },
                },
            },
            LocalTermExpectation::ImplicitlyConversion { destination: dst } => {
                self.resolve_implicit_conversion(rule, dst, table, level)
            }
            LocalTermExpectation::Sort => todo!(),
            LocalTermExpectation::FrameVariableType => todo!(),
            LocalTermExpectation::RefMut { lifetime } => todo!(),
            LocalTermExpectation::RitchieCall => todo!(),
            LocalTermExpectation::Type => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum LocalTermExpectation {
    AsBool,
    ImplicitlyConversion { destination: LocalTerm },
    Type,
    Sort,
    FrameVariableType,
    RefMut { lifetime: LocalTerm },
    RitchieCall,
}
