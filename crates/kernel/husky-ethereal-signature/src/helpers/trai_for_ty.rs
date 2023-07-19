use super::*;

pub struct TraitForTypeImplBlockEtherealSignatureTemplates<'a> {
    trai_side_derive_any: &'a [TraitForTypeImplBlockEtherealSignatureTemplate],
    trai_side_path_leading: &'a [TraitForTypeImplBlockEtherealSignatureTemplate],
    ty_side: &'a [TraitForTypeImplBlockEtherealSignatureTemplate],
}

impl<'a> TraitForTypeImplBlockEtherealSignatureTemplates<'a> {
    fn iter(self) -> impl Iterator<Item = TraitForTypeImplBlockEtherealSignatureTemplate> + 'a {
        (self
            .trai_side_derive_any
            .iter()
            .chain(self.trai_side_path_leading.iter())
            .chain(self.ty_side.iter()))
        .copied()
    }
}

pub fn trai_for_ty_impl_block_ethereal_signature_templates<'a>(
    db: &'a dyn EtherealSignatureDb,
    trai_path: TraitPath,
    ty_path: TypePath,
) -> EtherealSignatureResult<TraitForTypeImplBlockEtherealSignatureTemplates<'a>> {
    let ty_path_derives_trai_path = ty_path
        .derive_decr_ethereal_signature_templates(db, trai_path)?
        .is_some();
    Ok(TraitForTypeImplBlockEtherealSignatureTemplates {
        trai_side_derive_any: if ty_path_derives_trai_path {
            trai_side_derive_any_ethereal_signature_templates(db, trai_path)
        } else {
            &[]
        },
        // todo: ty_kind
        trai_side_path_leading: todo!(),
        ty_side: ty_path
            .ty_side_trai_for_ty_impl_block_signature_templates(db, trai_path)
            .into_result_option()?
            .unwrap_or(&[]),
    })
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn trai_side_derive_any_ethereal_signature_templates(
    db: &dyn EtherealSignatureDb,
    trai_path: TraitPath,
) -> SmallVec<[TraitForTypeImplBlockEtherealSignatureTemplate; 2]> {
    todo!()
}

pub trait HasTypeSideTraitForTypeImplBlockSignatureTemplates: Copy {
    fn ty_side_trai_for_ty_impl_block_signature_templates<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        trai_path: TraitPath,
    ) -> EtherealSignatureMaybeResult<&'a [TraitForTypeImplBlockEtherealSignatureTemplate]>;
}

impl HasTypeSideTraitForTypeImplBlockSignatureTemplates for TypePath {
    fn ty_side_trai_for_ty_impl_block_signature_templates<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        key: TraitPath,
    ) -> EtherealSignatureMaybeResult<&'a [TraitForTypeImplBlockEtherealSignatureTemplate]> {
        match ty_side_impl_block_signature_templates_map(db, self).get_value(key) {
            Some(result) => match result {
                Ok(templates) => JustOk(templates),
                Err(e) => JustErr(*e),
            },
            None => Nothing,
        }
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn ty_side_impl_block_signature_templates_map(
    db: &dyn EtherealSignatureDb,
    ty_path: TypePath,
) -> SmallVecPairMap<TraitPath, EtherealSignatureResult<TraitForTypeImplBlockSignatureTemplates>, 2>
{
    let map = ty_path.ty_side_trai_for_ty_impl_block_paths_map(db);
    map.map_collect(|paths| {
        paths
            .iter()
            .map(|path| path.ethereal_signature_template(db))
            .collect()
    })
}
