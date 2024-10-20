pub mod noun;

use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use noun::VdSemNounPhraseData;

use crate::expr::VdSemExprIdx;

pub enum VdSemPhrase {
    Expr(VdSemExprIdx),
}

pub enum VdSemPhraseData {
    Noun(VdSemNounPhraseData),
}

pub type VdSemPhraseArena = Arena<VdSemPhraseData>;
pub type VdSemPhraseIdx = ArenaIdx<VdSemPhraseData>;
pub type VdSemPhraseIdxRange = ArenaIdxRange<VdSemPhraseData>;
