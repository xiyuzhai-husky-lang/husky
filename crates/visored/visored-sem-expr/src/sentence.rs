use crate::clause::VdSemClauseIdxRange;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdSemSentenceData {
    Clauses(VdSemClauseIdxRange),
}

pub type VdSemSentenceArena = Arena<VdSemSentenceData>;
pub type VdSemSentenceIdx = ArenaIdx<VdSemSentenceData>;
pub type VdSemSentenceIdxRange = ArenaIdxRange<VdSemSentenceData>;
