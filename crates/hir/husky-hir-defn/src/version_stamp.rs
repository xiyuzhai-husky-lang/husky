mod associated_item;
mod major_item;

use crate::*;
use vec_like::VecSet;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new)]
pub struct ItemHirDefnVersionStamp {
    pub toolchain: Toolchain,
    pub hir_defn: ItemHirDefn,
    #[return_ref]
    pub item_hir_defns_in_current_crate: VecSet<ItemHirDefn>,
    #[return_ref]
    pub item_hir_defn_version_stamps_in_other_local_crates: VecSet<ItemHirDefnVersionStamp>,
}

impl ItemHirDefn {
    pub fn item_version_stamp(self, db: &dyn HirDefnDb) -> ItemHirDefnVersionStamp {
        let dependencies = self.dependencies(db);
        todo!()
    }
}
