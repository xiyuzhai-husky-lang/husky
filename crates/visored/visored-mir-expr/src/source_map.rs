use crate::{
    expr::VdMirExprOrderedMap,
    stmt::{VdMirStmtIdx, VdMirStmtIdxRange, VdMirStmtOrderedMap, VdMirStmtSource},
};
use visored_sem_expr::{
    clause::VdSemClauseIdx, division::VdSemDivisionIdx, expr::VdSemExprIdx,
    sentence::VdSemSentenceIdx, stmt::VdSemStmtIdx,
};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct VdMirSourceMap {
    stmt_map: VdMirStmtOrderedMap<VdMirStmtSource>,
}

impl VdMirSourceMap {
    pub(crate) fn set_stmts(
        &mut self,
        stmts: VdMirStmtIdxRange,
        sources: impl IntoIterator<Item = VdMirStmtSource>,
    ) {
        let mut source_iter = sources.into_iter();
        for stmt in stmts {
            self.stmt_map.insert_next(stmt, source_iter.next().unwrap())
        }
        debug_assert!(source_iter.next().is_none())
    }
}

impl std::ops::Index<VdMirStmtIdx> for VdMirSourceMap {
    type Output = VdMirStmtSource;

    fn index(&self, index: VdMirStmtIdx) -> &Self::Output {
        &self.stmt_map[index]
    }
}
