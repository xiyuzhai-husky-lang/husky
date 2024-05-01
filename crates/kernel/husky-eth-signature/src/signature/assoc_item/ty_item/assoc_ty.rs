use super::*;

#[salsa::interned]
pub struct TypeAssocTypeEthTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub assoc_ty: EthTerm,
}
