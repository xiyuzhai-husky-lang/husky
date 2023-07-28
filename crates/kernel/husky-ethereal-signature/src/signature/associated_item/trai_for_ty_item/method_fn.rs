use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeMethodFnEtherealSignatureTemplate {
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    pub self_parameter: EtherealTermRitchieRegularParameter,
    #[return_ref]
    pub parenic_parameters: ParenicEtherealParameters,
    pub return_ty: EtherealTerm,
}

impl TraitForTypeMethodFnEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TraitForTypeItemPath,
        declarative_signature_template: TraitForTypeMethodFnDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters = EtherealTemplateParameters::from_declarative(
            db,
            declarative_signature_template.template_parameters(db),
        )?;
        let self_parameter = EtherealTermRitchieRegularParameter::from_declarative(
            db,
            declarative_signature_template.self_parameter(db),
        )?;
        let parenic_parameters = ParenicEtherealParameters::from_declarative(
            db,
            declarative_signature_template.parenic_parameters(db),
        )?;
        let return_ty =
            EtherealTerm::ty_from_declarative(db, declarative_signature_template.return_ty(db))?;
        Ok(TraitForTypeMethodFnEtherealSignatureTemplate::new(
            db,
            template_parameters,
            self_parameter,
            parenic_parameters,
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
            .merge_with_item_template_parameters(self.template_parameters(db));
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
        self_parameter: template
            .self_parameter(db)
            .instantiate(db, &instantiation)
            .into(),
        parenic_parameters: template
            .parenic_parameters(db)
            .iter()
            .map(|param| -> EtherealTermRitchieParameter {
                param.instantiate(db, &instantiation).into()
            })
            .collect(),
        return_ty: template.return_ty(db).instantiate(db, &instantiation),
    })
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitForTypeMethodFnEtherealSignature {
    self_parameter: EtherealTermRitchieParameter,
    parenic_parameters: SmallVec<[EtherealTermRitchieParameter; 4]>,
    return_ty: EtherealTerm,
}

impl TraitForTypeMethodFnEtherealSignature {
    pub fn parenic_parameters(&self) -> &[EtherealTermRitchieParameter] {
        &self.parenic_parameters
    }

    pub fn return_ty(&self) -> EtherealTerm {
        self.return_ty
    }
}
