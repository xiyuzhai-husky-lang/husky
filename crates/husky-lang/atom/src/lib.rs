mod error;
mod group;
mod kind;
mod opr;
mod parser;
mod query;
mod scope_alias;

pub use error::{AtomError, AtomResult, AtomResultArc, AtomRule};
pub use group::{AtomGroup, GroupAttr};
pub use kind::{AtomKind, LambdaHead, Literal};
pub use opr::{BinaryOpr, Bracket, JoinOpr, ListEndAttr, ListStartAttr, PrefixOpr, SuffixOpr};
pub use parser::AtomParser;
pub use query::{AtomQuery, AtomQueryStorage, AtomizedText};

use error::atom_err;
use scope::GenericArgument;
use scope::ScopeId;
use text::HasTextRange;
use text::TextRange;
use token::{Token, TokenKind};
use word::BuiltinIdentifier;
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

#[derive(Clone, PartialEq, Eq)]
pub struct Atom {
    range: TextRange,
    pub kind: AtomKind,
}

impl Atom {
    pub fn new(range: TextRange, variant: AtomKind) -> Atom {
        Atom {
            range,
            kind: variant,
        }
    }
}

impl HasTextRange for Atom {
    fn text_range_ref(&self) -> &TextRange {
        &self.range
    }
}

impl std::fmt::Debug for Atom {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Atom{{{:?}, {:?}}}", &self.range, &self.kind))
        // .debug_struct("Atom")
        //     .field("range", &self.range)
        //     .field("kind", &self.kind)
        //     .finish()
    }
}

pub type AtomParseResult = Result<AtomGroup, AtomError>;

impl From<&Token> for Atom {
    fn from(token: &Token) -> Self {
        match &token.kind {
            TokenKind::Keyword(_) => panic!(),
            TokenKind::Identifier(ident) => Atom::new(token.text_range(), ident.into()),
            TokenKind::Special(special) => Atom::new(token.text_range(), special.into()),
            TokenKind::I32Literal(i) => Atom::new(token.text_range(), i.into()),
            TokenKind::F32Literal(f) => Atom::new(token.text_range(), f.into()),
        }
    }
}
