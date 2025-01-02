use std::marker::PhantomData;
use visored_mir_expr::{
    elaborator::linear::{IsVdMirSequentialElaboratorInner, VdMirSequentialElaborator},
    expr::VdMirExprIdx,
    hint::VdMirHintIdx,
    hypothesis::{constructor::VdMirHypothesisConstructor, VdMirHypothesisIdx},
    region::VdMirExprRegionDataRef,
    stmt::{VdMirStmtData, VdMirStmtIdx},
};

use crate::{
    hypothesis::{
        constructor::VdBaseqHypothesisConstructor,
        contradiction::{VdBaseqHypothesisContradiction, VdBaseqHypothesisResult},
        VdBaseqHypothesisIdx,
    },
    session::VdBaseqSession,
};

pub struct VdBaseqElaboratorInner<'db, 'sess> {
    session: &'sess VdBaseqSession<'db>,
    pub(crate) hypothesis_constructor: VdBaseqHypothesisConstructor<'db, 'sess>,
}

pub type VdBaseqElaborator<'db, 'sess> =
    VdMirSequentialElaborator<VdBaseqElaboratorInner<'db, 'sess>>;

impl<'db, 'sess> VdBaseqElaboratorInner<'db, 'sess> {
    pub fn new(session: &'sess VdBaseqSession<'db>) -> Self {
        Self {
            session,
            hypothesis_constructor: VdBaseqHypothesisConstructor::new(session),
        }
    }
}

impl<'db, 'sess> IsVdMirSequentialElaboratorInner for VdBaseqElaboratorInner<'db, 'sess> {
    type HypothesisIdx = VdBaseqHypothesisIdx<'sess>;
    type Contradiction = VdBaseqHypothesisContradiction<'sess>;

    fn elaborate_let_assigned_stmt(&mut self) -> VdBaseqHypothesisResult<'sess, ()> {
        Ok(())
    }

    fn elaborate_let_placeholder_stmt(&mut self) -> VdBaseqHypothesisResult<'sess, ()> {
        Ok(())
    }

    fn elaborate_goal_stmt(&mut self) -> VdBaseqHypothesisResult<'sess, ()> {
        Ok(())
    }

    fn elaborate_have_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        prop: VdMirExprIdx,
        hint: Option<VdMirHintIdx>,
        region_data: VdMirExprRegionDataRef,
    ) -> Result<Self::HypothesisIdx, Self::Contradiction> {
        let prop = todo!();
        match hint {
            Some(hint) => todo!(),
            None => self.obvious(prop),
        }
    }

    fn elaborate_show_stmt(
        &mut self,
    ) -> VdBaseqHypothesisResult<'sess, VdBaseqHypothesisIdx<'sess>> {
        todo!()
    }

    fn elaborate_qed_stmt(
        &mut self,
    ) -> VdBaseqHypothesisResult<'sess, VdBaseqHypothesisIdx<'sess>> {
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
