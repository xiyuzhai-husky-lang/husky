use crate::clause::VdSemClauseIdxRange;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_token::idx::LxRoseTokenIdx;

#[derive(Debug, PartialEq, Eq)]
pub enum VdSemSentenceData {
    Clauses {
        clauses: VdSemClauseIdxRange,
        end: VdSemSentenceEnd,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSemSentenceEnd {
    Period(LxRoseTokenIdx),
}

pub type VdSemSentenceArena = Arena<VdSemSentenceData>;
pub type VdSemSentenceArenaRef<'a> = ArenaRef<'a, VdSemSentenceData>;
pub type VdSemSentenceIdx = ArenaIdx<VdSemSentenceData>;
pub type VdSemSentenceIdxRange = ArenaIdxRange<VdSemSentenceData>;
pub type VdSemSentenceMap<T> = ArenaMap<VdSemSentenceData, T>;
