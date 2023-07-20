use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeMethodFnEtherealSignatureTemplate {
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
    pub self_parameter: RegularSpecificParameter,
    pub return_ty: EtherealTerm,
}

impl TraitForTypeMethodFnEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TraitForTypeItemPath,
        declarative_signature_template: TraitForTypeMethodFnDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let generic_parameters = EtherealGenericParameters::from_declarative(
            db,
            declarative_signature_template.generic_parameters(db),
        )?;
        let self_parameter = RegularSpecificParameter::from_declarative(
            db,
            declarative_signature_template.self_parameter(db),
        )?;
        let return_ty =
            EtherealTerm::ty_from_declarative(db, declarative_signature_template.return_ty(db))?;
        Ok(TraitForTypeMethodFnEtherealSignatureTemplate::new(
            db,
            generic_parameters,
            self_parameter,
            return_ty,
        ))
    }

    pub(super) fn inherit_partial_instantiation(
        self,
        db: &dyn EtherealSignatureDb,
        impl_block_template_partially_instantiated: TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated,
    ) -> TraitForTypeMethodFnEtherealSignatureTemplatePartiallyInstantiated {
        let partial_instantiation = impl_block_template_partially_instantiated
            .partial_instantiation(db)
            .merge_with_item_generic_parameters(self.generic_parameters(db));
        TraitForTypeMethodFnEtherealSignatureTemplatePartiallyInstantiated::new(
            db,
            self,
            partial_instantiation,
        )
    }
}

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeMethodFnEtherealSignatureTemplatePartiallyInstantiated {
    pub template: TraitForTypeMethodFnEtherealSignatureTemplate,
    #[return_ref]
    pub partial_instantiation: EtherealTermPartialInstantiation,
}

impl TraitForTypeMethodFnEtherealSignatureTemplatePartiallyInstantiated {
    pub fn try_into_signature(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> Option<&TraitForTypeMethodFnEtherealSignature> {
        trai_for_ty_method_fn_ethereal_signature_template_partially_instantiated_try_into_signature(
            db, self,
        )
        .as_ref()
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn trai_for_ty_method_fn_ethereal_signature_template_partially_instantiated_try_into_signature(
    db: &dyn EtherealSignatureDb,
    template_partially_instantiated: TraitForTypeMethodFnEtherealSignatureTemplatePartiallyInstantiated,
) -> Option<TraitForTypeMethodFnEtherealSignature> {
    // todo: deal with dependent type
    let instantiation = template_partially_instantiated
        .partial_instantiation(db)
        .try_into_instantiation()?;
    let template = template_partially_instantiated.template(db);
    Some(TraitForTypeMethodFnEtherealSignature {
        self_parameter: template.self_parameter(db).instantiate(db, &instantiation),
        call_parameters: todo!(),
        return_ty: template.return_ty(db).instantiate(db, &instantiation),
    })
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitForTypeMethodFnEtherealSignature {
    self_parameter: EtherealTermRitchieParameter,
    call_parameters: SmallVec<[EtherealTermRitchieParameter; 4]>,
    return_ty: EtherealTerm,
}

impl TraitForTypeMethodFnEtherealSignature {
    pub fn call_parameters(&self) -> &[EtherealTermRitchieParameter] {
        &self.call_parameters
    }

    pub fn return_ty(&self) -> EtherealTerm {
        self.return_ty
    }
}
