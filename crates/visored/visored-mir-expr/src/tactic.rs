use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};

pub enum VdMirTacticData {
    Ring,
}

pub struct VdMirTacticEntry {
    data: VdMirTacticData,
}

pub type VdMirTacticIdx = ArenaIdx<VdMirTacticEntry>;
pub type VdMirTacticIdxRange = ArenaIdxRange<VdMirTacticEntry>;
pub type VdMirTacticArena = Arena<VdMirTacticEntry>;
pub type VdMirTacticArenaRef<'a> = ArenaRef<'a, VdMirTacticEntry>;
