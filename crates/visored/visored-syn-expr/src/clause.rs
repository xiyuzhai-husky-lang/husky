use super::*;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};

pub enum VdSynClauseData {
    Verb,
}

pub type VdSynClauseArena = Arena<VdSynClauseData>;
pub type VdSynClauseArenaRef<'a> = ArenaRef<'a, VdSynClauseData>;
pub type VdSynClauseIdx = ArenaIdx<VdSynClauseData>;
pub type VdSynClauseIdxRange = ArenaIdxRange<VdSynClauseData>;
pub type VdSynClauseMap<T> = ArenaMap<VdSynClauseData, T>;
pub type VdSynClauseOrderedMap<T> = ArenaOrderedMap<VdSynClauseData, T>;
