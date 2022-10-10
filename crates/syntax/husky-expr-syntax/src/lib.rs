mod automata;
mod opn;
mod parser;
mod precedence;
#[cfg(test)]
mod tests;
mod variant;

pub use variant::*;

use automata::*;
use husky_opn_syntax::*;
use husky_text::*;
use husky_token::{Token, TokenKind};
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

impl<'a> Automata<'a> {}
