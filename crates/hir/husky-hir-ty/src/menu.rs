use crate::*;
use husky_vfs::Toolchain;
use smallvec::smallvec;

#[derive(Debug, PartialEq, Eq)]
pub struct HirTypeMenu {
    unit_ty: HirTypePathLeading,
}

impl HirTypeMenu {
    fn new(db: &dyn HirTypeDb, toolchain: Toolchain) -> Self {
        let item_path_menu = db.item_path_menu(toolchain);
        Self {
            unit_ty: HirTypePathLeading::new(db, item_path_menu.unit_ty_path(), smallvec![]),
        }
    }

    pub fn unit_ty(&self) -> HirTypePathLeading {
        self.unit_ty
    }
}

#[salsa::tracked(jar = HirTypeJar, return_ref)]
pub fn hir_ty_menu(db: &dyn HirTypeDb, toolchain: Toolchain) -> HirTypeMenu {
    HirTypeMenu::new(db, toolchain)
}
