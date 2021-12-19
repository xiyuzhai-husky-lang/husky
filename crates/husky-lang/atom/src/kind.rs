use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomKind {
    Scope(ScopeId),
    Variable(Identifier),
    Literal(Literal),
    Opr(Opr),
}

impl From<ScopeId> for AtomKind {
    fn from(scope_id: ScopeId) -> Self {
        Self::Scope(scope_id)
    }
}

impl From<Identifier> for AtomKind {
    fn from(ident: Identifier) -> Self {
        Self::Variable(ident)
    }
}

impl From<&Identifier> for AtomKind {
    fn from(ident: &Identifier) -> Self {
        Self::Variable(*ident)
    }
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

impl From<Literal> for AtomKind {
    fn from(lit: Literal) -> Self {
        Self::Literal(lit)
    }
}

impl From<&i32> for AtomKind {
    fn from(i: &i32) -> Self {
        Literal::I32Literal(*i).into()
    }
}

impl From<&f32> for AtomKind {
    fn from(f: &f32) -> Self {
        Literal::F32Literal(*f).into()
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    I32Literal(i32),
    F32Literal(f32),
}

impl From<i32> for Literal {
    fn from(i: i32) -> Self {
        Self::I32Literal(i)
    }
}

impl From<f32> for Literal {
    fn from(f: f32) -> Self {
        Self::F32Literal(f)
    }
}

impl From<&i32> for Literal {
    fn from(i: &i32) -> Self {
        Self::I32Literal(*i)
    }
}

impl From<&f32> for Literal {
    fn from(f: &f32) -> Self {
        Self::F32Literal(*f)
    }
}
