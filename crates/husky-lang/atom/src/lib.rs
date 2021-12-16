mod opr;

pub use opr::{BinaryOpr, Bracket, JoinOpr, Opr, Precedence, PrefixOpr, SuffixOpr};

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
