use atom::AtomKind;
use common::*;

use crate::*;
use scope::{ScopeId, ScopeKind};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprKind {
    Variable(CustomIdentifier),
    Scope(ScopeId, ScopeKind),
    Literal(PrimitiveValue),
    Bracketed(RawExprIdx),
    Opn { opr: Opr, opds: RawExprRange },
    Lambda(Vec<(CustomIdentifier, Option<ScopeId>)>, RawExprIdx),
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
