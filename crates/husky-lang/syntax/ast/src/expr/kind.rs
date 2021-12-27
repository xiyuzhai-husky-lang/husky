use atom::{AtomKind, BinaryOpr, Literal, PrefixOpr, SuffixOpr};
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
    Opn { opr: Opr, opds: ExprRange },
    Void,
}

impl From<&AtomKind> for ExprKind {
    fn from(kind: &AtomKind) -> Self {
        match kind {
            AtomKind::Variable(ident) => ExprKind::Variable(*ident),
            AtomKind::Literal(literal) => ExprKind::Literal(literal.clone()),
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValueExprKind {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Opr {
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(SuffixOpr),
    List(ListOpr),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ListOpr {
    TupleInit,
    NewVec,
    NewDict,
    Call,
    Index,
    ModuloIndex,
    StructInit,
}

impl From<BinaryOpr> for Opr {
    fn from(binary: BinaryOpr) -> Self {
        Self::Binary(binary)
    }
}

impl From<PrefixOpr> for Opr {
    fn from(prefix: PrefixOpr) -> Self {
        Self::Prefix(prefix)
    }
}

impl From<SuffixOpr> for Opr {
    fn from(suffix: SuffixOpr) -> Self {
        Self::Suffix(suffix)
    }
}

impl From<ListOpr> for Opr {
    fn from(list: ListOpr) -> Self {
        Self::List(list)
    }
}
