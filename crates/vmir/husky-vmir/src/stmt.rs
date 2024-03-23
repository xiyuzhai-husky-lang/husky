mod ifelse;
mod r#loop;
mod r#match;

use crate::{
    coersion::VmirCoersion,
    eval::EvalVmir,
    expr::VmirExprIdx,
    pattern::VmirPatternIdx,
    stmt::{
        ifelse::{VmirElifBranchs, VmirElseBranch, VmirIfBranch},
        r#match::VmirCaseBranches,
    },
    *,
};
use husky_expr::stmt::ConditionConversion;
use husky_hir_eager_expr::{HirEagerCondition, HirEagerStmtData, HirEagerStmtIdxRange};
use husky_task_interface::vm_control_flow::{LinkageImplVmControlFlow, VmControlFlow};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirStmtData<LinkageImpl: IsLinkageImpl> {
    Let,
    Return {
        result: VmirExprIdx<LinkageImpl>,
        coersion: VmirCoersion,
    },
    Require {
        condition: VmirCondition<LinkageImpl>,
    },
    Assert {
        condition: VmirCondition<LinkageImpl>,
    },
    Break,
    Eval {
        expr: VmirExprIdx<LinkageImpl>,
        coersion: Option<VmirCoersion>,
        discarded: bool,
    },
    ForBetween {
        stmts: VmirStmtIdxRange<LinkageImpl>,
    },
    Forext {
        stmts: VmirStmtIdxRange<LinkageImpl>,
    },
    ForIn {
        stmts: VmirStmtIdxRange<LinkageImpl>,
    },
    While {
        condition: VmirCondition<LinkageImpl>,
        stmts: VmirStmtIdxRange<LinkageImpl>,
    },
    DoWhile {
        condition: VmirCondition<LinkageImpl>,
        stmts: VmirStmtIdxRange<LinkageImpl>,
    },
    IfElse {
        if_branch: VmirIfBranch<LinkageImpl>,
        elif_branches: VmirElifBranchs<LinkageImpl>,
        else_branch: Option<VmirElseBranch<LinkageImpl>>,
    },
    Match {
        opd: VmirExprIdx<LinkageImpl>,
        case_branches: VmirCaseBranches<LinkageImpl>,
    },
}

pub type VmirStmtArena<LinkageImpl> = Arena<VmirStmtData<LinkageImpl>>;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VmirStmtIdx<LinkageImpl: IsLinkageImpl>(ArenaIdx<VmirStmtData<LinkageImpl>>);

impl<LinkageImpl: IsLinkageImpl> std::ops::Deref for VmirStmtIdx<LinkageImpl> {
    type Target = ArenaIdx<VmirStmtData<LinkageImpl>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VmirStmtIdxRange<LinkageImpl: IsLinkageImpl>(ArenaIdxRange<VmirStmtData<LinkageImpl>>);

impl<LinkageImpl: IsLinkageImpl> IntoIterator for VmirStmtIdxRange<LinkageImpl> {
    type Item = VmirStmtIdx<LinkageImpl>;

    type IntoIter = impl Iterator<Item = VmirStmtIdx<LinkageImpl>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(VmirStmtIdx)
    }
}

