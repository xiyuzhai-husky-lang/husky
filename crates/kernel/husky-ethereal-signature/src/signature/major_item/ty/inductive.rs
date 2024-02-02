use super::*;
use husky_declarative_signature::InductiveTypeDecTemplate;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct InductiveTypeEthTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
}

impl InductiveTypeEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypePath,
        declarative_signature_template: InductiveTypeDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters = EtherealTemplateParameters::from_declarative(
            db,
            declarative_signature_template.template_parameters(db),
        )?;
        Ok(Self::new(db, path, template_parameters))
    }
}
