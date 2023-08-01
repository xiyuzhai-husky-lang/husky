use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssociatedValEtherealSignatureTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
}
