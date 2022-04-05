use crate::*;
use syntax_types::{MembAccessDecl, MembCallDecl};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembDecl {
    pub kind: MembDeclKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MembDeclKind {
    Var(MembAccessDecl),
    Routine(MembCallDecl),
}

impl MembDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> MembDecl {
        match self.kind {
            MembDeclKind::Var(ref signature) => MembDecl {
                kind: MembDeclKind::Var(instantiator.instantiate_memb_access_decl(signature)),
            },
            MembDeclKind::Routine(ref signature) => MembDecl {
                kind: MembDeclKind::Routine(instantiator.instantiate_memb_routine_decl(signature)),
            },
        }
    }
}
