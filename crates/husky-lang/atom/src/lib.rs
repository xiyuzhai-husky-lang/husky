mod error;
mod opr;
mod parser;

pub use error::AtomError;
pub use opr::{BinaryOpr, Bracket, JoinOpr, Opr, Precedence, PrefixOpr, SuffixOpr};
pub use parser::AtomParser;

use scope::ScopeId;
use text::TextRange;
use word::Identifier;

pub enum Literal {}

pub struct Atom {
    pub range: TextRange,
    pub variant: AtomVariant,
}

pub enum AtomVariant {
    Scope { is_generic: bool, id: ScopeId },
    Variable(Identifier),
    Literal(Literal),
    Opr(Opr, Precedence),
}

impl Atom {
    pub fn parse(_token_group: &[token::Token]) -> (Vec<Atom>, Vec<AtomError>) {
        todo!()
    }
}
