use super::*;
use crate::menu::VdGlobalDispatchMenu;
use visored_term::{menu::VdTypeMenu, ty::VdType};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSqrtGlobalDispatch {
    Normal {},
}

impl VdSqrtGlobalDispatch {
    pub fn standard_defaults(
        zfc_ty_menu: &VdTypeMenu,
        global_dispatch_menu: &VdGlobalDispatchMenu,
    ) -> impl IntoIterator<Item = (VdType, VdSqrtGlobalDispatch)> {
        let VdTypeMenu {
            nat,
            int,
            rat,
            real,
            complex,
            ..
        } = *zfc_ty_menu;
        let VdGlobalDispatchMenu { real_sqrt, .. } = *global_dispatch_menu;
        [
            (nat, real_sqrt),
            (int, real_sqrt),
            (rat, real_sqrt),
            (real, real_sqrt),
        ]
    }
}
