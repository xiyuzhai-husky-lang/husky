use std::marker::PhantomData;
use visored_mir_expr::tactic::elaboration::{
    elaborator::linear::{IsVdMirTacticSequentialElaboratorInner, VdMirTacticSequentialElaborator},
    VdMirTacticElaborationTracker,
};

#[derive(Debug, Default)]
pub struct VdMirTacticStandardSequentialElaboratorInner<'sess> {
    pub(crate) phantom: PhantomData<&'sess ()>,
}

pub type VdMirTacticStandardSequentialElaborator<'sess> =
    VdMirTacticSequentialElaborator<VdMirTacticStandardSequentialElaboratorInner<'sess>>;

impl<'sess> IsVdMirTacticSequentialElaboratorInner
    for VdMirTacticStandardSequentialElaboratorInner<'sess>
{
    type ElaborationTracker = ();

    fn eval_tactic(
        &mut self,
        tactic: visored_mir_expr::tactic::VdMirTacticIdx,
        region_data: visored_mir_expr::region::VdMirExprRegionDataRef,
    ) -> Self::ElaborationTracker {
        ()
    }

    fn extract_elaboration_tracker(
        &self,
        elaboration: &Self::ElaborationTracker,
        region_data: visored_mir_expr::region::VdMirExprRegionDataRef,
    ) -> VdMirTacticElaborationTracker {
        VdMirTacticElaborationTracker::new_trivial()
    }
}
