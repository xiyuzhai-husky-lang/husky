use husky_entity_path::path::{ItemPath, ItemPathId};

#[derive(Debug, PartialEq, Eq)]
pub struct CodeLens {}

pub trait HasCodeLens: Copy {
    fn code_lens(self, db: &::salsa::Db) -> &[CodeLens];
}

impl HasCodeLens for ItemPath {
    fn code_lens(self, db: &salsa::Db) -> &[CodeLens] {
        item_code_lens(db, *self)
    }
}

#[salsa::tracked(return_ref)]
fn item_code_lens(db: &::salsa::Db, item_path_id: ItemPathId) -> Vec<CodeLens> {
    let item_path = item_path_id.item_path(db);
    match item_path {
        ItemPath::Submodule(_, _) => vec![],
        ItemPath::MajorItem(_) => vec![],
        ItemPath::AssocItem(_) => vec![],
        ItemPath::TypeVariant(_, _) => vec![],
        ItemPath::ImplBlock(_) => vec![],
        ItemPath::Attr(_, _) => vec![],
        ItemPath::Chunk(_, _) => vec![],
    }
}
