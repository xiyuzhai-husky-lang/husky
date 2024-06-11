use super::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[salsa::interned]
pub struct TraitAssocValEthTemplate {
    pub path: TraitItemPath,
}
