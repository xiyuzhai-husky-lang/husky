use crate::{
    expr::VdMirExprOrderedMap,
    hint::{VdMirHintIdx, VdMirHintIdxRange, VdMirHintOrderedMap, VdMirHintSource},
    stmt::{VdMirStmtIdx, VdMirStmtIdxRange, VdMirStmtOrderedMap, VdMirStmtSource},
};
use visored_sem_expr::{
    block::VdSemBlockIdx, clause::VdSemClauseIdx, division::VdSemDivisionIdx, expr::VdSemExprIdx,
    sentence::VdSemSentenceIdx,
};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct VdMirSourceMap {
    stmt_map: VdMirStmtOrderedMap<VdMirStmtSource>,
    tactic_map: VdMirHintOrderedMap<VdMirHintSource>,
}

impl VdMirSourceMap {
    pub(crate) fn set_stmts(
        &mut self,
        stmts: VdMirStmtIdxRange,
        sources: impl IntoIterator<Item = VdMirStmtSource>,
    ) {
        let mut sources = sources.into_iter();
        for stmt in stmts {
            // make sure they are of the same length
            let source = sources.next().unwrap();
            self.stmt_map.insert_next(stmt, source);
        }
        debug_assert!(sources.next().is_none());
    }

    pub(crate) fn set_tactics(
        &mut self,
        tactics: VdMirHintIdxRange,
        sources: impl IntoIterator<Item = VdMirHintSource>,
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

impl std::ops::Index<VdMirHintIdx> for VdMirSourceMap {
    type Output = VdMirHintSource;

    fn index(&self, index: VdMirHintIdx) -> &Self::Output {
        &self.tactic_map[index]
    }
}
