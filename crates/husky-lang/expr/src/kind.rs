use atom::{AtomKind, Literal, Opr, SuffixOpr};
use common::*;

use scope::ScopeId;
use text::TextRange;
use word::Identifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValueExpr {
    range: TextRange,
    kind: ValueExprKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprKind {
    Variable(Identifier),
    Scope(ScopeId),
    Literal(Literal),
    Bracketed(ExprIdx),
    Opn { opn: Opn, opds: ExprRange },
    Void,
}

impl From<&AtomKind> for ExprKind {
    fn from(kind: &AtomKind) -> Self {
        match kind {
            AtomKind::Variable(ident) => ExprKind::Variable(*ident),
            AtomKind::Literal(literal) => ExprKind::Literal(literal.clone()),
            AtomKind::Scope(_) => panic!(),
            AtomKind::Opr(_) => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValueExprKind {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Opn {
    ScopeCall(ScopeId),
    ValueCall,
    MemberCall(Identifier),
    Member(Identifier),
    Index,
    Opr(Opr),
}

impl From<SuffixOpr> for Opn {
    fn from(suffix: SuffixOpr) -> Self {
        Self::Opr(Opr::Suffix(suffix))
    }
}

impl From<Opr> for Opn {
    fn from(opr: Opr) -> Self {
        Self::Opr(opr)
    }
}
