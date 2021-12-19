mod error;
mod group;
mod kind;
mod opr;
mod parser;
mod query;
mod scope_alias;

pub use error::{AtomError, AtomRule};
pub use group::{AtomGroup, GroupAttr};
pub use kind::{AtomKind, Literal};
pub use opr::{BinaryOpr, Bracket, JoinOpr, Opr, PrefixOpr, SuffixOpr};
pub use parser::AtomParser;
pub use query::{AtomQuery, AtomQueryStorage, AtomizedText};

use scope::ScopeId;
use text::TextRange;
use token::{Token, TokenKind};
use word::{Identifier, Keyword};

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::I32Literal(l0), Self::I32Literal(r0)) => l0 == r0,
            (Self::F32Literal(l0), Self::F32Literal(r0)) => l0.to_bits() == r0.to_bits(),
            _ => false,
        }
    }
}

impl Eq for Literal {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Atom {
    pub range: TextRange,
    pub kind: AtomKind,
}

pub type AtomResult = Result<AtomGroup, AtomError>;

impl Atom {
    pub fn new(range: TextRange, variant: AtomKind) -> Atom {
        Atom {
            range,
            kind: variant,
        }
    }
}

impl From<&Token> for Atom {
    fn from(token: &Token) -> Self {
        match &token.kind {
            TokenKind::Keyword(_) => panic!(),
            TokenKind::Identifier(ident) => Atom::new(token.range, ident.into()),
            TokenKind::Special(special) => {
                let opr: Opr = special.into();
                Atom::new(token.range, opr.into())
            }
            TokenKind::I32Literal(i) => Atom::new(token.range, i.into()),
            TokenKind::F32Literal(f) => Atom::new(token.range, f.into()),
        }
    }
}
