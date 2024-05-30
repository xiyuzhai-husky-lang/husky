use super::*;
use husky_dec_signature::signature::major_item::ty::r#enum::EnumDecTemplate;

#[salsa::interned]
pub struct EnumEthTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
}

impl EnumEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TypePath,
        dec_template: EnumDecTemplate,
    ) -> EthSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, dec_template.template_parameters(db))?;
        Ok(Self::new(db, path, template_parameters))
    }
}
