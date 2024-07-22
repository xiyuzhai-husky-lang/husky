mod branch_stmt;
mod loop_stmt;

pub use self::branch_stmt::*;
pub use self::loop_stmt::*;

use self::{be_variable::HirEagerBeVariablesPattern, let_variable::HirEagerLetVariablesPattern};
use crate::{coercion::HirEagerCoercion, *};
use husky_expr::stmt::ConditionConversion;
use husky_fly_term::ExpectationOutcome;
use husky_hir_ty::ritchie::HirContract;
use husky_sem_expr::{stmt::condition::SemCondition, SemStmtData, SemStmtIdx, SemStmtIdxRange};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirEagerStmtData {
    Let {
        pattern: HirEagerLetVariablesPattern,
        contract: HirContract,
        initial_value: HirEagerExprIdx,
        coercion: Option<HirEagerCoercion>,
    },
    Return {
        result: HirEagerExprIdx,
        coercion: HirEagerCoercion,
    },
    Require {
        condition: HirEagerCondition,
    },
    Assert {
        condition: HirEagerCondition,
    },
    Break,
    Eval {
        expr: HirEagerExprIdx,
        coercion: Option<HirEagerCoercion>,
        discarded: bool,
    },
    ForBetween {
        particulars: HirEagerForBetweenParticulars,
        // for_loop_varible_idx: CurrentHirEagerSymbolIdx,
        stmts: HirEagerStmtIdxRange,
    },
    Forext {
        particulars: HirEagerForExtParticulars,
        stmts: HirEagerStmtIdxRange,
    },
    ForIn {
        condition: HirEagerExprIdx,
        stmts: HirEagerStmtIdxRange,
    },
    While {
        condition: HirEagerCondition,
        stmts: HirEagerStmtIdxRange,
    },
    DoWhile {
        condition: HirEagerCondition,
        stmts: HirEagerStmtIdxRange,
    },
    IfElse {
        if_branch: HirEagerIfBranch,
        elif_branches: Vec<HirEagerElifBranch>,
        else_branch: Option<HirEagerElseBranch>,
    },
    Match {
        opd: HirEagerExprIdx,
        contract: HirContract,
        case_branches: Vec<HirEagerCaseBranch>,
    },
}

pub type HirEagerStmtArena = Arena<HirEagerStmtData>;
pub type HirEagerStmtIdx = ArenaIdx<HirEagerStmtData>;
pub type HirEagerStmtIdxRange = ArenaIdxRange<HirEagerStmtData>;
pub type HirEagerStmtMap<V> = ArenaMap<HirEagerStmtData, V>;

impl ToHirEager for SemStmtIdx {
    type Output = Option<HirEagerStmtData>;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        Some(match *self.data(builder.sem_stmt_arena_ref()) {
            SemStmtData::Let {
                ref let_pattern_sem_obelisk,
                contract,
                initial_value,
                ref coercion_outcome,
                ..
            } => HirEagerStmtData::Let {
                pattern: builder.new_let_variables_pattern(let_pattern_sem_obelisk),
                contract: HirContract::from_contract(contract),
                initial_value: initial_value.to_hir_eager(builder),
                coercion: coercion_outcome
                    .as_ref()
                    .map(|coercion_outcome| coercion_outcome.coercion().to_hir_eager(builder)),
            },
            SemStmtData::Return {
                result,
                ref coercion_outcome,
                ..
            } => HirEagerStmtData::Return {
                result: result.to_hir_eager(builder),
                coercion: coercion_outcome
                    .as_ref()
                    .unwrap()
                    .coercion()
                    .to_hir_eager(builder),
            },
            SemStmtData::Require { condition, .. } => HirEagerStmtData::Require {
                condition: condition.to_hir_eager(builder),
            },
            SemStmtData::Assert { condition, .. } => HirEagerStmtData::Assert {
                condition: condition.to_hir_eager(builder),
            },
            SemStmtData::Break { break_token: _ } => HirEagerStmtData::Break,
            SemStmtData::Eval {
                expr,
                ref outcome,
                eol_semicolon,
            } => HirEagerStmtData::Eval {
                expr: expr.to_hir_eager(builder),
                coercion: match outcome {
                    Some(ExpectationOutcome::Coercion(coercion_outcome)) => {
                        Some(coercion_outcome.coercion().to_hir_eager(builder))
                    }
                    _ => None,
                },
                discarded: eol_semicolon.is_some(),
            },
            SemStmtData::ForBetween {
                ref particulars,
                for_loop_varible_idx: _for_loop_varible_idx,
                stmts: ref block,
                ..
            } => HirEagerStmtData::ForBetween {
                particulars: particulars.to_hir_eager(builder),
                stmts: block.to_hir_eager(builder),
            },
            SemStmtData::ForIn { .. } => todo!(),
            SemStmtData::Forext {
                ref particulars,
                stmts: ref block,
                ..
            } => HirEagerStmtData::Forext {
                particulars: particulars.to_hir_eager(builder),
                stmts: block.to_hir_eager(builder),
            },
            SemStmtData::While {
                condition,
                stmts: block,
                ..
            } => HirEagerStmtData::While {
                condition: condition.to_hir_eager(builder),
                stmts: block.to_hir_eager(builder),
            },
            SemStmtData::DoWhile {
                condition,
                stmts: block,
                ..
            } => HirEagerStmtData::DoWhile {
                condition: condition.to_hir_eager(builder),
                stmts: block.to_hir_eager(builder),
            },
            SemStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => HirEagerStmtData::IfElse {
                if_branch: if_branch.to_hir_eager(builder),
                elif_branches: elif_branches.to_hir_eager(builder),
                else_branch: else_branch.to_hir_eager(builder),
            },
            SemStmtData::Match {
                opd,
                contract,
                ref case_branches,
                ..
            } => HirEagerStmtData::Match {
                opd: opd.to_hir_eager(builder),
                contract: HirContract::from_contract(contract),
                case_branches: case_branches.to_hir_eager(builder),
            },
            SemStmtData::Narrate { narrate_token } => todo!(),
        })
    }
}

impl ToHirEager for SemStmtIdxRange {
    type Output = HirEagerStmtIdxRange;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        let mut sem_stmt_indices: Vec<SemStmtIdx> = vec![];
        let mut hir_eager_stmts: Vec<HirEagerStmtData> = vec![];
        for sem_stmt_idx in self {
            match sem_stmt_idx.to_hir_eager(builder) {
                Some(hir_eager_stmt) => {
                    sem_stmt_indices.push(sem_stmt_idx);
                    hir_eager_stmts.push(hir_eager_stmt)
                }
                None => todo!(),
            }
        }
        builder.alloc_stmts(sem_stmt_indices, hir_eager_stmts)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirEagerCondition {
    Be {
        opd: HirEagerExprIdx,
        contract: HirContract,
        pattern: HirEagerBeVariablesPattern,
    },
    Other {
        opd: HirEagerExprIdx,
        conversion: ConditionConversion,
    },
}

impl ToHirEager for SemCondition {
    // ad hoc
    type Output = HirEagerCondition;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        match *self {
            SemCondition::Be {
                src,
                contract,
                be_regional_token_idx: _,
                target,
            } => HirEagerCondition::Be {
                opd: src.to_hir_eager(builder),
                contract: HirContract::from_contract(contract),
                pattern: target.to_hir_eager(builder),
            },
            SemCondition::Other {
                sem_expr_idx,
                conversion,
            } => HirEagerCondition::Other {
                opd: sem_expr_idx.to_hir_eager(builder),
                conversion,
            },
        }
    }
}
