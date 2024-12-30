use visored_mir_expr::tactic::elaboration::elaborator::linear::{
    IsVdMirTacticLinearElaboratorInner, VdMirTacticLinearElaborator,
};

#[derive(Debug, Default)]
pub struct VdMirTacticStandardLinearElaboratorInner;

pub type VdMirTacticStandardLinearElaborator =
    VdMirTacticLinearElaborator<VdMirTacticStandardLinearElaboratorInner>;

impl IsVdMirTacticLinearElaboratorInner for VdMirTacticStandardLinearElaboratorInner {
    type ElaborationTracker = ();

    fn eval_tactic(
        &mut self,
        tactic: visored_mir_expr::tactic::VdMirTacticIdx,
        region_data: visored_mir_expr::region::VdMirExprRegionDataRef,
    ) -> Self::ElaborationTracker {
        todo!()
    }

    fn extract_elaboration_tracker(
        &self,
        elaboration: &Self::ElaborationTracker,
        region_data: visored_mir_expr::region::VdMirExprRegionDataRef,
    ) -> visored_mir_expr::tactic::elaboration::VdMirTacticElaborationTracker {
        todo!()
    }
}
