mod branch_stmt;
mod loop_stmt;

pub use self::branch_stmt::*;
pub use self::loop_stmt::*;

use crate::{coersion::HirEagerCoersion, *};
use husky_expr::stmt::ConditionConversion;
use husky_fluffy_term::ExpectationOutcome;
use husky_hir_ty::ritchie::HirEagerContract;
use husky_sema_expr::{SemaCondition, SemaStmtData, SemaStmtIdx, SemaStmtIdxRange};

use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirEagerStmtData {
    Let {
        pattern: HirEagerLetVariablesPattern,
        contract: HirEagerContract,
        initial_value: HirEagerExprIdx,
        coersion: Option<HirEagerCoersion>,
    },
    Return {
        result: HirEagerExprIdx,
        coersion: HirEagerCoersion,
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
        coersion: Option<HirEagerCoersion>,
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
        match_target: HirEagerExprIdx,
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
            &SemaStmtData::Let {
                ref let_pattern_sema_obelisk,
                contract,
                initial_value_sema_expr_idx,
                ref coersion_outcome,
                ..
            } => HirEagerStmtData::Let {
                pattern: builder.new_let_variables_pattern(let_pattern_sema_obelisk),
                contract: HirEagerContract::from_term(contract),
                initial_value: initial_value_sema_expr_idx.to_hir_eager(builder),
                coersion: coersion_outcome
                    .as_ref()
                    .map(|coersion_outcome| coersion_outcome.coersion().to_hir_eager(builder)),
            },
            SemaStmtData::Return {
                result,
                ref coersion_outcome,
                ..
            } => HirEagerStmtData::Return {
                result: result.to_hir_eager(builder),
                coersion: coersion_outcome
                    .as_ref()
                    .unwrap()
                    .coersion()
                    .to_hir_eager(builder),
            },
            SemaStmtData::Require { condition, .. } => HirEagerStmtData::Require {
                condition: condition.to_hir_eager(builder),
            },
            SemaStmtData::Assert { condition, .. } => HirEagerStmtData::Assert {
                condition: condition.to_hir_eager(builder),
            },
            SemaStmtData::Break { break_token: _ } => HirEagerStmtData::Break,
            SemaStmtData::Eval {
                sema_expr_idx,
                outcome,
                eol_semicolon,
            } => HirEagerStmtData::Eval {
                expr_idx: sema_expr_idx.to_hir_eager(builder),
                discarded: eol_semicolon.as_ref().expect("no error").is_some(),
                coersion: match outcome {
                    Some(ExpectationOutcome::Coersion(coersion_outcome)) => {
                        Some(coersion_outcome.coersion().to_hir_eager(builder))
                    }
                    _ => None,
                },
            },
            SemaStmtData::ForBetween {
                ref particulars,
                for_loop_var_symbol_idx: _frame_var_symbol_idx,
                ref block,
                ..
            } => HirEagerStmtData::ForBetween {
                particulars: particulars.to_hir_eager(builder),
                block: block.to_hir_eager(builder),
            },
            SemaStmtData::ForIn { .. } => todo!(),
            SemaStmtData::Forext {
                ref particulars,
                ref block,
                ..
            } => HirEagerStmtData::Forext {
                particulars: particulars.to_hir_eager(builder),
                block: block.to_hir_eager(builder),
            },
            SemaStmtData::While {
                condition, block, ..
            } => HirEagerStmtData::While {
                condition: condition.to_hir_eager(builder),
                stmts: block.to_hir_eager(builder),
            },
            SemaStmtData::DoWhile {
                condition, block, ..
            } => HirEagerStmtData::DoWhile {
                condition: condition.to_hir_eager(builder),
                block: block.to_hir_eager(builder),
            },
            SemaStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirEagerCondition {
    Be {
        src: HirEagerExprIdx,
        target: HirEagerBeVariablesPattern,
    },
    Other {
        hir_eager_expr_idx: HirEagerExprIdx,
        conversion: ConditionConversion,
    },
}

impl ToHirEager for SemaCondition {
    // ad hoc
    type Output = HirEagerCondition;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        match *self {
            SemaCondition::Be {
                src,
                be_regional_token_idx,
                target,
            } => HirEagerCondition::Be {
                src: src.to_hir_eager(builder),
                target: target.to_hir_eager(builder),
            },
            SemaCondition::Other {
                sema_expr_idx,
                conversion,
            } => HirEagerCondition::Other {
                hir_eager_expr_idx: sema_expr_idx.to_hir_eager(builder),
                conversion,
            },
        }
    }
}
