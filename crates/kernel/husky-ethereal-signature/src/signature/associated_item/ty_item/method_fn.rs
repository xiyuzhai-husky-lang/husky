use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeMethodFnEtherealSignatureTemplate {
    pub path: TypeItemPath,
    pub self_ty: EtherealTerm,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
    pub self_value_parameter: EtherealRitchieRegularParameter,
    #[return_ref]
    pub parenate_parameters: EtherealTermParenateParameters,
    pub return_ty: EtherealTerm,
}

impl TypeMethodFnEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TypeItemPath,
        tmpl: TypeMethodFnDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let self_ty = EtherealTerm::ty_from_declarative(db, tmpl.self_ty(db))?;
        let template_parameters =
            EtherealTermTemplateParameters::from_declarative(db, tmpl.template_parameters(db))?;
        let self_value_parameter =
            EtherealRitchieRegularParameter::from_declarative(db, tmpl.self_value_parameter(db))?;
        let parenate_parameters =
            EtherealTermParenateParameters::from_declarative(db, tmpl.parenate_parameters(db))?;
        let return_ty = EtherealTerm::ty_from_declarative(db, tmpl.return_ty(db))?;
        Ok(Self::new(
            db,
            path,
            self_ty,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
        ))
    }
}
