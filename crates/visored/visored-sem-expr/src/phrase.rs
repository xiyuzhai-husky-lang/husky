pub mod noun;

use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use noun::VisoredSemNounPhraseData;

use crate::expr::VisoredSemExprIdx;

pub enum VisoredSemPhrase {
    Expr(VisoredSemExprIdx),
}

pub enum VisoredSemPhraseData {
    Noun(VisoredSemNounPhraseData),
}

pub type VisoredSemPhraseArena = Arena<VisoredSemPhraseData>;
pub type VisoredSemPhraseIdx = ArenaIdx<VisoredSemPhraseData>;
pub type VisoredSemPhraseIdxRange = ArenaIdxRange<VisoredSemPhraseData>;
