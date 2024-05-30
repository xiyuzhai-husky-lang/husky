use super::*;
use husky_dec_signature::signature::attr::task::TaskAttrDecTemplate;

#[salsa::interned]
pub struct TaskAttrEthTemplate {
    pub path: AttrItemPath,
}

impl TaskAttrEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: AttrItemPath,
        dec_template: TaskAttrDecTemplate,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db, path))
    }
}
