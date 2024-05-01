use super::*;

#[salsa::interned]
pub struct TraitForTypeAssocValEthTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
}
