use scope_query::Instantiator;
use syntax_types::{MembAccessSignature, MembCallSignature};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembSignature {
    pub kind: MembSignatureKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MembSignatureKind {
    Var(MembAccessSignature),
    Routine(MembCallSignature),
}

impl MembSignature {
    pub fn instantiate(&self, instantiator: &Instantiator) -> MembSignature {
        match self.kind {
            MembSignatureKind::Var(ref signature) => MembSignature {
                kind: MembSignatureKind::Var(
                    instantiator.instantiate_memb_access_signature(signature),
                ),
            },
            MembSignatureKind::Routine(ref signature) => MembSignature {
                kind: MembSignatureKind::Routine(
                    instantiator.instantiate_memb_routine_signature(signature),
                ),
            },
        }
    }
}
