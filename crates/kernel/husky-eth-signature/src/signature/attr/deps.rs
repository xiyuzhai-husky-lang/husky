use super::*;

#[salsa::interned]
pub struct DepsAttrEthTemplate {
    pub path: AttrItemPath,
}
