use super::*;
use husky_dec_signature::signature::major_item::form::static_var::MajorStaticVarDecTemplate;

#[salsa::interned]
pub struct MajorStaticVarEthTemplate {
    pub path: MajorFormPath,
}

impl MajorStaticVarEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: MajorFormPath,
        dec_template: MajorStaticVarDecTemplate,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db, path))
    }
}
