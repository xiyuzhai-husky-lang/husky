pub mod linear;

use super::*;
pub trait IsVdMirTacticElaborator {
    fn eval_all_tactics_within_stmts(
        &mut self,
        stmts: VdMirStmtIdxRange,
        region_data: VdMirExprRegionDataRef,
    );
    fn eval_all_tactics_within_expr(
        &mut self,
        expr: VdMirExprIdx,
        region_data: VdMirExprRegionDataRef,
    );

    fn extract(&self, region_data: VdMirExprRegionDataMut);
}

pub type VdMirTacticTrivialElaborator = self::linear::VdMirTacticLinearElaborator<()>;
