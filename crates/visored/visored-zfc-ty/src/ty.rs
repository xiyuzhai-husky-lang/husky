pub mod table;

use smallvec::{smallvec, SmallVec};
use visored_coword::namae::VdNamae;
use visored_item_path::path::VdItemPath;

#[salsa::interned]
pub struct VdZfcType {
    pub data: VdZfcTypeData,
    pub refinements: SmallVec<[(); 2]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdZfcTypeData {
    ItemPath(VdItemPath), // TODO: do we need a path here?
}

impl VdZfcType {
    pub fn new_item_path(item_path: VdItemPath, db: &::salsa::Db) -> Self {
        VdZfcType::new(db, VdZfcTypeData::ItemPath(item_path).into(), smallvec![]).into()
    }
}
