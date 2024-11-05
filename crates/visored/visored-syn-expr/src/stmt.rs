use crate::{
    builder::{ToVdSyn, VdSynExprBuilder},
    sentence::VdSynSentenceIdxRange,
};
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_ast::ast::rose::LxRoseAstIdxRange;
use latex_token::idx::LxTokenIdxRange;

pub enum VdSynStmtData {
    Paragraph(VdSynSentenceIdxRange),
    Block {
        environment: (),
        stmts: VdSynStmtIdxRange,
    },
}

pub type VdSynStmtArena = Arena<VdSynStmtData>;
pub type VdSynStmtArenaRef<'a> = ArenaRef<'a, VdSynStmtData>;
pub type VdSynStmtIdx = ArenaIdx<VdSynStmtData>;
pub type VdSynStmtIdxRange = ArenaIdxRange<VdSynStmtData>;
pub type VdSynStmtMap<T> = ArenaMap<VdSynStmtData, T>;
pub type VdSynStmtOrderedMap<T> = ArenaOrderedMap<VdSynStmtData, T>;

impl ToVdSyn<VdSynStmtIdxRange> for (LxTokenIdxRange, LxRoseAstIdxRange) {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> VdSynStmtIdxRange {
        todo!()
    }
}
