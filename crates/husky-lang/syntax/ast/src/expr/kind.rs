use atom::AtomKind;
use common::*;

use crate::*;
use scope::{ScopeKind, ScopePtr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprKind {
    Variable(CustomIdentifier),
    Scope(ScopePtr, ScopeKind),
    Literal(PrimitiveValue),
    Bracketed(RawExprIdx),
    Opn { opr: Opr, opds: RawExprRange },
    Lambda(Vec<(CustomIdentifier, Option<ScopePtr>)>, RawExprIdx),
}

impl From<&AtomKind> for RawExprKind {
    fn from(kind: &AtomKind) -> Self {
        match kind {
            AtomKind::Variable(ident) => RawExprKind::Variable(*ident),
            AtomKind::Literal(literal) => RawExprKind::Literal(literal.clone()),
            AtomKind::Scope(scope, kind) => RawExprKind::Scope(*scope, *kind),
            _ => {
                p!(kind);
                panic!()
            }
        }
    }
}
