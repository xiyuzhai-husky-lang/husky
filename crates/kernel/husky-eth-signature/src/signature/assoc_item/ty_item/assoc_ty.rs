use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EthSignatureJar)]
pub struct TypeAssocTypeEthTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub assoc_ty: EthTerm,
}
