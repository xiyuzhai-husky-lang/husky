use std::marker::PhantomData;
use visored_mir_expr::{
    elaboration::VdMirTracker,
    elaborator::linear::{IsVdMirSequentialElaboratorInner, VdMirSequentialElaborator},
};

#[derive(Debug, Default)]
pub struct VdMirStandardSequentialElaboratorInner<'sess> {
    pub(crate) phantom: PhantomData<&'sess ()>,
}

pub type VdMirStandardSequentialElaborator<'sess> =
    VdMirSequentialElaborator<VdMirStandardSequentialElaboratorInner<'sess>>;

impl<'sess> IsVdMirSequentialElaboratorInner for VdMirStandardSequentialElaboratorInner<'sess> {
    type ElaborationTracker = ();

    fn elaborate_stmt(
        &mut self,
        stmt: visored_mir_expr::stmt::VdMirStmtIdx,
        region_data: visored_mir_expr::region::VdMirExprRegionDataRef,
    ) -> Self::ElaborationTracker {
        ()
    }

    fn extract_elaboration_tracker(
        &self,
        elaboration: &Self::ElaborationTracker,
        region_data: visored_mir_expr::region::VdMirExprRegionDataRef,
    ) -> VdMirTracker {
        VdMirTracker::new_trivial()
    }
}
