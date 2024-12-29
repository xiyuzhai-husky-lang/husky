use crate::region::{VdMirExprRegionData, VdMirExprRegionDataRef};

pub trait IsVdMirTacticElaborator {
    fn eval(&mut self, region_data_ref: VdMirExprRegionDataRef);

    fn extract(&self, region_data: &mut VdMirExprRegionData);
}

#[derive(Default)]
pub struct VdMirTacticTrivialElaborator;

impl IsVdMirTacticElaborator for VdMirTacticTrivialElaborator {
    fn eval(&mut self, region_data_ref: VdMirExprRegionDataRef) {}

    fn extract(&self, region_data: &mut VdMirExprRegionData) {}
}
