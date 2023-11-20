mod branch_stmt;
mod loop_stmt;

pub use self::branch_stmt::*;
pub use self::loop_stmt::*;

use crate::*;
use husky_sema_expr::{SemaStmtData, SemaStmtIdx, SemaStmtIdxRange};

use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirEagerStmtData {
    Let {
        pattern: HirEagerLetVariablesPattern,
        initial_value: HirEagerExprIdx,
    },
    Return {
        result: HirEagerExprIdx,
    },
    Require {
        condition: HirEagerCondition,
    },
    Assert {
        condition: HirEagerCondition,
    },
    Break,
    Eval {
        expr_idx: HirEagerExprIdx,
        discarded: bool,
    },
    ForBetween {
        particulars: HirEagerForBetweenParticulars,
        // frame_var_symbol_idx: CurrentHirEagerSymbolIdx,
        block: HirEagerStmtIdxRange,
    },
    Forext {
        particulars: HirEagerForExtParticulars,
        block: HirEagerStmtIdxRange,
    },
    ForIn {
        condition: HirEagerExprIdx,
        block: HirEagerStmtIdxRange,
    },
    While {
        condition: HirEagerCondition,
        stmts: HirEagerStmtIdxRange,
    },
    DoWhile {
        condition: HirEagerCondition,
        block: HirEagerStmtIdxRange,
    },
    IfElse {
        if_branch: HirEagerIfBranch,
        elif_branches: Vec<HirEagerElifBranch>,
        else_branch: Option<HirEagerElseBranch>,
    },
    Match {
        case_branches: Vec<HirEagerCaseBranch>,
        match_target: ArenaIdx<HirEagerExprData>,
    },
}

pub type HirEagerStmtArena = Arena<HirEagerStmtData>;
pub type HirEagerStmtIdx = ArenaIdx<HirEagerStmtData>;
pub type HirEagerStmtIdxRange = ArenaIdxRange<HirEagerStmtData>;
pub type HirEagerStmtMap<V> = ArenaMap<HirEagerStmtData, V>;

impl ToHirEager for SemaStmtIdx {
    type Output = Option<HirEagerStmtData>;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        Some(match self.data(builder.sema_stmt_arena_ref()) {
            SemaStmtData::Let {
                let_token: _,
                ref let_pattern_sema_obelisk,
                initial_value_sema_expr_idx: initial_value,
                ..
            } => HirEagerStmtData::Let {
                pattern: builder.new_let_variables_pattern(let_pattern_sema_obelisk),
                initial_value: initial_value.to_hir_eager(builder),
            },
            SemaStmtData::Return {
                return_token: _,
                result,
            } => HirEagerStmtData::Return {
                result: result.to_hir_eager(builder),
            },
            SemaStmtData::Require {
                require_token: _,
                condition,
            } => HirEagerStmtData::Require {
                condition: HirEagerCondition(condition.to_hir_eager(builder)),
            },
            SemaStmtData::Assert {
                assert_token: _,
                condition,
            } => HirEagerStmtData::Assert {
                condition: HirEagerCondition(condition.to_hir_eager(builder)),
            },
            SemaStmtData::Break { break_token: _ } => HirEagerStmtData::Break,
            SemaStmtData::Eval {
                sema_expr_idx: expr_idx,
                eol_semicolon,
            } => HirEagerStmtData::Eval {
                expr_idx: expr_idx.to_hir_eager(builder),
                discarded: eol_semicolon.as_ref().expect("no error").is_some(),
            },
            SemaStmtData::ForBetween {
                for_token: _,
                ref particulars,
                for_loop_var_symbol_idx: _frame_var_symbol_idx,
                eol_colon: _,
                ref block,
            } => HirEagerStmtData::ForBetween {
                particulars: particulars.to_hir_eager(builder),
                block: block.to_hir_eager(builder),
            },
            SemaStmtData::ForIn {
                for_token: _,
                condition: _,
                eol_colon: _,
                block: _,
            } => todo!(),
            SemaStmtData::Forext {
                forext_token: _,
                ref particulars,
                eol_colon: _,
                ref block,
            } => HirEagerStmtData::Forext {
                particulars: particulars.to_hir_eager(builder),
                block: block.to_hir_eager(builder),
            },
            SemaStmtData::While {
                condition, block, ..
            } => HirEagerStmtData::While {
                condition: HirEagerCondition(condition.to_hir_eager(builder)),
                stmts: block.to_hir_eager(builder),
            },
            SemaStmtData::DoWhile {
                condition, block, ..
            } => HirEagerStmtData::DoWhile {
                condition: HirEagerCondition(condition.to_hir_eager(builder)),
                block: block.to_hir_eager(builder),
            },
            SemaStmtData::IfElse {
                sema_if_branch: ref if_branch,
                sema_elif_branches: ref elif_branches,
                sema_else_branch: ref else_branch,
            } => HirEagerStmtData::IfElse {
                if_branch: if_branch.to_hir_eager(builder),
                elif_branches: elif_branches.to_hir_eager(builder),
                else_branch: else_branch.to_hir_eager(builder),
            },
            SemaStmtData::Match {
                match_target,
                case_branches,
                ..
            } => HirEagerStmtData::Match {
                match_target: match_target.to_hir_eager(builder),
                case_branches: case_branches.to_hir_eager(builder),
            },
        })
    }
}

impl ToHirEager for SemaStmtIdxRange {
    type Output = HirEagerStmtIdxRange;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        let mut sema_stmt_indices: Vec<SemaStmtIdx> = vec![];
        let mut hir_eager_stmts: Vec<HirEagerStmtData> = vec![];
        for sema_stmt_idx in self {
            match sema_stmt_idx.to_hir_eager(builder) {
                Some(hir_eager_stmt) => {
                    sema_stmt_indices.push(sema_stmt_idx);
                    hir_eager_stmts.push(hir_eager_stmt)
                }
                None => todo!(),
            }
        }
        builder.alloc_stmts(sema_stmt_indices, hir_eager_stmts)
    }
}

// todo: add field for coversion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirEagerCondition(HirEagerExprIdx);

impl HirEagerCondition {
    pub fn hir_eager_expr_idx(self) -> HirEagerExprIdx {
        self.0
    }
}
