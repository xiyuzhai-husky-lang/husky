pub mod linear;

use crate::{
    expr::VdMirExprIdx,
    hypothesis::constructor::VdMirHypothesisConstructor,
    region::{VdMirExprRegionDataMut, VdMirExprRegionDataRef},
    stmt::{VdMirStmtIdx, VdMirStmtIdxRange},
    *,
};

pub trait IsVdMirTacticElaborator: std::fmt::Debug {
    // # elaborate
    fn elaborate_stmts(&mut self, stmts: VdMirStmtIdxRange, region_data: VdMirExprRegionDataRef);
    fn elaborate_expr(&mut self, expr: VdMirExprIdx, region_data: VdMirExprRegionDataRef);
    // # extract
    fn extract_stmts(
        &self,
        stmts: VdMirStmtIdxRange,
        hypothesis_constructor: VdMirHypothesisConstructor,
    );
    fn extract_expr(&self, expr: VdMirExprIdx, hypothesis_constructor: VdMirHypothesisConstructor);
}

pub type VdMirTrivialElaborator = self::linear::VdMirSequentialElaborator<()>;
