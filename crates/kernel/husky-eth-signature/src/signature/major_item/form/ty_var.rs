use super::*;
use husky_dec_signature::signature::major_item::form::ty_var::TypeVarDecTemplate;

#[salsa::interned]
pub struct MajorTypeVarEthTemplate {
    pub path: MajorFormPath,
}

impl MajorTypeVarEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: MajorFormPath,
        dec_template: TypeVarDecTemplate,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db, path))
    }

    pub fn template_parameters(self, db: &::salsa::Db) -> &[EthTemplateParameter] {
        &[]
    }
}
