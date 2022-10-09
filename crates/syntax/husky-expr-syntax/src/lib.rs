mod automata;
mod opn;
mod parser;
mod precedence;
mod variant;

pub use automata::*;
pub use variant::*;

use husky_opn_syntax::*;
use husky_text::*;
use husky_word::*;
use precedence::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawExpr {
    pub range: TextRange,
    pub variant: RawExprVariant,
}

use ::arena::{map::ArenaMap, Arena, ArenaIdx, ArenaRange};

pub type RawExprArena = Arena<RawExpr>;
pub type RawExprIdx = ArenaIdx<RawExpr>;
pub type RawExprRange = ArenaRange<RawExpr>;
