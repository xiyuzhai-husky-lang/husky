use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeMethodFnEtherealSignatureTemplate {}

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
