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

    fn elaborate_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        region_data: VdMirExprRegionDataRef,
    ) -> Self::ElaborationTracker;

    fn extract_elaboration_tracker(
        &self,
        elaboration: &Self::ElaborationTracker,
        region_data: VdMirExprRegionDataRef,
    ) -> VdMirTracker;
}

impl IsVdMirSequentialElaboratorInner for () {
    type ElaborationTracker = ();

    fn elaborate_stmt(&mut self, stmt: VdMirStmtIdx, region_data: VdMirExprRegionDataRef) -> () {}

    fn extract_elaboration_tracker(
        &self,
        elaboration: &Self::ElaborationTracker,
        region_data: VdMirExprRegionDataRef,
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
    fn elaborate_stmts(&mut self, stmts: VdMirStmtIdxRange, region_data: VdMirExprRegionDataRef) {
        for stmt in stmts {
            self.elaborate_stmt(stmt, region_data);
        }
    }

    fn extract(&self, mut region_data: VdMirExprRegionDataMut) {
        for (stmt, elaboration_tracker) in self.stmt_elaboration_trackers.iter() {
            let elaboration_tracker = self
                .inner
                .extract_elaboration_tracker(elaboration_tracker, region_data.as_region_data_ref());
            region_data.set_elaboration_tracker(stmt, elaboration_tracker);
        }
    }
}

impl<Inner> VdMirSequentialElaborator<Inner>
where
    Inner: IsVdMirSequentialElaboratorInner,
{
    fn elaborate_stmt(&mut self, stmt: VdMirStmtIdx, region_data: VdMirExprRegionDataRef) {
        match *region_data.stmt_arena[stmt].data() {
            VdMirStmtData::Block { stmts, .. } => self.elaborate_stmts(stmts, region_data),
            VdMirStmtData::LetPlaceholder { .. }
            | VdMirStmtData::LetAssigned { .. }
            | VdMirStmtData::Goal { .. }
            | VdMirStmtData::Have { .. }
            | VdMirStmtData::Show { .. }
            | VdMirStmtData::Qed => (),
        }
        let elaboration = self.inner.elaborate_stmt(stmt, region_data);
        self.stmt_elaboration_trackers.insert_new(stmt, elaboration);
    }
}
