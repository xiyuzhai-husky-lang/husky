pub mod context;
mod convexity;
mod error;
pub mod parser;
mod stack;
mod variant;

pub use context::*;
pub use error::*;
use husky_print_utils::p;
pub use parser::AtomParser;
pub use parser::*;
pub use variant::{HuskyAtomVariant, LambdaHead};

use husky_entity_kind::*;
use husky_entity_route::EntityRouteItd;
use husky_liason_semantics::*;
use husky_opn_syntax::*;
use husky_text::TextRange;
use husky_text::TextRanged;
use husky_token::Convexity;
use husky_token::{Token, TokenKind};
use husky_word::Identifier;
use husky_word::RootBuiltinIdentifier;

#[derive(Clone, PartialEq, Eq)]
pub struct HuskyAtom {
    pub range: TextRange,
    pub variant: HuskyAtomVariant,
}

impl HuskyAtom {
    pub fn new(range: TextRange, variant: HuskyAtomVariant) -> HuskyAtom {
        HuskyAtom { range, variant }
    }
}

impl TextRanged for HuskyAtom {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

impl std::fmt::Debug for HuskyAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Atom{{range: {:?}, kind: {:?}}}",
            &self.range, &self.variant
        ))
    }
}

impl From<&Token> for HuskyAtom {
    fn from(token: &Token) -> Self {
        match token.kind {
            TokenKind::Keyword(_) | TokenKind::Identifier(_) => {
                p!(token.kind);
                panic!()
            }
            TokenKind::Special(special) => HuskyAtom::new(token.text_range(), special.into()),
            TokenKind::Literal(i) => HuskyAtom::new(token.text_range(), i.into()),
            TokenKind::WordOpr(word_opr) => HuskyAtom::new(token.text_range(), word_opr.into()),
            TokenKind::Unrecognized(_) => todo!(),
            TokenKind::IllFormedLiteral(_) => todo!(),
            TokenKind::Decorator(_) => todo!(),
            TokenKind::WordPattern(patt) => {
                HuskyAtom::new(token.text_range(), HuskyAtomVariant::WordPattern { patt })
            }
        }
    }
}
