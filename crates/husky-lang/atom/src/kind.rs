use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomKind {
    Scope(ScopeId),
    Variable(Identifier),
    Literal(Literal),
    Opr(Opr),
}

impl From<Opr> for AtomKind {
    fn from(opr: Opr) -> Self {
        Self::Opr(opr)
    }
}

impl From<BinaryOpr> for AtomKind {
    fn from(opr: BinaryOpr) -> Self {
        opr.into()
    }
}

impl From<PrefixOpr> for AtomKind {
    fn from(opr: PrefixOpr) -> Self {
        opr.into()
    }
}
