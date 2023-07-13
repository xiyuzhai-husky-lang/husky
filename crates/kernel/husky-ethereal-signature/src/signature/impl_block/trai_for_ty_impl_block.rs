use super::*;
use husky_entity_tree::{
    EntityTreeBundleResult, HasItemPaths, HasTypeSideTraitForTypeImplBlockPathsMap,
};
use smallvec::SmallVec;
use vec_like::{SmallVecPairMap, VecMapGetEntry};

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar, constructor = new)]
pub struct TraitForTypeImplBlockEtherealSignatureTemplate {
    pub path: TraitForTypeImplBlockPath,
    #[return_ref]
    pub implicit_parameters: EtherealGenericParameters,
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
        path,
        path.declarative_signature_template(db)?,
    )
}

impl TraitForTypeImplBlockEtherealSignatureTemplate {
    fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TraitForTypeImplBlockPath,
        declarative_signature_template: TraitForTypeImplBlockDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let implicit_parameters = EtherealGenericParameters::from_declarative(
            db,
            declarative_signature_template.implicit_parameters(db),
        )?;
        let trai = EtherealTerm::ty_from_declarative(db, declarative_signature_template.trai(db))?;
        let ty = EtherealTerm::ty_from_declarative(db, declarative_signature_template.ty(db))?;
        Ok(Self::new(db, path, implicit_parameters, trai, ty))
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

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar, constructor = new)]
pub struct TraitForTypeImplBlockEtherealSignatureTemplatePartialInstantiated {
    template: TraitForTypeImplBlockEtherealSignatureTemplate,
    partial_instantiation: EtherealTermPartialInstantiation,
}

impl TraitForTypeImplBlockEtherealSignatureTemplate {
    #[inline(always)]
    pub fn instantiate_ty(
        self,
        db: &dyn EtherealSignatureDb,
        arguments: &[EtherealTerm],
        ty_target: EtherealTerm,
    ) -> EtherealSignatureResult<TraitForTypeImplBlockEtherealSignatureTemplatePartialInstantiated>
    {
        let mut instantiation = self.implicit_parameters(db).instantiation();
        match instantiation.try_add_rules_from_application(db, self.ty(db), arguments) {
            JustOk(_) => Ok(
                TraitForTypeImplBlockEtherealSignatureTemplatePartialInstantiated::new(
                    db,
                    self,
                    instantiation,
                ),
            ),
            JustErr(_) => todo!(),
            Nothing => todo!(),
        }
    }
}

impl TraitForTypeImplBlockEtherealSignatureTemplatePartialInstantiated {
    pub fn try_into_signature(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> Option<TraitForTypeImplBlockEtherealSignature> {
        let instantiation = self.partial_instantiation(db).try_into_instantiation()?;
        let template = self.template(db);
        Some(TraitForTypeImplBlockEtherealSignature {
            path: template.path(db),
            trai: template.trai(db).instantiate(db, &instantiation),
            ty: template.ty(db).instantiate(db, &instantiation),
        })
    }

    /// for better caching, many common traits use "Output" as an associated item
    pub fn associated_output_term(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<EtherealTerm> {
        trai_for_ty_impl_block_with_ty_instantiated_associated_output_term(db, self)
    }

    pub fn associated_item_term(
        self,
        db: &dyn EtherealSignatureDb,
        ident: Ident,
    ) -> EtherealSignatureResult<EtherealTerm> {
        trai_for_ty_impl_block_with_ty_instantiated_associated_item_term(db, self, ident)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn trai_for_ty_impl_block_with_ty_instantiated_associated_output_term(
    db: &dyn EtherealSignatureDb,
    template: TraitForTypeImplBlockEtherealSignatureTemplatePartialInstantiated,
) -> EtherealSignatureResult<EtherealTerm> {
    trai_for_ty_impl_block_with_ty_instantiated_associated_item_term(
        db,
        template,
        db.coword_menu().camel_case_output_ident(),
    )
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn trai_for_ty_impl_block_with_ty_instantiated_associated_item_term(
    db: &dyn EtherealSignatureDb,
    template: TraitForTypeImplBlockEtherealSignatureTemplatePartialInstantiated,
    ident: Ident,
) -> EtherealSignatureResult<EtherealTerm> {
    // let path = template
    //     .path(db)
    //     .item_paths(db)
    //     .get_entry(ident)
    //     .ok_or(EtherealSignatureError::NoSuchItem)?
    //     .1;
    // let ethereal_signature_template = path.ethereal_signature_template(db)?;
    todo!()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitForTypeImplBlockEtherealSignature {
    path: TraitForTypeImplBlockPath,
    trai: EtherealTerm,
    ty: EtherealTerm,
}

impl TraitForTypeImplBlockEtherealSignature {
    pub fn path(&self) -> TraitForTypeImplBlockPath {
        self.path
    }

    pub fn trai(&self) -> EtherealTerm {
        self.trai
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }
}
