pub mod linear;

use crate::{
    expr::VdMirExprIdx,
    region::{VdMirExprRegionDataMut, VdMirExprRegionDataRef},
    stmt::{VdMirStmtIdx, VdMirStmtIdxRange},
    *,
};

pub trait IsVdMirTacticElaborator: std::fmt::Debug {
    fn elaborate_stmts(&mut self, stmts: VdMirStmtIdxRange, region_data: VdMirExprRegionDataRef);
    fn elaborate_stmt(&mut self, stmt: VdMirStmtIdx, region_data: VdMirExprRegionDataRef);
    fn extract(&self, region_data: VdMirExprRegionDataMut);
}

pub type VdMirTrivialElaborator = self::linear::VdMirSequentialElaborator<()>;
