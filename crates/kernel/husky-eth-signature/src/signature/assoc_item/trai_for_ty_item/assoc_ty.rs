use super::*;
use crate::signature::impl_block::trai_for_ty_impl_block::EthTraitForTypeImplBlockSignatureBuilder;
use husky_dec_signature::signature::assoc_item::trai_for_ty_item::assoc_ty::TraitForTypeAssocTypeDecTemplate;

#[salsa::interned]
pub struct TraitForTypeAssocTypeEthTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub assoc_ty: EthTerm,
}

impl TraitForTypeAssocTypeEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        dec_template: TraitForTypeAssocTypeDecTemplate,
    ) -> EthSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, dec_template.template_parameters(db))?;
        let ty_term = EthTerm::ty_from_dec(db, dec_template.ty_term(db))?;
        Ok(Self::new(db, path, template_parameters, ty_term))
    }

    pub(super) fn inherit_instantiation_builder(
        self,
        impl_block_signature_builder: EthTraitForTypeImplBlockSignatureBuilder,
        db: &::salsa::Db,
    ) -> TraitForTypeAssocTypeEtherealSignatureBuilder {
        let instantiation_builder = impl_block_signature_builder
            .instantiation_builder(db)
            .merge_with_item_template_parameters(self.template_parameters(db));
        TraitForTypeAssocTypeEtherealSignatureBuilder::new(
            db,
            self,
            instantiation_builder,
            impl_block_signature_builder.context(db).clone(),
        )
    }
}

#[salsa::interned(constructor = pub(super) new)]
pub struct TraitForTypeAssocTypeEtherealSignatureBuilder {
    pub template: TraitForTypeAssocTypeEthTemplate,
    #[return_ref]
    pub instantiation_builder: EthInstantiationBuilder,
    #[return_ref]
    pub context: EthSignatureBuilderContext,
}

impl TraitForTypeAssocTypeEtherealSignatureBuilder {
    pub fn try_into_signature(
        self,
        db: &::salsa::Db,
    ) -> Option<TraitForTypeAssocTypeEtherealSignature> {
        trai_for_ty_assoc_ty_ethereal_signature_signature_builder_try_into_signature(db, self)
    }
}

#[salsa::tracked]
fn trai_for_ty_assoc_ty_ethereal_signature_signature_builder_try_into_signature(
    db: &::salsa::Db,
    signature_builder: TraitForTypeAssocTypeEtherealSignatureBuilder,
) -> Option<TraitForTypeAssocTypeEtherealSignature> {
    let instantiation = signature_builder
        .instantiation_builder(db)
        .try_into_instantiation()?;
    let template = signature_builder.template(db);
    Some(TraitForTypeAssocTypeEtherealSignature {
        path: template.path(db),
        ty_term: template.assoc_ty(db).instantiate(
            &instantiation,
            signature_builder.context(db),
            db,
        ),
        instantiation,
    })
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitForTypeAssocTypeEtherealSignature {
    path: TraitForTypeItemPath,
    instantiation: EthInstantiation,
    ty_term: EthTerm,
}

impl TraitForTypeAssocTypeEtherealSignature {
    pub fn path(&self) -> TraitForTypeItemPath {
        self.path
    }

    pub fn ty_term(&self) -> EthTerm {
        self.ty_term
    }

    pub fn instantiation(&self) -> &EthInstantiation {
        &self.instantiation
    }
}
