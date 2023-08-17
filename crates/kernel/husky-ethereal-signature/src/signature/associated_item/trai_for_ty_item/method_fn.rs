use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeMethodFnEtherealSignatureTemplate {
    pub self_ty: EtherealTerm,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
    pub self_value_parameter: EtherealTermRitchieRegularParameter,
    #[return_ref]
    pub parenate_parameters: EtherealTermParenateParameters,
    pub return_ty: EtherealTerm,
}

impl TraitForTypeMethodFnEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TraitForTypeItemPath,
        dec_sig_tmpl: TraitForTypeMethodFnDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let self_ty = EtherealTerm::ty_from_declarative(db, dec_sig_tmpl.self_ty(db))?;
        let template_parameters = EtherealTermTemplateParameters::from_declarative(
            db,
            dec_sig_tmpl.template_parameters(db),
        )?;
        let self_value_parameter = EtherealTermRitchieRegularParameter::from_declarative(
            db,
            dec_sig_tmpl.self_value_parameter(db),
        )?;
        let parenate_parameters = EtherealTermParenateParameters::from_declarative(
            db,
            dec_sig_tmpl.parenate_parameters(db),
        )?;
        let return_ty = EtherealTerm::ty_from_declarative(db, dec_sig_tmpl.return_ty(db))?;
        Ok(TraitForTypeMethodFnEtherealSignatureTemplate::new(
            db,
            self_ty,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
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
        self_value_parameter: template
            .self_value_parameter(db)
            .instantiate(db, &instantiation)
            .into(),
        parenate_parameters: template
            .parenate_parameters(db)
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
    self_value_parameter: EtherealTermRitchieParameter,
    parenate_parameters: SmallVec<[EtherealTermRitchieParameter; 4]>,
    return_ty: EtherealTerm,
}

impl TraitForTypeMethodFnEtherealSignature {
    pub fn parenate_parameters(&self) -> &[EtherealTermRitchieParameter] {
        &self.parenate_parameters
    }

    pub fn return_ty(&self) -> EtherealTerm {
        self.return_ty
    }
}
