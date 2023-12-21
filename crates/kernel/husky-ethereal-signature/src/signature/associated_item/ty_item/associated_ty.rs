use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeAssociatedTypeEtherealSignatureTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    pub associated_ty: EtherealTerm,
}
