pub mod menu;

use crate::term::LnTerm;
use interned::db::attached_interner_db;
use lean_entity_path::LnItemPath;
use smallvec::*;

#[interned::interned]
pub struct LnInstantiation {
    pub item_path: LnItemPath,
    #[return_ref]
    pub arguments: SmallVec<[LnTerm; 2]>,
}

impl std::fmt::Debug for LnInstantiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let db = attached_interner_db();
        write!(f, "{:?}...", self.item_path(db))
    }
}
