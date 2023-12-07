use crate::{path_leading::HirTypePathLeading, *};
use husky_vfs::Toolchain;
use smallvec::smallvec;

#[derive(Debug, PartialEq, Eq)]
pub struct HirTypeMenu {
    unit_ty: HirTypePathLeading,
}

impl HirTypeMenu {
    fn new(db: &::salsa::Db, toolchain: Toolchain) -> Self {
        let item_path_menu = item_path_menu(db, toolchain);
        Self {
            unit_ty: HirTypePathLeading::new(db, item_path_menu.unit_ty_path(), smallvec![], true),
        }
    }

    pub fn unit_ty(&self) -> HirTypePathLeading {
        self.unit_ty
    }
}

#[salsa::tracked(jar = HirTypeJar, return_ref)]
pub fn hir_ty_menu(db: &::salsa::Db, toolchain: Toolchain) -> HirTypeMenu {
    HirTypeMenu::new(db, toolchain)
}
