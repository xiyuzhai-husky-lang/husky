use super::*;
use husky_syn_decl::decl::attr::backprop::BackpropAttrSynDecl;

#[salsa::interned(constructor = new)]
pub struct BackpropAttrDecTemplate {
    pub path: AttrItemPath,
}

impl BackpropAttrDecTemplate {
    pub(super) fn from_decl(
        path: AttrItemPath,
        decl: BackpropAttrSynDecl,
        db: &::salsa::Db,
    ) -> DecSignatureResult<Self> {
        Ok(Self::new(db, path))
    }
}
