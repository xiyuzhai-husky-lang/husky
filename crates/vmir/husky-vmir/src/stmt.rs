mod ifelse;
mod r#loop;
mod r#match;

use crate::{
    coercion::VmirCoercion,
    eval::EvalVmir,
    expr::VmirExprIdx,
    pattern::VmirPattern,
    stmt::{
        ifelse::{VmirElifBranchs, VmirElseBranch, VmirIfBranch},
        r#match::VmirCaseBranches,
    },
    *,
};
use husky_expr::stmt::ConditionConversion;
use husky_hir_eager_expr::{HirEagerCondition, HirEagerStmtData, HirEagerStmtIdxRange};
use husky_linket_impl::LinketImplVmControlFlow;
use husky_value_interface::vm_control_flow::VmControlFlow;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirStmtData<LinketImpl: IsLinketImpl> {
    Let {
        pattern: VmirPattern<LinketImpl>,
        initial_value: VmirExprIdx<LinketImpl>,
        coercion: Option<VmirCoercion>,
    },
    Return {
        result: VmirExprIdx<LinketImpl>,
        coercion: VmirCoercion,
    },
    Require {
        condition: VmirCondition<LinketImpl>,
    },
    Assert {
        condition: VmirCondition<LinketImpl>,
    },
    Break,
    Eval {
        expr: VmirExprIdx<LinketImpl>,
        coercion: Option<VmirCoercion>,
        discarded: bool,
    },
    ForBetween {
        stmts: VmirStmtIdxRange<LinketImpl>,
    },
    Forext {
        stmts: VmirStmtIdxRange<LinketImpl>,
    },
    ForIn {
        stmts: VmirStmtIdxRange<LinketImpl>,
    },
    While {
        condition: VmirCondition<LinketImpl>,
        stmts: VmirStmtIdxRange<LinketImpl>,
    },
    DoWhile {
        condition: VmirCondition<LinketImpl>,
        stmts: VmirStmtIdxRange<LinketImpl>,
    },
    IfElse {
        if_branch: VmirIfBranch<LinketImpl>,
        elif_branches: VmirElifBranchs<LinketImpl>,
        else_branch: Option<VmirElseBranch<LinketImpl>>,
    },
    Match {
        opd: VmirExprIdx<LinketImpl>,
        case_branches: VmirCaseBranches<LinketImpl>,
    },
}

pub type VmirStmtArena<LinketImpl> = Arena<VmirStmtData<LinketImpl>>;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VmirStmtIdx<LinketImpl: IsLinketImpl>(ArenaIdx<VmirStmtData<LinketImpl>>);

impl<LinketImpl: IsLinketImpl> std::ops::Deref for VmirStmtIdx<LinketImpl> {
    type Target = ArenaIdx<VmirStmtData<LinketImpl>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VmirStmtIdxRange<LinketImpl: IsLinketImpl>(ArenaIdxRange<VmirStmtData<LinketImpl>>);

impl<LinketImpl: IsLinketImpl> IntoIterator for VmirStmtIdxRange<LinketImpl> {
    type Item = VmirStmtIdx<LinketImpl>;

    type IntoIter = impl Iterator<Item = VmirStmtIdx<LinketImpl>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(VmirStmtIdx)
    }
}

