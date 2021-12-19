mod error;
mod group;
mod kind;
mod opr;
mod parser;
mod query;
mod scope_alias;

pub use error::{AtomError, AtomRule};
pub use group::{AtomGroup, GroupAttr};
pub use kind::AtomKind;
pub use opr::{BinaryOpr, Bracket, JoinOpr, Opr, PrefixOpr, SuffixOpr};
pub use parser::AtomParser;
pub use query::{AtomQuery, AtomQueryStorage, AtomizedText};

use scope::ScopeId;
use text::TextRange;
use token::{Token, TokenKind};
use word::{Identifier, Keyword};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    IntegerLiteral(String),
    FloatLiteral(String),
}

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
            TokenKind::Identifier(ident) => Atom::new(token.range, AtomKind::Variable(*ident)),
            TokenKind::Special(special) => {
                let opr: Opr = special.into();
                Atom::new(token.range, opr.into())
            }
            TokenKind::IntegerLiteral(string) => Atom::new(
                token.range,
                AtomKind::Literal(Literal::IntegerLiteral(string.clone())),
            ),
            TokenKind::FloatLiteral(string) => Atom::new(
                token.range,
                AtomKind::Literal(Literal::IntegerLiteral(string.clone())),
            ),
        }
    }
}
