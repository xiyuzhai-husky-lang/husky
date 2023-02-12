mod implicit_conversion;

use super::*;
use husky_print_utils::p;
use idx_arena::Arena;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub(crate) enum LocalTermExpectation {
    None,
    /// expect that the type of the expression
    /// is equal to `Type u` for some universe u
    TypeType,
    CastibleAsBool,
    FrameVariableType,
    Return {
        ty: Option<ReducedTerm>,
    },
    ImplicitlyConvertibleTo {
        ty: LocalTerm,
    },
    RefMut {
        lifetime: LocalTerm,
    },
    RitchieCall,
}

impl LocalTermExpectation {
    pub(crate) fn term(self) -> Option<ReducedTerm> {
        match self {
            LocalTermExpectation::None => None,
            LocalTermExpectation::TypeType => None,
            LocalTermExpectation::CastibleAsBool => None,
            LocalTermExpectation::FrameVariableType => None,
            LocalTermExpectation::Return { ty } => ty,
            LocalTermExpectation::ImplicitlyConvertibleTo { ty } => ty.resolved(),
            LocalTermExpectation::RefMut { .. } => None,
            LocalTermExpectation::RitchieCall => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LocalTermExpectationRule {
    src_expr_idx: ExprIdx,
    expectee: LocalTerm,
    variant: LocalTermExpectationRuleVariant,
    resolve_progress: LocalTermExpectationResolveProgress,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct LocalTermExpectationResolved {
    pub(crate) implicit_conversion: LocalTermImplicitConversion,
    pub(crate) local_term: LocalTerm,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LocalTermExpectationResolveProgress {
    Unresolved,
    ResolvedOk(LocalTermExpectationResolved),
    ResolvedErr(LocalTermExpectationResolveError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalTermExpectationResolveError {
    #[error("original {0}")]
    Original(#[from] OriginalLocalTermExpectationError),
    #[error("derived {0}")]
    Derived(#[from] DerivedLocalTermExpectationResolveError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalLocalTermExpectationError {
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedLocalTermExpectationResolveError {
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
            LocalTermExpectationResolveProgress::ResolvedOk(expectation_resolved) => {
                LocalTermExpectationResolveProgress::ResolvedOk(*expectation_resolved)
            }
            LocalTermExpectationResolveProgress::ResolvedErr(_) => {
                LocalTermExpectationResolveProgress::ResolvedErr(
                    DerivedLocalTermExpectationResolveError::Duplication(src).into(),
                )
            }
        }
    }

    pub(crate) fn reduced_term(&self) -> Option<ReducedTerm> {
        match self {
            LocalTermExpectationResolveProgress::Unresolved => None,
            LocalTermExpectationResolveProgress::ResolvedOk(LocalTermExpectationResolved {
                local_term,
                ..
            }) => match local_term {
                LocalTerm::Resolved(reduced_term) => Some(*reduced_term),
                LocalTerm::Unresolved(_) => todo!(),
            },
            LocalTermExpectationResolveProgress::ResolvedErr(_) => None,
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

    fn resolve_ok(&mut self, expectation_resolved: LocalTermExpectationResolved) {
        self.resolve_progress =
            LocalTermExpectationResolveProgress::ResolvedOk(expectation_resolved)
    }

    pub(super) fn resolve_err(&mut self, error: LocalTermExpectationResolveError) {
        self.resolve_progress = LocalTermExpectationResolveProgress::ResolvedErr(error)
    }
}

pub(super) enum LocalTermExpectationEffect {
    ResolvedOk {
        expectation_resolved: LocalTermExpectationResolved,
        actions: Vec<TermResolveAction>,
    },
    ResolvedErr {
        error: LocalTermExpectationResolveError,
    },
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
                LocalTermExpectationResolveProgress::ResolvedOk(_)
                | LocalTermExpectationResolveProgress::ResolvedErr(_) => false,
            })
    }

    pub(super) fn unresolved_expectation_rule_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (LocalTermExpectationRuleIdx, &mut LocalTermExpectationRule)> {
        self.arena
            .indexed_iter_mut_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::ResolvedOk(_)
                | LocalTermExpectationResolveProgress::ResolvedErr(_) => false,
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
        match effect {
            LocalTermExpectationEffect::ResolvedOk {
                expectation_resolved,
                actions,
            } => {
                self.arena
                    .update(rule_idx, |rule| rule.resolve_ok(expectation_resolved));
                Some(actions)
            }
            LocalTermExpectationEffect::ResolvedErr { error } => {
                self.arena.update(rule_idx, |rule| rule.resolve_err(error));
                None
            }
        }
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
                        LocalTermExpectationResolveProgress::ResolvedOk(
                            LocalTermExpectationResolved {
                                implicit_conversion: LocalTermImplicitConversion::None,
                                local_term: term.into(),
                            },
                        )
                    }
                    // MOM
                    term if term == reduced_term_menu.i32() => todo!(),
                    term if term == reduced_term_menu.r32() => todo!(),
                    term => todo!(),
                }
            }
            LocalTermExpectationRuleVariant::ImplicitlyConvertibleTo { dst } => match dst {
                LocalTerm::Resolved(dst) => match (resolved_term, dst) {
                    (resolved_term, dst) if resolved_term == dst => {
                        LocalTermExpectationResolveProgress::ResolvedOk(
                            LocalTermExpectationResolved {
                                implicit_conversion: LocalTermImplicitConversion::None,
                                local_term: dst.into(),
                            },
                        )
                    }
                    _ => todo!(),
                },
                LocalTerm::Unresolved(_) => LocalTermExpectationResolveProgress::Unresolved,
            },
            LocalTermExpectationRuleVariant::Type => match db.term_ty(resolved_term.term()) {
                Ok(term_ty) => match term_ty.term() {
                    Term::Category(cat) => match cat.universe().raw() {
                        0 => todo!(),
                        _ => LocalTermExpectationResolveProgress::ResolvedOk(
                            LocalTermExpectationResolved {
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
                LocalTermExpectationResolveProgress::ResolvedErr(
                    OriginalLocalTermExpectationError::Todo.into(),
                )
            }
            LocalTermExpectationRuleVariant::RitchieCall => {
                todo!()
            }
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
            LocalTermExpectationRuleVariant::ImplicitlyConvertibleTo { dst } => {
                // ad hoc
                LocalTermExpectationResolveProgress::Unresolved
            }
            LocalTermExpectationRuleVariant::Type => {
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
                        LocalTermExpectationResolveProgress::ResolvedErr(
                            OriginalLocalTermExpectationError::Todo.into(),
                        )
                    }
                }
            }
            LocalTermExpectationRuleVariant::RitchieCall => todo!(),
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
                            Some(LocalTermExpectationEffect::ResolvedErr {
                                error: OriginalLocalTermExpectationError::Todo.into(),
                            })
                        }
                    },
                },
            },
            LocalTermExpectationRuleVariant::ImplicitlyConvertibleTo { dst } => {
                self.implicit_conversion_expectation_rule_effect(rule, dst, table, level)
            }
            LocalTermExpectationRuleVariant::Type => todo!(),
            LocalTermExpectationRuleVariant::FrameVariableType => todo!(),
            LocalTermExpectationRuleVariant::RefMut { lifetime } => todo!(),
            LocalTermExpectationRuleVariant::RitchieCall => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum LocalTermExpectationRuleVariant {
    AsBool,
    ImplicitlyConvertibleTo { dst: LocalTerm },
    Type,
    FrameVariableType,
    RefMut { lifetime: LocalTerm },
    RitchieCall,
}