impl<LinketImpl: IsLinketImpl> VmirStmtIdxRange<LinketImpl> {
    fn split_last(self) -> (Self, VmirStmtIdx<LinketImpl>) {
        let (non_lasts, last) = self.0.split_last();
        (Self(non_lasts), VmirStmtIdx(last))
    }
}

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for HirEagerStmtIdxRange {
    type Output = VmirStmtIdxRange<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut crate::builder::VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinketImpl = LinketImpl>,
    {
        let stmts = self
            .into_iter()
            .map(|stmt| match builder.hir_eager_stmt_arena()[stmt] {
                HirEagerStmtData::Let {
                    pattern,
                    contract,
                    initial_value,
                    coercion,
                } => VmirStmtData::Let {
                    pattern: pattern.pattern_idx().to_vmir(builder),
                    initial_value: initial_value.to_vmir(builder),
                    coercion: coercion.to_vmir(builder),
                },
                HirEagerStmtData::Return { result, coercion } => VmirStmtData::Return {
                    result: result.to_vmir(builder),
                    coercion: coercion.to_vmir(builder),
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
                    coercion,
                    discarded,
                } => VmirStmtData::Eval {
                    expr: expr.to_vmir(builder),
                    coercion: coercion.to_vmir(builder),
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
                    opd,
                    ref case_branches,
                    ..
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
pub enum VmirCondition<LinketImpl: IsLinketImpl> {
    /// `be` condition with syntactically correct pattern.
    /// This requires special handling for many cases.
    Be {
        opd: VmirExprIdx<LinketImpl>,
        pattern: VmirPattern<LinketImpl>,
    },
    /// all other conditions.
    /// for simplicity, `be` with a syntactically broken pattern is also included in there
    Other {
        opd: VmirExprIdx<LinketImpl>,
        conversion: VmirConditionConversion<LinketImpl>,
    },
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VmirConditionConversion<LinketImpl> {
    None,
    IntToBool,
    Todo(LinketImpl),
}

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for &HirEagerCondition {
    type Output = VmirCondition<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinketImpl = LinketImpl>,
    {
        match *self {
            HirEagerCondition::Be {
                opd,
                contract,
                ref pattern,
            } => VmirCondition::Be {
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

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for ConditionConversion {
    type Output = VmirConditionConversion<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinketImpl = LinketImpl>,
    {
        match self {
            ConditionConversion::None => VmirConditionConversion::None,
            ConditionConversion::IntToBool(_) => VmirConditionConversion::IntToBool,
        }
    }
}

/// # eval

impl<LinketImpl: IsLinketImpl> VmirStmtIdxRange<LinketImpl> {
    pub fn eval<'comptime>(
        self,
        ctx: &mut impl EvalVmir<'comptime, LinketImpl>,
    ) -> LinketImplVmControlFlow<LinketImpl> {
        ctx.eval_stmts(self, |ctx| self.eval_aux(ctx))
    }

    pub fn eval_aux<'comptime>(
        self,
        ctx: &mut impl EvalVmir<'comptime, LinketImpl>,
    ) -> LinketImplVmControlFlow<LinketImpl> {
        let (non_lasts, last) = self.split_last();
        last.eval(ctx)
    }
}

impl<LinketImpl: IsLinketImpl> VmirStmtIdx<LinketImpl> {
    pub fn eval<'comptime>(
        self,
        ctx: &mut impl EvalVmir<'comptime, LinketImpl>,
    ) -> LinketImplVmControlFlow<LinketImpl> {
        ctx.eval_stmt(self, |ctx| self.eval_aux(ctx))
    }

    pub fn eval_aux<'comptime>(
        self,
        ctx: &mut impl EvalVmir<'comptime, LinketImpl>,
    ) -> LinketImplVmControlFlow<LinketImpl> {
        use VmControlFlow::*;

        match *self.entry(ctx.vmir_stmt_arena()) {
            VmirStmtData::Let {
                pattern,
                initial_value,
                coercion,
            } => {
                let initial_value = initial_value.eval(coercion, ctx)?;
                pattern.take_value(initial_value, ctx);
                Continue(().into())
            }
            VmirStmtData::Return { result, coercion } => Return(result.eval(coercion, ctx)?),
            VmirStmtData::Require { condition } => match condition.eval(ctx)? {
                true => todo!(),
                false => todo!(),
            },
            VmirStmtData::Assert { condition } => match condition.eval(ctx)? {
                true => todo!(),
                false => todo!(),
            },
            VmirStmtData::Break => LoopExit(().into()),
            VmirStmtData::Eval {
                expr,
                coercion,
                discarded,
            } => {
                let result = expr.eval(coercion, ctx)?;
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

impl<LinketImpl: IsLinketImpl> VmirCondition<LinketImpl> {
    fn eval<'comptime>(
        self,
        ctx: &mut impl EvalVmir<'comptime, LinketImpl>,
    ) -> VmControlFlow<bool, LinketImpl::Value, LinketImpl::Exception> {
        match self {
            VmirCondition::Be { opd, pattern } => todo!(),
            VmirCondition::Other { opd, conversion } => opd.eval(None, ctx).map(|v| v.to_bool()),
        }
    }
}
