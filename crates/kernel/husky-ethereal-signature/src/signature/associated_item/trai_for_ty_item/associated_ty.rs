use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssociatedTypeEtherealSignatureTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub implicit_parameters: EtherealGenericParameters,
}

impl TraitForTypeAssociatedTypeEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TraitForTypeItemPath,
        declarative_signature_template: TraitForTypeAssociatedTypeDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let implicit_parameters = EtherealGenericParameters::from_declarative(
            db,
            declarative_signature_template.implicit_parameters(db),
        )?;
        Ok(Self::new(db, path, implicit_parameters))
    }
}
