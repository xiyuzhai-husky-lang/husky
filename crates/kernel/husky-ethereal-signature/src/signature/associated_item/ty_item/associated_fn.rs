use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeAssociatedFnEtherealSignatureTemplate {
    pub self_ty: EtherealTerm,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterEtherealSignatures,
    #[return_ref]
    pub regular_parameters: ExplicitParameterEtherealSignatureTemplates,
    pub return_ty: EtherealTerm,
}
