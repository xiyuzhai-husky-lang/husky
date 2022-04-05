use crate::*;
use syntax_types::{MembAccessDecl, MembCallDecl};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembSignature {
    pub kind: MembSignatureKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MembSignatureKind {
    Var(MembAccessDecl),
    Routine(MembCallDecl),
}

impl MembSignature {
    pub fn instantiate(&self, instantiator: &Instantiator) -> MembSignature {
        match self.kind {
            MembSignatureKind::Var(ref signature) => MembSignature {
                kind: MembSignatureKind::Var(instantiator.instantiate_memb_access_decl(signature)),
            },
            MembSignatureKind::Routine(ref signature) => MembSignature {
                kind: MembSignatureKind::Routine(
                    instantiator.instantiate_memb_routine_decl(signature),
                ),
            },
        }
    }
}
