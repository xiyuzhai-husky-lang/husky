use vec_like::{SmallVecPairMap, VecMapGetEntry};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DeriveDecrEtherealSignatureTemplate {}

pub trait HasDeriveDecrEtherealSignatureTemplates: Copy {
    fn derive_decr_ethereal_signature_templates_map(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &[(
            TraitPath,
            SmallVec<[DeriveDecrEtherealSignatureTemplate; 1]>,
        )],
    >;

    fn derive_decr_ethereal_signature_templates(
        self,
        db: &dyn EtherealSignatureDb,
        trai_path: TraitPath,
    ) -> EtherealSignatureResult<Option<&[DeriveDecrEtherealSignatureTemplate]>> {
        Ok(self
            .derive_decr_ethereal_signature_templates_map(db)?
            .get_entry(trai_path)
            .map(|(_, templates)| templates as &[_]))
    }
}

impl HasDeriveDecrEtherealSignatureTemplates for TypePath {
    fn derive_decr_ethereal_signature_templates_map(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &[(
            TraitPath,
            SmallVec<[DeriveDecrEtherealSignatureTemplate; 1]>,
        )],
    > {
        Ok(ty_path_derive_decr_ethereal_signature_templates_map(db, self).as_ref()?)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn ty_path_derive_decr_ethereal_signature_templates_map(
    db: &dyn EtherealSignatureDb,
    ty_path: TypePath,
) -> EtherealSignatureResult<
    SmallVecPairMap<TraitPath, SmallVec<[DeriveDecrEtherealSignatureTemplate; 1]>, 8>,
> {
    Ok(ty_path
        .derive_decr_declarative_signature_templates_map(db)?
        .iter()
        .map(|_| todo!())
        .collect())
}
