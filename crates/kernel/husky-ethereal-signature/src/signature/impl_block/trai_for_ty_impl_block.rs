use super::*;
use husky_entity_tree::{EntityTreeBundleResult, HasTypeSideTraitForTypeImplBlockPathsMap};
use smallvec::SmallVec;
use vec_like::SmallVecPairMap;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeImplBlockEtherealSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterEtherealSignatures,
    pub trai: EtherealTerm,
    pub ty: EtherealTerm,
    // todo: where clause
}

impl HasEtherealSignatureTemplate for TraitForTypeImplBlockPath {
    type EtherealSignatureTemplate = TraitForTypeImplBlockEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        trai_for_ty_impl_block_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn trai_for_ty_impl_block_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    path: TraitForTypeImplBlockPath,
) -> EtherealSignatureResult<TraitForTypeImplBlockEtherealSignatureTemplate> {
    TraitForTypeImplBlockEtherealSignatureTemplate::from_declarative(
        db,
        path.declarative_signature_template(db)?,
    )
}

impl TraitForTypeImplBlockEtherealSignatureTemplate {
    fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature_template: TraitForTypeImplBlockDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let implicit_parameters = ImplicitParameterEtherealSignatures::from_declarative(
            db,
            declarative_signature_template.implicit_parameters(db),
        )?;
        let trai = EtherealTerm::ty_from_declarative(db, declarative_signature_template.trai(db))?;
        let ty = EtherealTerm::ty_from_declarative(db, declarative_signature_template.ty(db))?;
        Ok(Self::new(db, implicit_parameters, trai, ty))
    }
}

pub type TraitForTypeImplBlockSignatureTemplates =
    SmallVec<[TraitForTypeImplBlockEtherealSignatureTemplate; 2]>;

pub trait HasTypeSideTraitForTypeImplBlockSignatureTemplates: Copy {
    type Key: Copy;

    fn ty_side_trai_for_ty_impl_block_signature_templates<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        key: Self::Key,
    ) -> EtherealSignatureMaybeResult<&'a [TraitForTypeImplBlockEtherealSignatureTemplate]>;
}

impl HasTypeSideTraitForTypeImplBlockSignatureTemplates for TypePath {
    type Key = TraitPath;

    fn ty_side_trai_for_ty_impl_block_signature_templates<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        key: Self::Key,
    ) -> EtherealSignatureMaybeResult<&'a [TraitForTypeImplBlockEtherealSignatureTemplate]> {
        match ty_side_impl_block_signature_templates_map(db, self).get_value(key) {
            Some(result) => match result {
                Ok(_) => todo!(),
                Err(_) => todo!(),
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

pub trait HasTraitSideTraitForTypeImplBlockSignatureTemplates: Copy {
    type Key: Copy;
    fn trai_side_trai_for_ty_impl_block_signature_templates<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        key: Self::Key,
    ) -> Option<&'a [TraitForTypeImplBlockEtherealSignatureTemplate]>;
}

impl HasTraitSideTraitForTypeImplBlockSignatureTemplates for TraitPath {
    type Key = TypePath;

    fn trai_side_trai_for_ty_impl_block_signature_templates<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        key: Self::Key,
    ) -> Option<&'a [TraitForTypeImplBlockEtherealSignatureTemplate]> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitForTypeImplBlockSignature {}
