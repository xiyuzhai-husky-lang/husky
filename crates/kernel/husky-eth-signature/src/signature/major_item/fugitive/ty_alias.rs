use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeAliasEthTemplate {
    pub path: MajorFugitivePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
}

impl TypeAliasEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: MajorFugitivePath,
        dec_template: TypeAliasDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, dec_template.template_parameters(db))?;
        Ok(Self::new(db, path, template_parameters))
    }
}
