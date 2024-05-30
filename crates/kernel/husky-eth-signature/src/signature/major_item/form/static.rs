use super::*;
use husky_dec_signature::signature::major_item::form::r#static::MajorStaticDecTemplate;

#[salsa::interned]
pub struct MajorStaticEthTemplate {
    pub path: MajorFormPath,
}

impl MajorStaticEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: MajorFormPath,
        dec_template: MajorStaticDecTemplate,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db, path))
    }
}
