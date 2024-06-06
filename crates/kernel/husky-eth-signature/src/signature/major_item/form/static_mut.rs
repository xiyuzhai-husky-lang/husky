use super::*;
use husky_dec_signature::signature::major_item::form::static_mut::MajorStaticMutDecTemplate;

#[salsa::interned]
pub struct MajorStaticMutEthTemplate {
    pub path: MajorFormPath,
}

impl MajorStaticMutEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: MajorFormPath,
        dec_template: MajorStaticMutDecTemplate,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db, path))
    }
}
