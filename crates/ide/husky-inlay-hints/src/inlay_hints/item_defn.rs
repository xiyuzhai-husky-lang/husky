use super::*;
use husky_entity_path::path::ItemPath;

#[salsa::tracked(return_ref)]
pub(crate) fn item_defn_inlay_hints(db: &::salsa::Db, item_path_id: ItemPathId) -> Vec<InlayHint> {
    let item_path = item_path_id.item_path(db);
    match item_path {
        ItemPath::Submodule(_, _) => vec![],
        ItemPath::MajorItem(_) => vec![],
        ItemPath::AssocItem(_) => vec![],
        ItemPath::TypeVariant(_, _) => vec![],
        ItemPath::ImplBlock(_) => vec![],
        ItemPath::Attr(_, _) => vec![],
        ItemPath::Script(_, _) => vec![],
    }
}
