use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssociatedValEthTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
}
