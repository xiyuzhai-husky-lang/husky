use hint::VdMirHintIdx;

use super::*;
use crate::{
    elaboration::VdMirTracker,
    stmt::{VdMirStmtData, VdMirStmtMap},
};

#[derive(Debug)]
pub struct VdMirSequentialElaborator<Inner>
where
    Inner: IsVdMirSequentialElaboratorInner,
{
    stmt_elaboration_trackers: VdMirStmtMap<Inner::ElaborationTracker>,
    inner: Inner,
}

pub trait IsVdMirSequentialElaboratorInner: std::fmt::Debug {
    type ElaborationTracker: std::fmt::Debug + Eq;

    fn elaborate_have_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        prop: VdMirExprIdx,
        hint: Option<VdMirHintIdx>,
        region_data: VdMirExprRegionDataRef,
    ) -> Self::ElaborationTracker;

    fn extract_elaborations(
        &self,
        elaboration: &Self::ElaborationTracker,
        hypothesis_constructor: VdMirHypothesisConstructor,
    ) -> VdMirTracker;
}

impl IsVdMirSequentialElaboratorInner for () {
    type ElaborationTracker = ();

    fn elaborate_have_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        prop: VdMirExprIdx,
        hint: Option<VdMirHintIdx>,
        region_data: VdMirExprRegionDataRef,
    ) -> () {
    }

    fn extract_elaborations(
        &self,
        elaboration: &Self::ElaborationTracker,
        hypothesis_constructor: VdMirHypothesisConstructor,
    ) -> VdMirTracker {
        VdMirTracker::new_trivial()
    }
}

impl<Inner> VdMirSequentialElaborator<Inner>
where
    Inner: IsVdMirSequentialElaboratorInner,
{
    pub fn new(inner: Inner, region_data: VdMirExprRegionDataRef) -> Self {
        Self {
            stmt_elaboration_trackers: VdMirStmtMap::new2(region_data.stmt_arena),
            inner,
        }
    }

    pub fn new_default(region_data: VdMirExprRegionDataRef) -> Self
    where
        Inner: Default,
    {
        Self::new(Default::default(), region_data)
    }
}

impl<Inner> IsVdMirTacticElaborator for VdMirSequentialElaborator<Inner>
where
    Inner: IsVdMirSequentialElaboratorInner,
{
    // # elaborate
    fn elaborate_stmts(&mut self, stmts: VdMirStmtIdxRange, region_data: VdMirExprRegionDataRef) {
        for stmt in stmts {
            self.elaborate_stmt(stmt, region_data);
        }
    }

    fn elaborate_expr(&mut self, expr: VdMirExprIdx, region_data: VdMirExprRegionDataRef) {
        todo!()
    }

    // # extract
    fn extract_stmts(
        &self,
        stmts: VdMirStmtIdxRange,
        mut hypothesis_constructor: VdMirHypothesisConstructor,
    ) {
        for (stmt, elaboration_tracker) in self.stmt_elaboration_trackers.iter() {
            todo!()
            // let elaboration_tracker = self
            //     .inner
            //     .extract_elaborations(elaboration_tracker, hypothesis_constructor);
            // hypothesis_constructor.add_hypothesis(stmt, elaboration);
        }
    }

    fn extract_expr(&self, expr: VdMirExprIdx, hypothesis_constructor: VdMirHypothesisConstructor) {
        todo!()
    }
}

impl<Inner> VdMirSequentialElaborator<Inner>
where
    Inner: IsVdMirSequentialElaboratorInner,
{
    fn elaborate_stmt(&mut self, stmt: VdMirStmtIdx, region_data: VdMirExprRegionDataRef) {
        match *region_data.stmt_arena[stmt].data() {
            VdMirStmtData::Block { stmts, .. } => self.elaborate_stmts(stmts, region_data),
            VdMirStmtData::LetPlaceholder { .. } => todo!(),
            VdMirStmtData::LetAssigned { .. } => todo!(),
            VdMirStmtData::Goal { .. } => (),
            VdMirStmtData::Have { prop, hint } => {
                let elaboration = self
                    .inner
                    .elaborate_have_stmt(stmt, prop, hint, region_data);
                self.stmt_elaboration_trackers.insert_new(stmt, elaboration);
            }
            VdMirStmtData::Show { .. } | VdMirStmtData::Qed { .. } => todo!(),
        }
    }
}
