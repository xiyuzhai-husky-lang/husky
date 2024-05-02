use self::signature::trai_for_ty_item::assoc_ty::TraitForTypeAssocTypeEtherealSignatureBuilder;

use super::*;
use assoc_item::trai_for_ty_item::TraitForTypeItemEtherealSignatureBuilder;
use husky_dec_signature::signature::{
    impl_block::trai_for_ty_impl_block::{DeclarativeSelfType, TraitForTypeImplBlockDecTemplate},
    HasDecTemplate,
};
use husky_entity_path::path::impl_block::trai_for_ty_impl_block::TraitForTypeImplBlockPath;
use husky_entity_tree::HasAssocItemPaths;
use husky_eth_term::term::symbolic_variable::EthSymbolicVariable;
use husky_term_prelude::TypeFinalDestinationExpectation;
use vec_like::VecMapGetEntry;

#[salsa::tracked(constructor = new)]
pub struct TraitForTypeImplBlockEthTemplate {
    pub path: TraitForTypeImplBlockPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub trai: EthTerm,
    pub self_ty_refined: EtherealSelfTypeInTraitImpl,
}

impl TraitForTypeImplBlockEthTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> EthTerm {
        self.self_ty_refined(db).term()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EtherealSelfTypeInTraitImpl {
    PathLeading(EthTerm),
    DeriveAny(EthSymbolicVariable),
}

impl EtherealSelfTypeInTraitImpl {
    pub fn term(self) -> EthTerm {
        match self {
            EtherealSelfTypeInTraitImpl::PathLeading(ty_term) => ty_term,
            EtherealSelfTypeInTraitImpl::DeriveAny(ty_term_symbol) => ty_term_symbol.into(),
        }
    }
}

impl EthInstantiate for EtherealSelfTypeInTraitImpl {
    type Output = EthTerm;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EthInstantiation) -> Self::Output {
        match self {
            EtherealSelfTypeInTraitImpl::PathLeading(term) => term.instantiate(db, instantiation),
            EtherealSelfTypeInTraitImpl::DeriveAny(term_symbol) => {
                term_symbol.instantiate(db, instantiation)
            }
        }
    }
}

impl EtherealSelfTypeInTraitImpl {
    fn from_dec(db: &::salsa::Db, declarative_self_ty: DeclarativeSelfType) -> EthTermResult<Self> {
        Ok(match declarative_self_ty {
            DeclarativeSelfType::Path(declarative_term) => {
                EtherealSelfTypeInTraitImpl::PathLeading(EthTerm::ty_from_dec(
                    db,
                    declarative_term,
                )?)
            }
            DeclarativeSelfType::DerivedAny(dec_symbolic_variable) => {
                EtherealSelfTypeInTraitImpl::DeriveAny(EthSymbolicVariable::from_dec(
                    db,
                    dec_symbolic_variable,
                )?)
            }
        })
    }

    pub fn parameter_symbol(self) -> Option<EthSymbolicVariable> {
        match self {
            EtherealSelfTypeInTraitImpl::PathLeading(_) => None,
            EtherealSelfTypeInTraitImpl::DeriveAny(symbol) => Some(symbol),
        }
    }
}

impl HasEthTemplate for TraitForTypeImplBlockPath {
    type EthTemplate = TraitForTypeImplBlockEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        trai_for_ty_impl_block_eth_template(db, self)
    }
}

#[salsa::tracked]
fn trai_for_ty_impl_block_eth_template(
    db: &::salsa::Db,
    path: TraitForTypeImplBlockPath,
) -> EtherealSignatureResult<TraitForTypeImplBlockEthTemplate> {
    TraitForTypeImplBlockEthTemplate::from_dec(db, path, path.dec_template(db)?)
}

impl TraitForTypeImplBlockEthTemplate {
    fn from_dec(
        db: &::salsa::Db,
        path: TraitForTypeImplBlockPath,
        dec_template: TraitForTypeImplBlockDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, dec_template.template_parameters(db))?;
        let trai = EthTerm::from_dec(
            db,
            dec_template.trai(db),
            TypeFinalDestinationExpectation::Any,
        )?;
        let self_ty = EtherealSelfTypeInTraitImpl::from_dec(db, dec_template.self_ty(db))?;
        Ok(Self::new(db, path, template_parameters, trai, self_ty))
    }
}

pub type TraitForTypeImplBlockSignatureTemplates = SmallVec<[TraitForTypeImplBlockEthTemplate; 2]>;

#[salsa::interned(constructor = new)]
pub struct EthTraitForTypeImplBlockSignatureBuilder {
    pub template: TraitForTypeImplBlockEthTemplate,
    pub instantiation_builder: EtherealInstantiationBuilder,
}

