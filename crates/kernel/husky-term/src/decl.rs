use crate::*;
use husky_word::IdentPairMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Decl {
    Module,
    Term(TyDecl),
}

#[derive(Debug, PartialEq, Eq)]
pub struct NamespaceDecl {
    members: IdentPairMap<TermData>,
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
