use crate::stmt::VdMirStmtData;

use super::*;

#[derive(Debug)]
pub struct VdMirTacticSequentialElaborator<Inner>
where
    Inner: IsVdMirTacticSequentialElaboratorInner,
{
    tactic_elaborations: VdMirTacticMap<Inner::ElaborationTracker>,
    inner: Inner,
}

pub trait IsVdMirTacticSequentialElaboratorInner: std::fmt::Debug {
    type ElaborationTracker: std::fmt::Debug + Eq;

    fn eval_tactic(
        &mut self,
        tactic: VdMirTacticIdx,
        region_data: VdMirExprRegionDataRef,
    ) -> Self::ElaborationTracker;

    fn extract_elaboration_tracker(
        &self,
        elaboration: &Self::ElaborationTracker,
        region_data: VdMirExprRegionDataRef,
    ) -> VdMirTacticElaborationTracker;
}

impl IsVdMirTacticSequentialElaboratorInner for () {
    type ElaborationTracker = ();

    fn eval_tactic(&mut self, tactic: VdMirTacticIdx, region_data: VdMirExprRegionDataRef) -> () {}

    fn extract_elaboration_tracker(
        &self,
        elaboration: &Self::ElaborationTracker,
        region_data: VdMirExprRegionDataRef,
    ) -> VdMirTacticElaborationTracker {
        VdMirTacticElaborationTracker::new_trivial()
    }
}

impl<Inner> VdMirTacticSequentialElaborator<Inner>
where
    Inner: IsVdMirTacticSequentialElaboratorInner,
{
    pub fn new(inner: Inner, region_data: VdMirExprRegionDataRef) -> Self {
        Self {
            tactic_elaborations: VdMirTacticMap::new2(region_data.tactic_arena),
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

impl<Inner> IsVdMirTacticElaborator for VdMirTacticSequentialElaborator<Inner>
where
    Inner: IsVdMirTacticSequentialElaboratorInner,
{
    fn eval_all_tactics_within_stmts(
        &mut self,
        stmts: VdMirStmtIdxRange,
        region_data: VdMirExprRegionDataRef,
    ) {
        for stmt in stmts {
            self.eval_all_tactics_within_stmt(stmt, region_data);
        }
    }

    fn eval_all_tactics_within_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        region_data: VdMirExprRegionDataRef,
    ) {
        match region_data.stmt_arena[stmt] {
            VdMirStmtData::Block { stmts, ref meta } => {
                self.eval_all_tactics_within_stmts(stmts, region_data)
            }
            VdMirStmtData::LetPlaceholder {
                ref pattern,
                ref ty,
            } => (),
            VdMirStmtData::LetAssigned {
                ref pattern,
                assignment,
            } => (),
            VdMirStmtData::Goal { ref prop } => (),
            VdMirStmtData::Have { ref prop, tactics } => self.eval_tactics(tactics, region_data),
            VdMirStmtData::Show { ref prop, tactics } => todo!(),
        }
    }

    fn eval_all_tactics_within_expr(
        &mut self,
        expr: VdMirExprIdx,
        region_data: VdMirExprRegionDataRef,
    ) {
    }

    fn extract(&self, mut region_data: VdMirExprRegionDataMut) {
        for (tactic, elaboration) in self.tactic_elaborations.iter() {
            let elaboration_tracker = self
                .inner
                .extract_elaboration_tracker(elaboration, region_data.as_region_data_ref());
            region_data.set_elaboration_tracker(tactic, elaboration_tracker);
        }
    }
}

impl<Inner> VdMirTacticSequentialElaborator<Inner>
where
    Inner: IsVdMirTacticSequentialElaboratorInner,
{
    fn eval_tactics(&mut self, tactics: VdMirTacticIdxRange, region_data: VdMirExprRegionDataRef) {
        for tactic in tactics {
            let elaboration = self.inner.eval_tactic(tactic, region_data);
            self.tactic_elaborations.insert_new(tactic, elaboration);
        }
    }
}
