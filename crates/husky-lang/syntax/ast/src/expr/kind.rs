use atom::{AtomKind, BinaryOpr, PrefixOpr, SuffixOpr};
use common::*;

use scope::ScopeId;
use text::TextRange;
use word::Identifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprKind {
    Variable(Identifier),
    Scope(ScopeId),
    Literal(Literal),
    Bracketed(RawExprIdx),
    Opn { opr: Opr, opds: RawExprRange },
    Lambda(Vec<(CustomIdentifier, Option<ScopeId>)>, RawExprIdx),
}

impl From<&AtomKind> for RawExprKind {
    fn from(kind: &AtomKind) -> Self {
        match kind {
            AtomKind::Variable(ident) => RawExprKind::Variable(*ident),
            AtomKind::Literal(literal) => RawExprKind::Literal(literal.clone()),
            _ => panic!(),
        }
    }
}
