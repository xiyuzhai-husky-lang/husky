use super::*;
use husky_dec_signature::signature::assoc_item::trai_item::assoc_static_var::TraitAssocStaticVarDecTemplate;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[salsa::interned]
pub struct TraitAssocStaticVarEthTemplate {
    pub path: TraitItemPath,
}

impl TraitAssocStaticVarEthTemplate {
    pub(super) fn from_dec(
        path: TraitItemPath,
        template: TraitAssocStaticVarDecTemplate,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db, path))
    }
}
