use super::*;
use husky_dec_signature::signature::attr::backprop::BackpropAttrDecTemplate;

#[salsa::interned(constructor = new)]
pub struct BackpropAttrEthTemplate {
    pub path: AttrItemPath,
}

impl BackpropAttrEthTemplate {
    pub(super) fn from_dec(
        path: AttrItemPath,
        dec_template: BackpropAttrDecTemplate,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db, path))
    }
}
