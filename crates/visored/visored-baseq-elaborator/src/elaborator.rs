use std::marker::PhantomData;
use visored_mir_expr::{
    elaboration::VdMirTracker,
    elaborator::linear::{IsVdMirSequentialElaboratorInner, VdMirSequentialElaborator},
    expr::VdMirExprIdx,
    hint::VdMirHintIdx,
    hypothesis::constructor::VdMirHypothesisConstructor,
    region::VdMirExprRegionDataRef,
    stmt::{VdMirStmtData, VdMirStmtIdx},
};

#[derive(Debug, Default)]
pub struct VdBaseqElaboratorInner<'sess> {
    pub(crate) phantom: PhantomData<&'sess ()>,
}

pub type VdBaseqElaborator<'sess> = VdMirSequentialElaborator<VdBaseqElaboratorInner<'sess>>;

impl<'sess> IsVdMirSequentialElaboratorInner for VdBaseqElaboratorInner<'sess> {
    type ElaborationTracker = ();

    fn elaborate_have_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        prop: VdMirExprIdx,
        hint: Option<VdMirHintIdx>,
        region_data: VdMirExprRegionDataRef,
    ) -> Self::ElaborationTracker {
        todo!()
    }

    fn extract_elaborations(
        &self,
        elaboration: &Self::ElaborationTracker,
        hypothesis_constructor: VdMirHypothesisConstructor,
    ) -> VdMirTracker {
        todo!()
        // VdMirTracker::new_trivial()
    }
}
