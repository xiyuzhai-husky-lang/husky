use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DeriveDecrEtherealSignatureTemplate {}

pub trait HasDeriveDecrEtherealSignatureTemplates: Copy {
    fn derive_decr_ethereal_signature_templates(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<&[DeriveDecrEtherealSignatureTemplate]>;
}

impl HasDeriveDecrEtherealSignatureTemplates for TypePath {
    fn derive_decr_ethereal_signature_templates(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<&[DeriveDecrEtherealSignatureTemplate]> {
        Ok(ty_path_derive_decr_ethereal_signature_templates(db, self).as_ref()?)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn ty_path_derive_decr_ethereal_signature_templates(
    db: &dyn EtherealSignatureDb,
    ty_path: TypePath,
) -> EtherealSignatureResult<SmallVec<[DeriveDecrEtherealSignatureTemplate; 8]>> {
    ty_path.derive_decr_declarative_signature_templates(db);
    todo!()
}
