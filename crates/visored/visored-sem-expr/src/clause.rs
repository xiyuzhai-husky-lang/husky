use super::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

pub enum VdSemClauseData {
    Verb,
}

pub type VdSemClauseArena = Arena<VdSemClauseData>;
pub type VdSemClauseIdx = ArenaIdx<VdSemClauseData>;
pub type VdSemClauseIdxRange = ArenaIdxRange<VdSemClauseData>;
