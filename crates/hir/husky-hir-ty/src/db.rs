use crate::{
    menu::{hir_ty_menu, HirTypeMenu},
    path_leading::HirTypePathLeading,
    trai::HirTrait,
    *,
};
use husky_vfs::Toolchain;

pub trait HirTypeDb {
    fn hir_ty_menu(&self, toolchain: Toolchain) -> &HirTypeMenu;
}

impl HirTypeDb for ::salsa::Db {
    fn hir_ty_menu(&self, toolchain: Toolchain) -> &HirTypeMenu {
        hir_ty_menu(self, toolchain)
    }
}

#[salsa::jar(db = HirTypeDb)]
pub struct HirTypeJar(
    HirTypePathLeading,
    HirTypeTypeAssociatedType,
    HirTypeTraitAssociatedType,
    hir_template_symbol_from_ethereal,
    HirConstSymbol,
    crate::ty::ritchie::hir_ty_from_ethereal_term_ritchie,
    hir_ty_from_ethereal_term_application,
    crate::ty::ritchie::HirRitchieType,
    crate::ty::path_leading::hir_ty_path_leading_is_copyable_obviously,
    HirTrait,
    crate::trai::hir_trai_from_ethereal_term_application,
    crate::menu::hir_ty_menu,
);
