use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssociatedFnEtherealSignatureTemplate {
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    #[return_ref]
    pub parenate_parameters: EtherealParenateParameters,
    pub return_ty: EtherealTerm,
}
