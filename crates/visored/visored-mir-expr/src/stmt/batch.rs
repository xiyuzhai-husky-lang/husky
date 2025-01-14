use super::*;

pub(crate) struct VdMirStmtBatch {
    stmts: Vec<VdMirStmtEntry>,
    sources: Vec<VdMirStmtSource>,
    goal_place: OncePlace<VdMirExprIdx>,
}

impl VdMirStmtBatch {
    pub(super) fn new() -> Self {
        Self {
            stmts: Vec::new(),
            sources: Vec::new(),
            goal_place: OncePlace::default(),
        }
    }

    pub(super) fn push(&mut self, stmt: VdMirStmtEntry, source: VdMirStmtSource) {
        self.stmts.push(stmt);
        self.sources.push(source);
    }

    pub(super) fn goal_place_mut(&mut self) -> &mut OncePlace<VdMirExprIdx> {
        &mut self.goal_place
    }

    pub(crate) fn finish(
        self,
    ) -> (
        Vec<VdMirStmtEntry>,
        Vec<VdMirStmtSource>,
        Option<VdMirExprIdx>,
    ) {
        (self.stmts, self.sources, self.goal_place.finish())
    }
}
