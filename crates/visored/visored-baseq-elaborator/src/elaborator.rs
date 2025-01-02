use std::marker::PhantomData;
use visored_mir_expr::{
    elaborator::linear::{IsVdMirSequentialElaboratorInner, VdMirSequentialElaborator},
    expr::VdMirExprIdx,
    hint::VdMirHintIdx,
    hypothesis::{constructor::VdMirHypothesisConstructor, VdMirHypothesisIdx},
    region::VdMirExprRegionDataRef,
    stmt::{VdMirStmtData, VdMirStmtIdx},
};

#[derive(Debug, Default)]
pub struct VdBaseqElaboratorInner<'sess> {
    pub(crate) phantom: PhantomData<&'sess ()>,
}

pub type VdBaseqElaborator<'sess> = VdMirSequentialElaborator<VdBaseqElaboratorInner<'sess>>;

impl<'sess> IsVdMirSequentialElaboratorInner for VdBaseqElaboratorInner<'sess> {
    type HypothesisIdx = ();
    type Contradiction = ();

    fn elaborate_let_assigned_stmt(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn elaborate_let_placeholder_stmt(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn elaborate_goal_stmt(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn elaborate_have_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        prop: VdMirExprIdx,
        hint: Option<VdMirHintIdx>,
        region_data: VdMirExprRegionDataRef,
    ) -> Result<Self::HypothesisIdx, Self::Contradiction> {
        todo!()
    }

    fn elaborate_show_stmt(&mut self) -> Result<(), ()> {
        todo!()
    }

    fn elaborate_qed_stmt(&mut self) -> Result<(), ()> {
        todo!()
    }

    fn elaborate_expr(
        &mut self,
        expr: VdMirExprIdx,
        region_data: VdMirExprRegionDataRef,
    ) -> Result<Self::HypothesisIdx, Self::Contradiction> {
        todo!()
    }

    fn transcribe_hypothesis(
        &mut self,
        hypothesis: Self::HypothesisIdx,
        goal: VdMirExprIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> VdMirHypothesisIdx {
        todo!()
    }
}
