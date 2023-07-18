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
    pub generic_parameters: EtherealGenericParameters,
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
        let generic_parameters = EtherealGenericParameters::from_declarative(
            db,
            declarative_signature_template.generic_parameters(db),
        )?;
        let trai = EtherealTerm::ty_from_declarative(db, declarative_signature_template.trai(db))?;
        let ty = EtherealTerm::ty_from_declarative(db, declarative_signature_template.ty(db))?;
        Ok(Self::new(db, path, generic_parameters, trai, ty))
    }
}

pub type TraitForTypeImplBlockSignatureTemplates =
    SmallVec<[TraitForTypeImplBlockEtherealSignatureTemplate; 2]>;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar, constructor = new)]
pub struct TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated {
    pub template: TraitForTypeImplBlockEtherealSignatureTemplate,
    pub partial_instantiation: EtherealTermPartialInstantiation,
}

impl TraitForTypeImplBlockEtherealSignatureTemplate {
    #[inline(always)]
    pub fn instantiate_ty(
        self,
        db: &dyn EtherealSignatureDb,
        arguments: &[EtherealTerm],
        ty_target: EtherealTerm,
    ) -> EtherealSignatureResult<TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated>
    {
        let mut instantiation = self.generic_parameters(db).instantiation();
        match instantiation.try_add_rules_from_application(db, self.ty(db), arguments) {
            JustOk(_) => Ok(
                TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated::new(
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

impl TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated {
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

    /// for better caching, many common traits use "Output" as an associated
    /// only use this when you are sure the trait has an associated type
    /// named "Output"
    pub fn associated_output_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated,
    > {
        trai_for_ty_impl_block_with_ty_instantiated_associated_output_ethereal_signature_template(
            db, self,
        )
    }

    pub fn associated_item_ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
        ident: Ident,
    ) -> EtherealSignatureResult<TraitForTypeItemEtherealSignatureTemplatePartiallyInstantiated>
    {
        trai_for_ty_impl_block_with_ty_instantiated_item_ethereal_signature_template(
            db, self, ident,
        )
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn trai_for_ty_impl_block_with_ty_instantiated_associated_output_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    template: TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated,
) -> EtherealSignatureResult<TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated>
{
    match trai_for_ty_impl_block_with_ty_instantiated_item_ethereal_signature_template(
        db,
        template,
        db.coword_menu().camel_case_output_ident(),
    )? {
        TraitForTypeItemEtherealSignatureTemplatePartiallyInstantiated::AssociatedType(
            item_template,
        ) => Ok(item_template),
        _ => unreachable!(),
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn trai_for_ty_impl_block_with_ty_instantiated_item_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    template_partially_instantiated: TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated,
    ident: Ident,
) -> EtherealSignatureResult<TraitForTypeItemEtherealSignatureTemplatePartiallyInstantiated> {
    let item_path = template_partially_instantiated
        .template(db)
        .path(db)
        .item_paths(db)
        .get_entry(ident)
        .ok_or(EtherealSignatureError::NoSuchItem)?
        .1;
    let item_ethereal_signature_template = item_path.ethereal_signature_template(db)?;
    Ok(item_ethereal_signature_template
        .inherit_partial_instantiation(db, template_partially_instantiated))
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
