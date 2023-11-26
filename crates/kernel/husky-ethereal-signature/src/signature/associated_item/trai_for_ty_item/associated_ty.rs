use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssociatedTypeEtherealSignatureTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
    pub associated_ty: EtherealTerm,
}

impl TraitForTypeAssociatedTypeEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        declarative_signature_template: TraitForTypeAssociatedTypeDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters = EtherealTermTemplateParameters::from_declarative(
            db,
            declarative_signature_template.template_parameters(db),
        )?;
        let ty_term =
            EtherealTerm::ty_from_declarative(db, declarative_signature_template.ty_term(db))?;
        Ok(Self::new(db, path, template_parameters, ty_term))
    }

    pub(super) fn inherit_instantiation_builder(
        self,
        db: &::salsa::Db,
        impl_block_signature_builder: TraitForTypeImplBlockEtherealSignatureBuilder,
    ) -> TraitForTypeAssociatedTypeEtherealSignatureBuilder {
        let instantiation_builder = impl_block_signature_builder
            .instantiation_builder(db)
            .merge_with_item_template_parameters(self.template_parameters(db));
        TraitForTypeAssociatedTypeEtherealSignatureBuilder::new(db, self, instantiation_builder)
    }
}

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar, constructor = pub(super) new)]
pub struct TraitForTypeAssociatedTypeEtherealSignatureBuilder {
    pub template: TraitForTypeAssociatedTypeEtherealSignatureTemplate,
    #[return_ref]
    pub instantiation_builder: EtherealInstantiationBuilder,
}

impl TraitForTypeAssociatedTypeEtherealSignatureBuilder {
    pub fn try_into_signature(
        self,
        db: &::salsa::Db,
    ) -> Option<TraitForTypeAssociatedTypeEtherealSignature> {
        trai_for_ty_associated_ty_ethereal_signature_signature_builder_try_into_signature(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn trai_for_ty_associated_ty_ethereal_signature_signature_builder_try_into_signature(
    db: &::salsa::Db,
    signature_builder: TraitForTypeAssociatedTypeEtherealSignatureBuilder,
) -> Option<TraitForTypeAssociatedTypeEtherealSignature> {
    let instantiation = signature_builder
        .instantiation_builder(db)
        .try_into_instantiation()?;
    let template = signature_builder.template(db);
    Some(TraitForTypeAssociatedTypeEtherealSignature {
        ty_term: template.associated_ty(db).instantiate(db, &instantiation),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssociatedTypeEtherealSignature {
    ty_term: EtherealTerm,
}

impl TraitForTypeAssociatedTypeEtherealSignature {
    pub fn ty_term(&self) -> EtherealTerm {
        self.ty_term
    }
}
