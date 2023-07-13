use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssociatedTypeEtherealSignatureTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
}

impl TraitForTypeAssociatedTypeEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TraitForTypeItemPath,
        declarative_signature_template: TraitForTypeAssociatedTypeDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let generic_parameters = EtherealGenericParameters::from_declarative(
            db,
            declarative_signature_template.generic_parameters(db),
        )?;
        Ok(Self::new(db, path, generic_parameters))
    }
}

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar, constructor = pub(super) new)]
pub struct TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated {
    pub template: TraitForTypeAssociatedTypeEtherealSignatureTemplate,
    #[return_ref]
    pub partial_instantiation: EtherealTermPartialInstantiation,
}

impl TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated {
    pub fn try_into_signature(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> Option<TraitForTypeAssociatedTypeEtherealSignature> {
        let instantiation = self.partial_instantiation(db).try_into_instantiation()?;
        Some(TraitForTypeAssociatedTypeEtherealSignature {})
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssociatedTypeEtherealSignature {}
