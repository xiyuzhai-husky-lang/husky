use crate::clause::VdSemClauseIdxRange;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdSemSentenceData {
    Clauses(VdSemClauseIdxRange),
}

pub type VdSemSentenceArena = Arena<VdSemSentenceData>;
pub type VdSemSentenceArenaRef<'a> = ArenaRef<'a, VdSemSentenceData>;
pub type VdSemSentenceIdx = ArenaIdx<VdSemSentenceData>;
pub type VdSemSentenceIdxRange = ArenaIdxRange<VdSemSentenceData>;
