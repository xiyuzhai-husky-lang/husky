use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeMethodFnEtherealSignatureTemplate {}

impl TraitForTypeMethodFnEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TraitForTypeItemPath,
        declarative_signature_template: TraitForTypeMethodFnDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(TraitForTypeMethodFnEtherealSignatureTemplate::new(db))
    }
}

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeMethodFnEtherealSignatureTemplatePartiallyInstantiated {}

impl TraitForTypeMethodFnEtherealSignatureTemplatePartiallyInstantiated {
    pub fn try_into_signature(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> Option<TraitForTypeMethodFnEtherealSignature> {
        todo!()
    }
}

pub struct TraitForTypeMethodFnEtherealSignature {}