impl TraitForTypeImplBlockEthTemplate {
    /// try to give a partial instantiation such that `self_ty` is equal to `target_ty`
    /// returns `Nothing` when template matching failed
    #[inline(always)]
    pub fn instantiate_ty(
        self,
        db: &::salsa::Db,
        target_ty_arguments: &[EthTerm],
        target_ty_term: EthTerm,
    ) -> EtherealSignatureMaybeResult<EthTraitForTypeImplBlockSignatureBuilder> {
        let mut instantiation = self
            .template_parameters(db)
            .empty_instantiation_builder(self.path(db).into(), true);
        match self.self_ty_refined(db) {
            EtherealSelfTypeInTraitImpl::PathLeading(self_ty_term) => {
                instantiation.try_add_rules_from_application(
                    self_ty_term,
                    target_ty_arguments,
                    db,
                )?;
                JustOk(EthTraitForTypeImplBlockSignatureBuilder::new(
                    db,
                    self,
                    instantiation,
                ))
            }
            EtherealSelfTypeInTraitImpl::DeriveAny(symbol) => {
                let JustOk(()) = instantiation.try_add_symbol_rule(symbol, target_ty_term) else {
                    unreachable!("this can't go wrong because instantiation was empty")
                };
                JustOk(EthTraitForTypeImplBlockSignatureBuilder::new(
                    db,
                    self,
                    instantiation,
                ))
            }
        }
    }
}

impl EthTraitForTypeImplBlockSignatureBuilder {
    pub fn try_into_signature(
        self,
        db: &::salsa::Db,
    ) -> Option<TraitForTypeImplBlockEtherealSignature> {
        let instantiation = self.instantiation_builder(db).try_into_instantiation()?;
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
        target_trai_arguments: &[EthTerm],
        db: &::salsa::Db,
    ) -> EtherealSignatureMaybeResult<Self> {
        let mut instantiation_builder = self.instantiation_builder(db);
        let template = self.template(db);
        instantiation_builder.try_add_rules_from_application(
            template.trai(db),
            target_trai_arguments,
            db,
        )?;
        JustOk(Self::new(db, template, instantiation_builder))
    }

    /// for better caching, many common traits use "Output" as an associated
    /// only use this when you are sure the trait has an associated type
    /// named "Output"
    pub fn assoc_output_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<TraitForTypeAssocTypeEtherealSignatureBuilder> {
        trai_for_ty_impl_block_with_ty_instantiated_assoc_output_ethereal_signature_builder(
            db, self,
        )
    }

    pub fn assoc_item_eth_template(
        self,
        db: &::salsa::Db,
        ident: Ident,
    ) -> EtherealSignatureResult<TraitForTypeItemEtherealSignatureBuilder> {
        trai_for_ty_impl_block_with_ty_instantiated_item_eth_template(db, self, ident)
    }
}

#[salsa::tracked]
fn trai_for_ty_impl_block_with_ty_instantiated_assoc_output_ethereal_signature_builder(
    db: &::salsa::Db,
    template: EthTraitForTypeImplBlockSignatureBuilder,
) -> EtherealSignatureResult<TraitForTypeAssocTypeEtherealSignatureBuilder> {
    match trai_for_ty_impl_block_with_ty_instantiated_item_eth_template(
        db,
        template,
        coword_menu(db).camel_case_output_ident(),
    )? {
        TraitForTypeItemEtherealSignatureBuilder::AssocType(item_template) => Ok(item_template),
        _ => unreachable!(),
    }
}

#[salsa::tracked]
fn trai_for_ty_impl_block_with_ty_instantiated_item_eth_template(
    db: &::salsa::Db,
    signature_builder: EthTraitForTypeImplBlockSignatureBuilder,
    ident: Ident,
) -> EtherealSignatureResult<TraitForTypeItemEtherealSignatureBuilder> {
    let item_path = signature_builder
        .template(db)
        .path(db)
        .assoc_item_paths(db)
        .get_entry(ident)
        .ok_or(
            EtherealSignatureError::NoSuchItemInTraitForTypeImplBlockEtherealSignatureBuilder {
                signature_builder,
                ident,
            },
        )?
        .1;
    let item_eth_template = item_path.eth_template(db)?;
    Ok(item_eth_template.inherit_instantiation_builder(db, signature_builder))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitForTypeImplBlockEtherealSignature {
    path: TraitForTypeImplBlockPath,
    trai: EthTerm,
    self_ty: EthTerm,
}

impl TraitForTypeImplBlockEtherealSignature {
    pub fn path(&self) -> TraitForTypeImplBlockPath {
        self.path
    }

    pub fn trai(&self) -> EthTerm {
        self.trai
    }

    pub fn ty(&self) -> EthTerm {
        self.self_ty
    }
}
