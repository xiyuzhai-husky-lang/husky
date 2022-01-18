mod opn;
mod parser;

pub use opn::*;
pub use parser::ExprParser;

use scope::ScopeId;
use syntax_types::*;
use text::TextRange;
use word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    pub range: TextRange,
    pub ty: ScopeId,
    pub kind: ExprKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    Variable(CustomIdentifier),
    Scope {
        id: ScopeId,
        compiled: Option<()>,
    },
    Literal(PrimitiveValue),
    Bracketed(Box<Expr>),
    Opn {
        opn: Opn,
        compiled: Option<()>,
        opds: Vec<Expr>,
    },
    Lambda(Vec<(CustomIdentifier, Option<ScopeId>)>, Box<Expr>),
}
