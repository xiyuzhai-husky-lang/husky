use super::*;
use husky_declarative_signature::InductiveTypeDecTemplate;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct InductiveTypeEthTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
}

impl InductiveTypeEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypePath,
        dec_template: InductiveTypeDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_declarative(db, dec_template.template_parameters(db))?;
        Ok(Self::new(db, path, template_parameters))
    }
}
