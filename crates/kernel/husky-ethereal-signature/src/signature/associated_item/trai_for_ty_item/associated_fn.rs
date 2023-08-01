use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssociatedFnEtherealSignatureTemplate {
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
    #[return_ref]
    pub parenate_parameters: EtherealTermParenateParameters,
    pub return_ty: EtherealTerm,
}