impl<LinkageImpl: IsLinkageImpl> VmirStmtIdxRange<LinkageImpl> {
    fn split_last(self) -> (Self, VmirStmtIdx<LinkageImpl>) {
        let (non_lasts, last) = self.0.split_last();
        (Self(non_lasts), VmirStmtIdx(last))
    }
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for HirEagerStmtIdxRange {
    type Output = VmirStmtIdxRange<LinkageImpl>;

    fn to_vmir<Linktime>(self, builder: &mut crate::builder::VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        let stmts = self
            .into_iter()
            .map(|stmt| match builder.hir_eager_stmt_arena()[stmt] {
                HirEagerStmtData::Let {
                    pattern,
                    contract,
                    initial_value,
                    coersion,
                } => VmirStmtData::Let,
                HirEagerStmtData::Return { result, coersion } => VmirStmtData::Return {
                    result: result.to_vmir(builder),
                    coersion: coersion.to_vmir(builder),
                },
                HirEagerStmtData::Require { ref condition } => VmirStmtData::Require {
                    condition: condition.to_vmir(builder),
                },
                HirEagerStmtData::Assert { ref condition } => VmirStmtData::Assert {
                    condition: condition.to_vmir(builder),
                },
                HirEagerStmtData::Break => VmirStmtData::Break,
                HirEagerStmtData::Eval {
                    expr,
                    coersion,
                    discarded,
                } => VmirStmtData::Eval {
                    expr: expr.to_vmir(builder),
                    coersion: coersion.to_vmir(builder),
                    discarded,
                },
                HirEagerStmtData::ForBetween {
                    ref particulars,
                    stmts,
                } => VmirStmtData::ForBetween {
                    stmts: stmts.to_vmir(builder),
                },
                HirEagerStmtData::Forext {
                    ref particulars,
                    stmts,
                } => VmirStmtData::Forext {
                    stmts: stmts.to_vmir(builder),
                },
                HirEagerStmtData::ForIn {
                    ref condition,
                    stmts,
                } => VmirStmtData::ForIn {
                    stmts: stmts.to_vmir(builder),
                },
                HirEagerStmtData::While {
                    ref condition,
                    stmts,
                } => VmirStmtData::While {
                    condition: condition.to_vmir(builder),
                    stmts: stmts.to_vmir(builder),
                },
                HirEagerStmtData::DoWhile {
                    ref condition,
                    stmts,
                } => VmirStmtData::DoWhile {
                    condition: condition.to_vmir(builder),
                    stmts: stmts.to_vmir(builder),
                },
                HirEagerStmtData::IfElse {
                    ref if_branch,
                    ref elif_branches,
                    ref else_branch,
                } => VmirStmtData::IfElse {
                    if_branch: if_branch.to_vmir(builder),
                    elif_branches: elif_branches.to_vmir(builder),
                    else_branch: else_branch.to_vmir(builder),
                },
                HirEagerStmtData::Match {
                    ref opd,
                    ref case_branches,
                } => VmirStmtData::Match {
                    opd: opd.to_vmir(builder),
                    case_branches: case_branches.to_vmir(builder),
                },
            })
            .collect();
        VmirStmtIdxRange(builder.alloc_stmts(stmts))
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VmirCondition<LinkageImpl: IsLinkageImpl> {
    /// `be` condition with syntactically correct pattern.
    /// This requires special handling for many cases.
    Be {
        opd: VmirExprIdx<LinkageImpl>,
        pattern: VmirPatternIdx<LinkageImpl>,
    },
    /// all other conditions.
    /// for simplicity, `be` with a syntactically broken pattern is also included in there
    Other {
        opd: VmirExprIdx<LinkageImpl>,
        conversion: VmirConditionConversion<LinkageImpl>,
    },
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VmirConditionConversion<LinkageImpl> {
    None,
    IntToBool,
    Todo(LinkageImpl),
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for &HirEagerCondition {
    type Output = VmirCondition<LinkageImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        match *self {
            HirEagerCondition::Be { opd, ref pattern } => VmirCondition::Be {
                opd: opd.to_vmir(builder),
                pattern: pattern.pattern.to_vmir(builder),
            },
            HirEagerCondition::Other { opd, conversion } => VmirCondition::Other {
                opd: opd.to_vmir(builder),
                conversion: conversion.to_vmir(builder),
            },
        }
    }
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for ConditionConversion {
    type Output = VmirConditionConversion<LinkageImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        match self {
            ConditionConversion::None => VmirConditionConversion::None,
            ConditionConversion::IntToBool(_) => VmirConditionConversion::IntToBool,
        }
    }
}

impl<LinkageImpl: IsLinkageImpl> VmirStmtIdxRange<LinkageImpl> {
    pub fn eval<'comptime>(
        self,
        ctx: &mut impl EvalVmir<'comptime, LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl> {
        ctx.eval_stmts(self, |ctx| self.eval_aux(ctx))
    }

    pub fn eval_aux<'comptime>(
        self,
        ctx: &mut impl EvalVmir<'comptime, LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl> {
        let (non_lasts, last) = self.split_last();
        last.eval(ctx)
    }
}

impl<LinkageImpl: IsLinkageImpl> VmirStmtIdx<LinkageImpl> {
    pub fn eval<'comptime>(
        self,
        ctx: &mut impl EvalVmir<'comptime, LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl> {
        ctx.eval_stmt(self, |ctx| self.eval_aux(ctx))
    }

    pub fn eval_aux<'comptime>(
        self,
        ctx: &mut impl EvalVmir<'comptime, LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl> {
        use VmControlFlow::*;

        match *self.entry(ctx.vmir_stmt_arena()) {
            VmirStmtData::Let => todo!(),
            VmirStmtData::Return { result, coersion } => todo!(),
            VmirStmtData::Require { condition } => todo!(),
            VmirStmtData::Assert { condition } => todo!(),
            VmirStmtData::Break => todo!(),
            VmirStmtData::Eval {
                expr,
                coersion,
                discarded,
            } => {
                let result = expr.eval(ctx)?;
                match discarded {
                    true => Continue(().into()),
                    false => Continue(result),
                }
            }
            VmirStmtData::ForBetween { stmts } => todo!(),
            VmirStmtData::Forext { stmts } => todo!(),
            VmirStmtData::ForIn { stmts } => todo!(),
            VmirStmtData::While { condition, stmts } => todo!(),
            VmirStmtData::DoWhile { condition, stmts } => todo!(),
            VmirStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => todo!(),
            VmirStmtData::Match {
                opd,
                ref case_branches,
            } => todo!(),
        }
    }
}
