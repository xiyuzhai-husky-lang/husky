mod convexity;
mod error;
mod kind;
pub(crate) mod parser;
mod stack;
pub(crate) mod symbol_proxy;

pub(crate) use kind::{AtomKind, LambdaHead};
pub(crate) use parser::parse_ty;
pub(crate) use syntax_types::*;

use entity_route::EntityRoutePtr;
use text::TextRange;
use text::TextRanged;
use token::{Token, TokenKind};
use word::Identifier;
use word::RootIdentifier;

#[derive(Clone, PartialEq, Eq)]
pub struct Atom {
    range: TextRange,
    pub kind: AtomKind,
}

impl Atom {
    pub fn new(range: TextRange, kind: AtomKind) -> Atom {
        Atom { range, kind }
    }
}

impl TextRanged for Atom {
    fn text_range_ref(&self) -> &TextRange {
        &self.range
    }
}

impl std::fmt::Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Atom{{{:?}, {:?}}}", &self.range, &self.kind))
    }
}

impl From<&Token> for Atom {
    fn from(token: &Token) -> Self {
        match token.kind {
            TokenKind::Keyword(_) | TokenKind::Identifier(_) => panic!(),
            TokenKind::Special(special) => Atom::new(token.text_range(), special.into()),
            TokenKind::I32Literal(i) => Atom::new(token.text_range(), i.into()),
            TokenKind::F32Literal(f) => Atom::new(token.text_range(), f.into()),
        }
    }
}
