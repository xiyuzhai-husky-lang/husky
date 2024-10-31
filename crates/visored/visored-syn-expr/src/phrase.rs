pub mod noun;

use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use noun::VdSynNounPhraseData;

use crate::expr::VdSynExprIdx;

pub enum VdSynPhrase {
    Expr(VdSynExprIdx),
}

pub enum VdSynPhraseData {
    Noun(VdSynNounPhraseData),
}

pub type VdSynPhraseArena = Arena<VdSynPhraseData>;
pub type VdSynPhraseArenaRef<'a> = ArenaRef<'a, VdSynPhraseData>;
pub type VdSynPhraseIdx = ArenaIdx<VdSynPhraseData>;
pub type VdSynPhraseIdxRange = ArenaIdxRange<VdSynPhraseData>;
