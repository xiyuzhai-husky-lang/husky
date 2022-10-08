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
    pub fn ty_family(&self) -> TyFamily {
        self.ty_family
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TyFamily {
    Physics,
    Concept,
    Monad,
    Prop,
}
