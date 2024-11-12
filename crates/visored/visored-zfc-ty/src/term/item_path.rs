use super::*;

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdZfcItemPath(VdZfcTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdZfcItemPathData {
    item_path: VdItemPath,
}

impl VdZfcItemPathData {
    pub fn item_path(&self) -> VdItemPath {
        self.item_path
    }
}

impl VdZfcTerm {
    pub fn new_item_path(item_path: VdItemPath, db: &::salsa::Db) -> Self {
        VdZfcItemPath(VdZfcTermId::new(db, VdZfcItemPathData { item_path }.into())).into()
    }
}
