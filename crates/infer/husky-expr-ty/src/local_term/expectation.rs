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

pub(crate) trait ExpectLocalTerm: Clone + Into<LocalTermExpectationRuleVariant> {
    type Result: Into<LocalTermExpectationResult>;

    fn destination(&self) -> Option<LocalTerm>;
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// #[non_exhaustive]
// pub(crate) enum LocalTermExpectation {
//     Type,
//     /// expect that the type of the expression
//     /// is equal to `Type u` for some universe u
//     Sort,
//     CastibleAsBool,
//     FrameVariableType,
//     Return {
//         ty: Option<ReducedTerm>,
//     },
//     ImplicitlyConvertibleTo {
//         ty: LocalTerm,
//     },
//     RefMut {
//         lifetime: LocalTerm,
//     },
//     RitchieCall,
// }

// impl LocalTermExpectation {
//     pub(crate) fn term(self) -> Option<ReducedTerm> {
//         match self {
//             ExpectType => None,
//             ExpectSort => None,
//             ExpectExplicitConversion::Bool => None,
//             LocalTermExpectation::FrameVariableType => None,
//             LocalTermExpectation::Return { ty } => ty,
//             ExpectImplicitConversion { ty } => ty.resolved(),
//             LocalTermExpectation::RefMut { .. } => None,
//             LocalTermExpectation::RitchieCall => None,
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq)]
pub struct LocalTermExpectationRule {
    src_expr_idx: ExprIdx,
    expectee: LocalTerm,
    variant: LocalTermExpectationRuleVariant,
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

impl LocalTermExpectationRule {
    pub(crate) fn variant(&self) -> &LocalTermExpectationRuleVariant {
        &self.variant
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

pub(super) struct LocalTermExpectationEffect {
    pub(super) result: LocalTermExpectationResult,
    pub(super) actions: Vec<TermResolveAction>,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub(crate) struct LocalTermExpectationRules {
    arena: Arena<LocalTermExpectationRule>,
    first_unresolved_expectation: usize,
}

impl std::ops::Index<LocalTermExpectationRuleIdx> for LocalTermExpectationRules {
    type Output = LocalTermExpectationRule;

    fn index(&self, index: LocalTermExpectationRuleIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl LocalTermExpectationRules {
    pub(super) fn unresolved_rule_iter(
        &self,
    ) -> impl Iterator<Item = (LocalTermExpectationRuleIdx, &LocalTermExpectationRule)> {
        self.arena
            .indexed_iter_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::Resolved(_) => false,
            })
    }

    pub(super) fn unresolved_expectation_rule_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (LocalTermExpectationRuleIdx, &mut LocalTermExpectationRule)> {
        self.arena
            .indexed_iter_mut_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::Resolved(_) => false,
            })
    }

    pub(super) fn alloc_rule(
        &mut self,
        rule: LocalTermExpectationRule,
    ) -> LocalTermExpectationRuleIdx {
        self.arena.alloc_one(rule)
    }

    pub(super) fn take_effect(
        &mut self,
        rule_idx: LocalTermExpectationRuleIdx,
        effect: LocalTermExpectationEffect,
    ) -> Option<Vec<TermResolveAction>> {
        self.arena
            .update(rule_idx, |rule| rule.set_resolved(effect.result));
        Some(effect.actions)
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn new_expectation_rule(
        &self,
        src_expr_idx: ExprIdx,
        target: LocalTerm,
        variant: LocalTermExpectationRuleVariant,
    ) -> LocalTermExpectationRule {
        match target {
            LocalTerm::Resolved(target) => {
                self.new_resolved_expectation_rule(src_expr_idx, target, variant)
            }
            LocalTerm::Unresolved(target) => {
                self.new_unresolved_expectation_rule(src_expr_idx, target, variant)
            }
        }
    }

    fn new_resolved_expectation_rule(
        &self,
        src_expr_idx: ExprIdx,
        resolved_term: ReducedTerm,
        variant: LocalTermExpectationRuleVariant,
    ) -> LocalTermExpectationRule {
        let db = self.db();
        let reduced_term_menu = self.reduced_term_menu();
        let resolve_progress = match variant {
            LocalTermExpectationRuleVariant::AsBool => {
                match resolved_term {
                    term if term == reduced_term_menu.bool() => {
                        LocalTermExpectationResolveProgress::Resolved(
                            LocalTermExpectationResult::OkExplicitConversion {
                                local_term: term.into(),
                                implicit_conversion: LocalTermImplicitConversion::None,
                            },
                        )
                    }
                    // MOM
                    term if term == reduced_term_menu.i32() => todo!(),
                    term if term == reduced_term_menu.r32() => todo!(),
                    term => todo!(),
                }
            }
            LocalTermExpectationRuleVariant::ImplicitlyConversion { destination: dst } => match dst
            {
                LocalTerm::Resolved(dst) => match (resolved_term, dst) {
                    (resolved_term, dst) if resolved_term == dst => {
                        LocalTermExpectationResolveProgress::Resolved(
                            LocalTermExpectationResult::OkImplicitConversion {
                                local_term: dst.into(),
                                implicit_conversion: LocalTermImplicitConversion::None,
                            },
                        )
                    }
                    _ => todo!(),
                },
                LocalTerm::Unresolved(_) => LocalTermExpectationResolveProgress::Unresolved,
            },
            LocalTermExpectationRuleVariant::Sort => match db.term_ty(resolved_term.term()) {
                Ok(term_ty) => match term_ty.term() {
                    Term::Category(cat) => match cat.universe().raw() {
                        0 => todo!(),
                        _ => LocalTermExpectationResolveProgress::Resolved(
                            LocalTermExpectationResult::OkSort {
                                implicit_conversion: LocalTermImplicitConversion::None,
                                local_term: resolved_term.into(),
                            },
                        ),
                    },
                    _ => todo!(),
                },
                Err(_) => todo!(),
            },
            LocalTermExpectationRuleVariant::FrameVariableType => todo!(),
            LocalTermExpectationRuleVariant::RefMut { lifetime } => {
                // ad hoc
                LocalTermExpectationResolveProgress::Resolved(LocalTermExpectationResult::Err(
                    OriginalLocalTermExpectationError::Todo.into(),
                ))
            }
            LocalTermExpectationRuleVariant::RitchieCall => {
                todo!()
            }
            LocalTermExpectationRuleVariant::Type => todo!(),
        };
        LocalTermExpectationRule {
            src_expr_idx,
            expectee: resolved_term.into(),
            variant,
            resolve_progress,
        }
    }
    fn new_unresolved_expectation_rule(
        &self,
        src_expr_idx: ExprIdx,
        target: UnresolvedTermIdx,
        variant: LocalTermExpectationRuleVariant,
    ) -> LocalTermExpectationRule {
        let resolve_progress = match variant {
            LocalTermExpectationRuleVariant::AsBool => {
                // ad hoc
                LocalTermExpectationResolveProgress::Unresolved
            }
            LocalTermExpectationRuleVariant::ImplicitlyConversion { destination: dst } => {
                // ad hoc
                LocalTermExpectationResolveProgress::Unresolved
            }
            LocalTermExpectationRuleVariant::Sort => {
                // ad hoc
                LocalTermExpectationResolveProgress::Unresolved
            }
            LocalTermExpectationRuleVariant::FrameVariableType => {
                // ad hoc
                LocalTermExpectationResolveProgress::Unresolved
            }
            LocalTermExpectationRuleVariant::RefMut { lifetime } => {
                match self.local_term_table()[target].unresolved_term() {
                    UnresolvedTerm::ImplicitSymbol(_) => todo!(),
                    UnresolvedTerm::TypeApplication { ty, arguments }
                        if *ty == self.entity_path_menu().ref_mut_ty_path() =>
                    {
                        todo!()
                    }
                    UnresolvedTerm::TypeApplication { ty, arguments } => {
                        LocalTermExpectationResolveProgress::Resolved(
                            LocalTermExpectationResult::Err(
                                OriginalLocalTermExpectationError::Todo.into(),
                            ),
                        )
                    }
                }
            }
            LocalTermExpectationRuleVariant::RitchieCall => todo!(),
            LocalTermExpectationRuleVariant::Type => todo!(),
        };
        LocalTermExpectationRule {
            src_expr_idx,
            expectee: target.into(),
            variant,
            resolve_progress,
        }
    }

    pub(super) fn expectation_rule_effect(
        &self,
        rule: &LocalTermExpectationRule,
        level: LocalTermResolveLevel,
    ) -> Option<LocalTermExpectationEffect> {
        let table = self.local_term_table();
        match rule.variant {
            LocalTermExpectationRuleVariant::AsBool => match rule.expectee {
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
                            Some(LocalTermExpectationEffect {
                                result: LocalTermExpectationResult::Err(
                                    OriginalLocalTermExpectationError::Todo.into(),
                                ),
                                actions: vec![],
                            })
                        }
                    },
                },
            },
            LocalTermExpectationRuleVariant::ImplicitlyConversion { destination: dst } => {
                self.implicit_conversion_expectation_rule_effect(rule, dst, table, level)
            }
            LocalTermExpectationRuleVariant::Sort => todo!(),
            LocalTermExpectationRuleVariant::FrameVariableType => todo!(),
            LocalTermExpectationRuleVariant::RefMut { lifetime } => todo!(),
            LocalTermExpectationRuleVariant::RitchieCall => todo!(),
            LocalTermExpectationRuleVariant::Type => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum LocalTermExpectationRuleVariant {
    AsBool,
    ImplicitlyConversion { destination: LocalTerm },
    Type,
    Sort,
    FrameVariableType,
    RefMut { lifetime: LocalTerm },
    RitchieCall,
}
