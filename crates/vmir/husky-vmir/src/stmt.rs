use crate::ToVmir;
use husky_hir_eager_expr::{HirEagerStmtData, HirEagerStmtIdx};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

pub enum VmirStmtData {
    Let,
    Require,
    Return,
}

pub type VmirStmtArena = Arena<VmirStmtData>;
pub type VmirStmtIdx = ArenaIdx<VmirStmtData>;
pub type VmirStmtIdxRange = ArenaIdxRange<VmirStmtData>;

impl ToVmir for HirEagerStmtIdx {
    type Output = VmirStmtIdx;

    fn to_vmir(self, builder: &mut crate::builder::VmirExprBuilder) -> Self::Output {
        match builder.hir_eager_stmt_arena()[self] {
            HirEagerStmtData::Let {
                pattern,
                contract,
                initial_value,
                coersion,
            } => todo!(),
            HirEagerStmtData::Return { result, coersion } => todo!(),
            HirEagerStmtData::Require { ref condition } => todo!(),
            HirEagerStmtData::Assert { ref condition } => todo!(),
            HirEagerStmtData::Break => todo!(),
            HirEagerStmtData::Eval {
                expr_idx,
                coersion,
                discarded,
            } => todo!(),
            HirEagerStmtData::ForBetween {
                ref particulars,
                block,
            } => todo!(),
            HirEagerStmtData::Forext {
                ref particulars,
                block,
            } => todo!(),
            HirEagerStmtData::ForIn {
                ref condition,
                block,
            } => todo!(),
            HirEagerStmtData::While {
                ref condition,
                stmts,
            } => todo!(),
            HirEagerStmtData::DoWhile {
                ref condition,
                block,
            } => todo!(),
            HirEagerStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => todo!(),
            HirEagerStmtData::Match {
                ref case_branches,
                ref match_target,
            } => todo!(),
        }
    }
}
