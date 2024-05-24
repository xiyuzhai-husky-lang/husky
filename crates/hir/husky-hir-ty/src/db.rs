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

#[salsa::jar]
pub struct HirTypeJar(
    HirTypePathLeading,
    HirTypeTypeAssocType,
    HirTypeTraitAssocType,
    hir_template_variable_from_eth,
    HirComptermTemplateVariable,
    crate::ty::ritchie::hir_ty_from_eth_term_ritchie,
    hir_ty_from_eth_term_application,
    crate::ty::ritchie::HirRitchieType,
    HirTrait,
    crate::trai::hir_trai_from_eth_term_application,
    crate::menu::hir_ty_menu,
);
