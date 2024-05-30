use super::*;
use husky_dec_signature::signature::major_item::form::compterm::MajorComptermDecTemplate;

#[salsa::interned]
pub struct MajorComptermEthTemplate {
    pub path: MajorFormPath,
}

impl MajorComptermEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: MajorFormPath,
        dec_template: MajorComptermDecTemplate,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db, path))
    }
}
