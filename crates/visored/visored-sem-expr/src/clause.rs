use super::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

pub enum VisoredSemClauseData {
    Verb,
}

pub type VisoredSemClauseArena = Arena<VisoredSemClauseData>;
pub type VisoredSemClauseIdx = ArenaIdx<VisoredSemClauseData>;
pub type VisoredSemClauseIdxRange = ArenaIdxRange<VisoredSemClauseData>;
