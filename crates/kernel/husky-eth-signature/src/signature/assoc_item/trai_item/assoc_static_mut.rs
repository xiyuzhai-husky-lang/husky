use super::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[salsa::interned]
pub struct TraitAssocStaticMutEthTemplate {
    pub path: TraitItemPath,
}

impl TraitAssocStaticMutEthTemplate {
    #[inline(always)]
    pub fn template_parameters(self, db: &::salsa::Db) -> &[EthTemplateParameter] {
        &[]
    }
}
