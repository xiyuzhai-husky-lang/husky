mod elaborator;
pub mod error;

use super::*;
use crate::{
    expr::VdMirExprIdx,
    region::{VdMirExprRegionData, VdMirExprRegionDataMut, VdMirExprRegionDataRef},
    stmt::VdMirStmtIdxRange,
};

pub enum VdMirTacticElaboration {
    Explicit(VdMirTacticIdxRange),
    Implicit,
    Illicit,
}

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

#[derive(Default)]
pub struct VdMirTacticLinearElaborator<Core> {
    top_tactics: Vec<VdMirTacticIdxRange>,
    core: Core,
}

pub type VdMirTacticTrivialElaborator = VdMirTacticLinearElaborator<()>;

impl<Core> VdMirTacticLinearElaborator<Core> {
    fn new(core: Core) -> Self {
        Self {
            top_tactics: vec![],
            core,
        }
    }
}

impl<Core> IsVdMirTacticElaborator for VdMirTacticLinearElaborator<Core> {
    fn eval_all_tactics_within_stmts(
        &mut self,
        stmts: VdMirStmtIdxRange,
        region_data: VdMirExprRegionDataRef,
    ) {
        todo!()
    }

    fn eval_all_tactics_within_expr(
        &mut self,
        expr: VdMirExprIdx,
        region_data: VdMirExprRegionDataRef,
    ) {
        todo!()
    }

    fn extract(&self, region_data: VdMirExprRegionDataMut) {
        todo!()
    }
}
