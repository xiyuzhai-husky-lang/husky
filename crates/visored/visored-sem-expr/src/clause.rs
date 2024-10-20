use super::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};

pub enum VdSemClauseData {
    Verb,
}

pub type VdSemClauseArena = Arena<VdSemClauseData>;
pub type VdSemClauseArenaRef<'a> = ArenaRef<'a, VdSemClauseData>;
pub type VdSemClauseIdx = ArenaIdx<VdSemClauseData>;
pub type VdSemClauseIdxRange = ArenaIdxRange<VdSemClauseData>;
