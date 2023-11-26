use crate::{
    menu::{hir_ty_menu, HirTypeMenu},
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
    HirTrait,
    crate::trai::hir_trai_from_ethereal_term_application,
    crate::menu::hir_ty_menu,
);
