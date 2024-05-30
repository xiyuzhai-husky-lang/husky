use super::*;
use husky_dec_signature::signature::attr::task::TaskAttrDecTemplate;

#[salsa::interned]
pub struct TaskAttrEthTemplate {}

impl TaskAttrEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        dec_template: TaskAttrDecTemplate,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db))
    }
}
