pub mod error;

use super::*;
use crate::region::{VdMirExprRegionData, VdMirExprRegionDataMut, VdMirExprRegionDataRef};

pub enum VdMirTacticElaboration {
    Explicit(VdMirTacticIdxRange),
    Implicit,
    Illicit,
}

pub trait IsVdMirTacticElaborator {
    fn eval(&mut self, region_data: VdMirExprRegionDataRef);

    fn extract(&self, region_data: VdMirExprRegionDataMut);
}

#[derive(Default)]
pub struct VdMirTacticTrivialElaborator;

impl IsVdMirTacticElaborator for VdMirTacticTrivialElaborator {
    fn eval(&mut self, region_data: VdMirExprRegionDataRef) {}

    fn extract(&self, region_data: VdMirExprRegionDataMut) {}
}
