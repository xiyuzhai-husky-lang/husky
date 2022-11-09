use crate::*;
use husky_word::IdentPairDict;

pub trait AskDecl {
    fn ask_namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl>;
    fn ask_ty_decl(&self, ty: Ty) -> TermResultArc<TyDecl>;
    fn ask_decl(&self, entity_path: EntityPathItd) -> TermResultArc<Decl>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Decl {
    Module,
    Ty(TyDecl),
}

#[derive(Debug, PartialEq, Eq)]
pub struct NamespaceDecl {
    members: IdentPairDict<TermOwned>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TyDecl {
    ty_family: TyFamily,
    parameters: Vec<Parameter>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parameter {/*todo */}

impl TyDecl {
    pub fn new(ty_family: TyFamily) -> Self {
        Self {
            ty_family,
            parameters: vec![],
        }
    }

    pub fn ty_family(&self) -> TyFamily {
        self.ty_family
    }

    pub fn parameters(&self) -> &[Parameter] {
        &self.parameters
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TyFamily {
    Physical,
    Conceptual,
    Monadic,
    Propositional,
}
