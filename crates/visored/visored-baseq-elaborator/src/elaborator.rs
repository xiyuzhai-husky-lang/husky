use std::marker::PhantomData;
use visored_mir_expr::{
    elaboration::VdMirTracker,
    elaborator::linear::{IsVdMirSequentialElaboratorInner, VdMirSequentialElaborator},
    region::VdMirExprRegionDataRef,
    stmt::VdMirStmtIdx,
};

#[derive(Debug, Default)]
pub struct VdBaseqElaboratorInner<'sess> {
    pub(crate) phantom: PhantomData<&'sess ()>,
}

pub type VdBaseqElaborator<'sess> = VdMirSequentialElaborator<VdBaseqElaboratorInner<'sess>>;

impl<'sess> IsVdMirSequentialElaboratorInner for VdBaseqElaboratorInner<'sess> {
    type ElaborationTracker = ();

    fn elaborate_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        region_data: VdMirExprRegionDataRef,
    ) -> Self::ElaborationTracker {
        todo!()
    }

    fn extract_elaboration_tracker(
        &self,
        elaboration: &Self::ElaborationTracker,
        region_data: VdMirExprRegionDataRef,
    ) -> VdMirTracker {
        todo!()
        // VdMirTracker::new_trivial()
    }
}
