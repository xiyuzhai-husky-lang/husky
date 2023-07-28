use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeMethodFnEtherealSignatureTemplate {
    pub self_ty: EtherealTerm,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    #[return_ref]
    pub self_parameter: EtherealTermRitchieRegularParameter,
    #[return_ref]
    pub parenic_parameters: ParenicEtherealParameters,
    pub return_ty: EtherealTerm,
}

impl TypeMethodFnEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature: TypeMethodFnDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let self_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.self_ty(db))?;
        let template_parameters = EtherealTemplateParameters::from_declarative(
            db,
            declarative_signature.template_parameters(db),
        )?;
        let self_parameter = EtherealTermRitchieRegularParameter::from_declarative(
            db,
            declarative_signature.self_parameter(db),
        )?;
        let parenic_parameters = ParenicEtherealParameters::from_declarative(
            db,
            declarative_signature.parenic_parameters(db),
        )?;
        let return_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.return_ty(db))?;
        Ok(Self::new(
            db,
            self_ty,
            template_parameters,
            self_parameter,
            parenic_parameters,
            return_ty,
        ))
    }
}
