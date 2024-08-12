use super::*;
use husky_dec_signature::signature::assoc_item::trai_for_ty_item::assoc_ritchie::TraitForTypeAssocRitchieDecTemplate;

#[salsa::interned]
pub struct TraitForTypeAssocRitchieEthTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    #[return_ref]
    pub parenate_parameters: EtherealParenateParameters,
    pub return_ty: EthTerm,
}

impl TraitForTypeAssocRitchieEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        dec_template: TraitForTypeAssocRitchieDecTemplate,
    ) -> EthSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, dec_template.template_parameters(db))?;
        let parenate_parameters =
            EtherealParenateParameters::from_dec(db, dec_template.parenate_parameters(db))?;
        let return_ty = EthTerm::ty_from_dec(db, dec_template.return_ty(db))?;
        Ok(Self::new(
            db,
            path,
            template_parameters,
            parenate_parameters,
            return_ty,
        ))
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitForTypeAssocRitchieEthSignature {
    path: TraitForTypeItemPath,
    instantiation: EthInstantiation,
}

impl TraitForTypeAssocRitchieEthSignature {
    pub fn path(&self) -> TraitForTypeItemPath {
        self.path
    }

    pub fn instantiation(&self) -> &EthInstantiation {
        &self.instantiation
    }
}

#[salsa::interned]
pub struct TraitForTypeAssocRitchieEthSignatureBuilder {
    pub template: TraitForTypeAssocRitchieEthTemplate,
    #[return_ref]
    pub instantiation_builder: EthInstantiationBuilder,
    pub context_itd: EthTermContextItd,
}

impl TraitForTypeAssocRitchieEthTemplate {
    pub(super) fn inherit_instantiation_builder(
        self,
        impl_block_signature_builder: TraitForTypeImplBlockEthSignatureBuilderItd,
        db: &::salsa::Db,
    ) -> TraitForTypeAssocRitchieEthSignatureBuilder {
        let instantiation_builder = impl_block_signature_builder
            .instantiation_builder(db)
            .assoc_item_builder(self.path(db), self.template_parameters(db));
        TraitForTypeAssocRitchieEthSignatureBuilder::new(
            db,
            self,
            instantiation_builder,
            impl_block_signature_builder.context_itd(db),
        )
    }
}

impl TraitForTypeAssocRitchieEthSignatureBuilder {
    pub fn try_into_signature(
        self,
        db: &::salsa::Db,
    ) -> Option<TraitForTypeAssocRitchieEthSignature> {
        trai_for_ty_assoc_ritchie_eth_signature_builder_try_into_signature(db, self)
    }

    pub fn context_ref(self, db: &::salsa::Db) -> EthTermContextRef {
        EthTermContextRef::from_context_itd(self.context_itd(db), db)
    }
}

#[salsa::tracked]
fn trai_for_ty_assoc_ritchie_eth_signature_builder_try_into_signature(
    db: &::salsa::Db,
    signature_builder: TraitForTypeAssocRitchieEthSignatureBuilder,
) -> Option<TraitForTypeAssocRitchieEthSignature> {
    let instantiation = signature_builder
        .instantiation_builder(db)
        .try_into_instantiation()?;
    let template = signature_builder.template(db);
    Some(TraitForTypeAssocRitchieEthSignature {
        path: template.path(db),
        instantiation,
    })
}
