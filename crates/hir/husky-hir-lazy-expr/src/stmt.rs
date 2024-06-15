mod branch_stmt;

pub use self::branch_stmt::*;

use crate::*;
use husky_expr::stmt::ConditionConversion;
use husky_hir_ty::HirType;
use husky_sem_expr::{stmt::condition::SemCondition, SemStmtData, SemStmtIdx, SemStmtIdxRange};
use idx_arena::ArenaRef;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db]
pub enum HirLazyStmtData {
    Let {
        pattern: HirLazyLetVariablesPattern,
        initial_value: HirLazyExprIdx,
    },
    Return {
        result: HirLazyExprIdx,
    },
    Require {
        condition: HirLazyCondition,
        return_ty: HirType,
    },
    Assert {
        condition: HirLazyCondition,
    },
    Eval {
        expr_idx: HirLazyExprIdx,
        discarded: bool,
    },
    IfElse {
        if_branch: HirLazyIfBranch,
        elif_branches: Vec<HirLazyElifBranch>,
        else_branch: Option<HirLazyElseBranch>,
    },
    Match {},
    Narrate {},
}

pub type HirLazyStmtArena = Arena<HirLazyStmtData>;
pub type HirLazyStmtArenaRef<'a> = ArenaRef<'a, HirLazyStmtData>;
pub type HirLazyStmtIdx = ArenaIdx<HirLazyStmtData>;
pub type HirLazyStmtIdxRange = ArenaIdxRange<HirLazyStmtData>;
pub type HirLazyStmtMap<V> = ArenaMap<HirLazyStmtData, V>;

impl ToHirLazy for SemStmtIdx {
    type Output = Option<HirLazyStmtData>;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        Some(match self.data(builder.sem_stmt_arena_ref()) {
            SemStmtData::Let {
                let_token: _,
                let_pattern_sem_obelisk: let_variables_pattern,
                initial_value_sem_expr_idx: initial_value,
                ..
            } => HirLazyStmtData::Let {
                pattern: builder.new_let_variables_pattern(let_variables_pattern),
                initial_value: initial_value.to_hir_lazy(builder),
            },
            SemStmtData::Return { result, .. } => HirLazyStmtData::Return {
                result: result.to_hir_lazy(builder),
            },
            SemStmtData::Require {
                require_token: _,
                condition,
            } => HirLazyStmtData::Require {
                condition: condition.to_hir_lazy(builder),
                return_ty: HirType::from_eth(
                    builder.sem_expr_region_data().return_ty().unwrap(),
                    builder.db(),
                )
                .unwrap(),
            },
            SemStmtData::Assert {
                assert_token: _,
                condition,
            } => HirLazyStmtData::Assert {
                condition: condition.to_hir_lazy(builder),
            },
            SemStmtData::Eval {
                sem_expr_idx: expr_idx,
                outcome: _,
                eol_semicolon,
            } => HirLazyStmtData::Eval {
                expr_idx: expr_idx.to_hir_lazy(builder),
                discarded: eol_semicolon.is_some(),
            },
            SemStmtData::Break { .. } => unreachable!(),
            SemStmtData::ForBetween { .. } => unreachable!(),
            SemStmtData::ForIn { .. } => unreachable!(),
            SemStmtData::Forext { .. } => unreachable!(),
            SemStmtData::While { .. } => unreachable!(),
            SemStmtData::DoWhile { .. } => unreachable!(),
            SemStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => HirLazyStmtData::IfElse {
                if_branch: if_branch.to_hir_lazy(builder),
                elif_branches: elif_branches
                    .iter()
                    .map(|elif_branch| elif_branch.to_hir_lazy(builder))
                    .collect(),
                else_branch: else_branch
                    .as_ref()
                    .map(|else_branch| else_branch.to_hir_lazy(builder)),
            },
            SemStmtData::Match { .. } => todo!(),
            SemStmtData::Narrate { narrate_token } => HirLazyStmtData::Narrate {},
        })
    }
}

impl ToHirLazy for SemStmtIdxRange {
    type Output = HirLazyStmtIdxRange;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        let mut sem_stmt_indices: Vec<SemStmtIdx> = vec![];
        let mut hir_lazy_stmts: Vec<HirLazyStmtData> = vec![];
        for sem_stmt_idx in self {
            match sem_stmt_idx.to_hir_lazy(builder) {
                Some(hir_lazy_stmt) => {
                    sem_stmt_indices.push(sem_stmt_idx);
                    hir_lazy_stmts.push(hir_lazy_stmt)
                }
                None => todo!(),
            }
        }
        builder.alloc_stmts(sem_stmt_indices, hir_lazy_stmts)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirLazyCondition {
    Be {
        src: HirLazyExprIdx,
        pattern: HirLazyBeVariablesPattern,
    },
    Other {
        hir_lazy_expr_idx: HirLazyExprIdx,
        conversion: ConditionConversion,
    },
}

impl ToHirLazy for SemCondition {
    // ad hoc
    type Output = HirLazyCondition;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        match *self {
            SemCondition::Be {
                src,
                be_regional_token_idx: _,
                target,
            } => HirLazyCondition::Be {
                src: src.to_hir_lazy(builder),
                pattern: target.to_hir_lazy(builder),
            },
            SemCondition::Other {
                sem_expr_idx,
                conversion,
            } => HirLazyCondition::Other {
                hir_lazy_expr_idx: sem_expr_idx.to_hir_lazy(builder),
                conversion,
            },
        }
    }
}
