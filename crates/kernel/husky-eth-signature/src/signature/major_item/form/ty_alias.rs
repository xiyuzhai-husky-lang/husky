use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EthSignatureJar)]
pub struct MajorTypeAliasEthTemplate {
    pub path: MajorFormPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
}

impl MajorTypeAliasEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: MajorFormPath,
        dec_template: TypeAliasDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, dec_template.template_parameters(db))?;
        Ok(Self::new(db, path, template_parameters))
    }
}
