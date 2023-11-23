use super::*;
use husky_entity_syn_tree::HasAssociatedItemPaths;
use husky_term_prelude::TermTypeExpectation;
use smallvec::SmallVec;
use vec_like::VecMapGetEntry;

#[salsa::tracked(db = EtherealSignatureDb, jar = EtherealSignatureJar, constructor = new)]
pub struct TraitForTypeImplBlockEtherealSignatureTemplate {
    pub path: TraitForTypeImplBlockPath,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
    pub trai: EtherealTerm,
    pub self_ty_refined: EtherealSelfTypeInTraitImpl,
}

impl TraitForTypeImplBlockEtherealSignatureTemplate {
    pub fn self_ty(self, db: &dyn EtherealSignatureDb) -> EtherealTerm {
        self.self_ty_refined(db).term()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EtherealSelfTypeInTraitImpl {
    PathLeading(EtherealTerm),
    DeriveAny(EtherealTermSymbol),
}

impl EtherealSelfTypeInTraitImpl {
    pub fn term(self) -> EtherealTerm {
        match self {
            EtherealSelfTypeInTraitImpl::PathLeading(ty_term) => ty_term,
            EtherealSelfTypeInTraitImpl::DeriveAny(ty_term_symbol) => ty_term_symbol.into(),
        }
    }
}

impl EtherealTermInstantiate for EtherealSelfTypeInTraitImpl {
    type Target = EtherealTerm;

    fn instantiate(
        self,
        db: &dyn EtherealTermDb,
        instantiation: &EtherealInstantiation,
    ) -> Self::Target {
        match self {
            EtherealSelfTypeInTraitImpl::PathLeading(term) => term.instantiate(db, instantiation),
            EtherealSelfTypeInTraitImpl::DeriveAny(term_symbol) => {
                term_symbol.instantiate(db, instantiation)
            }
        }
    }
}

impl EtherealSelfTypeInTraitImpl {
    fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_self_ty: DeclarativeSelfType,
    ) -> EtherealTermResult<Self> {
        Ok(match declarative_self_ty {
            DeclarativeSelfType::Path(declarative_term) => {
                EtherealSelfTypeInTraitImpl::PathLeading(EtherealTerm::ty_from_declarative(
                    db,
                    declarative_term,
                )?)
            }
            DeclarativeSelfType::DerivedAny(declarative_term_symbol) => {
                EtherealSelfTypeInTraitImpl::DeriveAny(EtherealTermSymbol::from_declarative(
                    db,
                    declarative_term_symbol,
                )?)
            }
        })
    }

    pub fn parameter_symbol(self) -> Option<EtherealTermSymbol> {
        match self {
            EtherealSelfTypeInTraitImpl::PathLeading(_) => None,
            EtherealSelfTypeInTraitImpl::DeriveAny(symbol) => Some(symbol),
        }
    }
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
        let template_parameters = EtherealTermTemplateParameters::from_declarative(
            db,
            declarative_signature_template.template_parameters(db),
        )?;
        let trai = EtherealTerm::from_declarative(
            db,
            declarative_signature_template.trai(db),
            TermTypeExpectation::Any,
        )?;
        let self_ty = EtherealSelfTypeInTraitImpl::from_declarative(
            db,
            declarative_signature_template.self_ty(db),
        )?;
        Ok(Self::new(db, path, template_parameters, trai, self_ty))
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
    /// returns `Nothing` when template matching failed
    #[inline(always)]
    pub fn instantiate_ty(
        self,
        db: &dyn EtherealSignatureDb,
        target_ty_arguments: &[EtherealTerm],
        target_ty_term: EtherealTerm,
    ) -> EtherealSignatureMaybeResult<
        TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated,
    > {
        let mut instantiation = self.template_parameters(db).empty_partial_instantiation();
        match self.self_ty_refined(db) {
            EtherealSelfTypeInTraitImpl::PathLeading(self_ty_term) => {
                match instantiation.try_add_rules_from_application(
                    self_ty_term,
                    target_ty_arguments,
                    db,
                ) {
                    JustOk(_) => JustOk(
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
            EtherealSelfTypeInTraitImpl::DeriveAny(symbol) => {
                instantiation.try_add_symbol_rule(symbol, target_ty_term)?;
                JustOk(
                    TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated::new(
                        db,
                        self,
                        instantiation,
                    ),
                )
            }
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
            self_ty: template.self_ty(db).instantiate(db, &instantiation),
        })
    }

    /// normally further instantiation comes from methods or associated fns/gns/functions
    /// but this serves as a useful shortcut for traits like `Unveil`
    /// return `Nothing` when template matching failed
    pub fn instantiate_trai(
        self,
        target_trai_arguments: &[EtherealTerm],
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureMaybeResult<Self> {
        let mut partial_instantiation = self.partial_instantiation(db);
        let template = self.template(db);
        partial_instantiation.try_add_rules_from_application(
            template.trai(db),
            target_trai_arguments,
            db,
        )?;
        JustOk(Self::new(db, template, partial_instantiation))
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

#[salsa::tracked(jar = EtherealSignatureJar,)]
fn trai_for_ty_impl_block_with_ty_instantiated_item_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    template_partially_instantiated: TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated,
    ident: Ident,
) -> EtherealSignatureResult<TraitForTypeItemEtherealSignatureTemplatePartiallyInstantiated> {
    let item_path = template_partially_instantiated
        .template(db)
        .path(db)
        .associated_item_paths(db)
        .get_entry(ident)
        .ok_or(EtherealSignatureError::NoSuchItemInTraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated {template_partially_instantiated, ident})?
        .1;
    let item_ethereal_signature_template = item_path.ethereal_signature_template(db)?;
    Ok(item_ethereal_signature_template
        .inherit_partial_instantiation(db, template_partially_instantiated))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitForTypeImplBlockEtherealSignature {
    path: TraitForTypeImplBlockPath,
    trai: EtherealTerm,
    self_ty: EtherealTerm,
}

impl TraitForTypeImplBlockEtherealSignature {
    pub fn path(&self) -> TraitForTypeImplBlockPath {
        self.path
    }

    pub fn trai(&self) -> EtherealTerm {
        self.trai
    }

    pub fn ty(&self) -> EtherealTerm {
        self.self_ty
    }
}
