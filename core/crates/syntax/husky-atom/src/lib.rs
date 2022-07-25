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
pub use variant::{AtomVariant, LambdaHead};

use entity_kind::*;
use husky_entity_route::EntityRoutePtr;
use husky_liason_semantics::*;
use husky_text::TextRange;
use husky_text::TextRanged;
use husky_token::Convexity;
use husky_token::{HuskyToken, HuskyTokenKind};
use husky_word::Identifier;
use husky_word::RootIdentifier;

#[derive(Clone, PartialEq, Eq)]
pub struct HuskyAtom {
    pub range: TextRange,
    pub variant: AtomVariant,
}

impl HuskyAtom {
    pub fn new(range: TextRange, kind: AtomVariant) -> HuskyAtom {
        HuskyAtom {
            range,
            variant: kind,
        }
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

impl From<&HuskyToken> for HuskyAtom {
    fn from(token: &HuskyToken) -> Self {
        match token.kind {
            HuskyTokenKind::Keyword(_) | HuskyTokenKind::Identifier(_) => {
                p!(token.kind);
                panic!()
            }
            HuskyTokenKind::Special(special) => HuskyAtom::new(token.text_range(), special.into()),
            HuskyTokenKind::PrimitiveLiteral(i) => HuskyAtom::new(token.text_range(), i.into()),
            HuskyTokenKind::WordOpr(word_opr) => {
                HuskyAtom::new(token.text_range(), word_opr.into())
            }
            HuskyTokenKind::Unrecognized(_) => todo!(),
            HuskyTokenKind::IllFormedLiteral(_) => todo!(),
            HuskyTokenKind::Decorator(_) => todo!(),
        }
    }
}
