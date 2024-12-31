pub mod linear;

use super::*;
use crate::stmt::VdMirStmtIdx;

pub trait IsVdMirTacticElaborator: std::fmt::Debug {
    fn eval_all_tactics_within_stmts(
        &mut self,
        stmts: VdMirStmtIdxRange,
        region_data: VdMirExprRegionDataRef,
    );
    fn eval_all_tactics_within_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        region_data: VdMirExprRegionDataRef,
    );
    /// TODO: should we even keep this? When does expr contain tactics?
    fn eval_all_tactics_within_expr(
        &mut self,
        expr: VdMirExprIdx,
        region_data: VdMirExprRegionDataRef,
    );

    fn extract(&self, region_data: VdMirExprRegionDataMut);
}

pub type VdMirTacticTrivialElaborator = self::linear::VdMirTacticSequentialElaborator<()>;
