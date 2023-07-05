use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitKeyedParameterEtherealSignatureTemplate {
    key: Ident,
    contract: Contract,
    ty: EtherealTerm,
    default: Option<EtherealTerm>,
}

impl ExplicitKeyedParameterEtherealSignatureTemplate {
    pub fn key(&self) -> Ident {
        self.key
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }

    pub fn default(&self) -> Option<EtherealTerm> {
        self.default
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitKeyedParameterEtherealSignature {
    key: Ident,
    contract: Contract,
    ty: EtherealTerm,
    default: Option<EtherealTerm>,
}
