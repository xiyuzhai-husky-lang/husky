mod builder;
mod opn;
mod parser;

pub(crate) use builder::ExprInstructionBuilder;
pub use opn::*;
pub(crate) use parser::ExprParser;

use scope::ScopeId;
use syntax_types::*;
use text::TextRange;
use virtual_stack::{Compiled, VirtualStack, VirtualStackResult};
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
        compiled: Option<Compiled>,
    },
    Literal(PrimitiveValue),
    Bracketed(Box<Expr>),
    Opn {
        opn: Opn,
        compiled: Option<Compiled>,
        opds: Vec<Expr>,
    },
    Lambda(Vec<(CustomIdentifier, Option<ScopeId>)>, Box<Expr>),
}
