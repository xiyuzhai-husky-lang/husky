use crate::sentence::VdSemSentenceIdxRange;
use crate::*;
use husky_coword::Coword;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};

pub enum VdSemStmtData {
    Paragraph(VdSemSentenceIdxRange),
    Block {
        environment: (),
        stmts: VdSemStmtIdxRange,
    },
}

pub type VdSemStmtArena = Arena<VdSemStmtData>;
pub type VdSemStmtArenaRef<'a> = ArenaRef<'a, VdSemStmtData>;
pub type VdSemStmtIdx = ArenaIdx<VdSemStmtData>;
pub type VdSemStmtIdxRange = ArenaIdxRange<VdSemStmtData>;
pub type VdSemStmtMap<T> = ArenaMap<VdSemStmtData, T>;
pub type VdSemStmtOrderedMap<T> = ArenaOrderedMap<VdSemStmtData, T>;

impl ToVdSem<VdSemStmtIdxRange> for VdSynStmtIdxRange {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemStmtIdxRange {
        todo!()
    }
}
