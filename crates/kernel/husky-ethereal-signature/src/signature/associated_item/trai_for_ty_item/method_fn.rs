use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar, constructor = new)]
pub struct TraitForTypeMethodFnEtherealSignatureTemplate {
    pub path: TraitForTypeItemPath,
    pub self_ty: EtherealTerm,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
    pub self_value_parameter: EtherealRitchieRegularParameter,
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
        let self_value_parameter = EtherealRitchieRegularParameter::from_declarative(
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
            path,
            self_ty,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
        ))
    }

    pub(super) fn inherit_instantiation_builder(
        self,
        db: &dyn EtherealSignatureDb,
        impl_block_signature_builder: TraitForTypeImplBlockEtherealSignatureBuilder,
    ) -> TraitForTypeMethodFnEtherealSignatureBuilder {
        let instantiation_builder = impl_block_signature_builder
            .instantiation_builder(db)
            .merge_with_item_template_parameters(self.template_parameters(db));
        TraitForTypeMethodFnEtherealSignatureBuilder::new(db, self, instantiation_builder)
    }
}

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeMethodFnEtherealSignatureBuilder {
    pub template: TraitForTypeMethodFnEtherealSignatureTemplate,
    #[return_ref]
    pub instantiation_builder: EtherealInstantiationBuilder,
}

impl TraitForTypeMethodFnEtherealSignatureBuilder {
    pub fn try_finish(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> Option<&TraitForTypeMethodFnEtherealSignature> {
        trai_for_ty_method_fn_ethereal_signature_signature_builder_try_into_signature(db, self)
            .as_ref()
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn trai_for_ty_method_fn_ethereal_signature_signature_builder_try_into_signature(
    db: &dyn EtherealSignatureDb,
    signature_builder: TraitForTypeMethodFnEtherealSignatureBuilder,
) -> Option<TraitForTypeMethodFnEtherealSignature> {
    // todo: deal with dependent type
    let instantiation = signature_builder
        .instantiation_builder(db)
        .try_into_instantiation()?;
    let template = signature_builder.template(db);
    Some(TraitForTypeMethodFnEtherealSignature {
        path: template.path(db),
        self_value_parameter: template
            .self_value_parameter(db)
            .instantiate(db, &instantiation)
            .into(),
        parenate_parameters: template
            .parenate_parameters(db)
            .iter()
            .map(|param| -> EtherealRitchieParameter {
                param.instantiate(db, &instantiation).into()
            })
            .collect(),
        return_ty: template.return_ty(db).instantiate(db, &instantiation),
        instantiation,
    })
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitForTypeMethodFnEtherealSignature {
    path: TraitForTypeItemPath,
    instantiation: EtherealInstantiation,
    self_value_parameter: EtherealRitchieParameter,
    parenate_parameters: SmallVec<[EtherealRitchieParameter; 4]>,
    return_ty: EtherealTerm,
}

impl TraitForTypeMethodFnEtherealSignature {
    pub fn parenate_parameters(&self) -> &[EtherealRitchieParameter] {
        &self.parenate_parameters
    }

    pub fn return_ty(&self) -> EtherealTerm {
        self.return_ty
    }

    pub fn path(&self) -> TraitForTypeItemPath {
        self.path
    }

    pub fn instantiation(&self) -> &EtherealInstantiation {
        &self.instantiation
    }
}
