use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitVariadicParameterEtherealSignatureTemplate {
    contract: Contract,
    ty: EtherealTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitVariadicParameterEtherealSignature {
    contract: Contract,
    ty: EtherealTerm,
}
