use crate::clause::VisoredSemClauseIdxRange;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VisoredSemSentenceData {
    Clauses(VisoredSemClauseIdxRange),
}

pub type VisoredSemSentenceArena = Arena<VisoredSemSentenceData>;
pub type VisoredSemSentenceIdx = ArenaIdx<VisoredSemSentenceData>;
pub type VisoredSemSentenceIdxRange = ArenaIdxRange<VisoredSemSentenceData>;
