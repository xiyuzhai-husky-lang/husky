use husky_entity_path::path::impl_block::TypeSketch;
use husky_entity_path::path::{assoc_item::AssocItemPath, ItemPath, ItemPathId};

pub(crate) fn item_path_id_interface_cache_path(path: ItemPath, db: &::salsa::Db) -> Option<&str> {
    item_path_id_interface_cache_path_aux(db, *path)
        .as_ref()
        .map(|v| v as &str)
}

#[salsa::tracked(return_ref)]
pub(crate) fn item_path_id_interface_cache_path_aux(
    db: &::salsa::Db,
    path_id: ItemPathId,
) -> Option<String> {
    let path = path_id.item_path(db);
    // ad hoc
    match path {
        ItemPath::Submodule(_, _) => None,
        ItemPath::MajorItem(path) => Some(format!(
            "__{}__ITEM_PATH_ID_INTERFACE",
            path.ident(db).data(db)
        )),
        ItemPath::AssocItem(path) => match path {
            AssocItemPath::TypeItem(path) => Some(format!(
                "__{}__{}__ITEM_PATH_ID_INTERFACE",
                path.ty_path(db).ident(db).data(db),
                path.ident(db).data(db)
            )),
            AssocItemPath::TraitItem(_) => todo!(),
            AssocItemPath::TraitForTypeItem(path) => Some(format!(
                "__{}__for__{}__ITEM_PATH_ID_INTERFACE",
                path.impl_block(db).trai_path(db).ident(db).data(db),
                match path.impl_block(db).ty_sketch(db) {
                    TypeSketch::DeriveAny => todo!(),
                    TypeSketch::Path(path) => path.ident(db).data(db),
                }
            )),
        },
        ItemPath::TypeVariant(_, _) => None,
        ItemPath::ImplBlock(_) => None,
        ItemPath::Attr(_, _) => None,
        ItemPath::Script(_, _) => None,
    }
}
