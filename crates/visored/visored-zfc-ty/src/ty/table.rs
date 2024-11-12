use crate::{menu::vd_zfc_ty_menu, ty::VdZfcType};
use rustc_hash::FxHashMap;
use visored_item_path::path::VdItemPath;

pub struct VdItemPathZfcTypeTable {
    tys: FxHashMap<VdItemPath, VdZfcType>,
}

impl VdItemPathZfcTypeTable {
    pub fn new(tys: impl IntoIterator<Item = (VdItemPath, VdZfcType)>) -> Self {
        Self {
            tys: tys.into_iter().collect(),
        }
    }

    pub fn new_standard(db: &::salsa::Db) -> Self {
        // TODO: use menu?
        // let vd_item_path_menu = todo!();
        let zfc_ty_menu = vd_zfc_ty_menu(db);
        Self::new([
            (VdItemPath::NATURAL_NUMBER, zfc_ty_menu.set_category_ty()),
            (VdItemPath::RATIONAL_NUMBER, zfc_ty_menu.set_category_ty()),
            (VdItemPath::INTEGER, zfc_ty_menu.set_category_ty()),
            (VdItemPath::REAL_NUMBER, zfc_ty_menu.set_category_ty()),
            (VdItemPath::COMPLEX_NUMBER, zfc_ty_menu.set_category_ty()),
        ])
    }
}

impl std::ops::Index<VdItemPath> for VdItemPathZfcTypeTable {
    type Output = VdZfcType;
    fn index(&self, item_path: VdItemPath) -> &Self::Output {
        &self.tys[&item_path]
    }
}
