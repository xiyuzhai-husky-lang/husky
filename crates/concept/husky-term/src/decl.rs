use crate::*;
use husky_word::IdentPairDict;

pub trait AskDecl {
    fn ask_namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl>;
    fn ask_ty_decl(&self, ty: Ty) -> TermResultArc<TyDecl>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct NamespaceDecl {
    members: IdentPairDict<Term>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TyDecl {
    ty_family: TyFamily,
}

impl TyDecl {
    pub fn new(ty_family: TyFamily) -> Self {
        Self { ty_family }
    }

    pub fn ty_family(&self) -> TyFamily {
        self.ty_family
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TyFamily {
    Physical,
    Conceptual,
    Monadic,
    Propositional,
}
