use super::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[salsa::interned]
pub struct TraitAssocValEthTemplate {
    pub path: TraitItemPath,
}

/// # getters
impl TraitAssocValEthTemplate {
    #[inline(always)]
    pub fn template_parameters(self, db: &::salsa::Db) -> &[EthTemplateParameter] {
        &[]
    }
}
