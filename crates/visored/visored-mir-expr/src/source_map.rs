use crate::{
    expr::VdMirExprOrderedMap,
    stmt::{VdMirStmtIdx, VdMirStmtIdxRange, VdMirStmtOrderedMap, VdMirStmtSource},
    tactic::{VdMirTacticIdx, VdMirTacticIdxRange, VdMirTacticOrderedMap, VdMirTacticSource},
};
use visored_sem_expr::{
    block::VdSemBlockIdx, clause::VdSemClauseIdx, division::VdSemDivisionIdx, expr::VdSemExprIdx,
    sentence::VdSemSentenceIdx,
};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct VdMirSourceMap {
    stmt_map: VdMirStmtOrderedMap<VdMirStmtSource>,
    tactic_map: VdMirTacticOrderedMap<VdMirTacticSource>,
}

impl VdMirSourceMap {
    pub(crate) fn set_stmts(
        &mut self,
        stmts: VdMirStmtIdxRange,
        sources: impl IntoIterator<Item = VdMirStmtSource>,
    ) {
        for (stmt, source) in stmts.into_iter().zip(sources) {
            self.stmt_map.insert_next(stmt, source);
        }
    }

    pub(crate) fn set_tactics(
        &mut self,
        tactics: VdMirTacticIdxRange,
        sources: impl IntoIterator<Item = VdMirTacticSource>,
    ) {
        for (tactic, source) in tactics.into_iter().zip(sources) {
            self.tactic_map.insert_next(tactic, source);
        }
    }
}

impl std::ops::Index<VdMirStmtIdx> for VdMirSourceMap {
    type Output = VdMirStmtSource;

    fn index(&self, index: VdMirStmtIdx) -> &Self::Output {
        &self.stmt_map[index]
    }
}

impl std::ops::Index<VdMirTacticIdx> for VdMirSourceMap {
    type Output = VdMirTacticSource;

    fn index(&self, index: VdMirTacticIdx) -> &Self::Output {
        &self.tactic_map[index]
    }
}
