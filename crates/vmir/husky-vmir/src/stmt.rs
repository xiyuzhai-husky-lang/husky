mod ifelse;
mod r#loop;
mod r#match;

use crate::{
    coersion::VmirCoersion,
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
pub type VmirStmtIdx<LinkageImpl> = ArenaIdx<VmirStmtData<LinkageImpl>>;
pub type VmirStmtIdxRange<LinkageImpl> = ArenaIdxRange<VmirStmtData<LinkageImpl>>;

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
        builder.alloc_stmts(stmts)
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
