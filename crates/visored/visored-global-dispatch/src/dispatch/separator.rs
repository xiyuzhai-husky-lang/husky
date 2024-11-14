use crate::menu::VdGlobalDispatchMenu;

use super::*;
use visored_opr::{menu::VdOprMenu, separator::VdBaseSeparator};
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;
use visored_zfc_ty::{menu::VdZfcTypeMenu, ty::VdZfcType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VdSeparatorGlobalDispatch {
    Normal {
        base_separator: VdBaseSeparator,
        signature: VdBaseSeparatorSignature,
    },
    InSet {
        expr_ty: VdZfcType,
    },
}

impl VdSeparatorGlobalDispatch {
    pub(crate) fn standard_defaults<'db>(
        zfc_ty_menu: &'db VdZfcTypeMenu,
        vd_opr_menu: &'db VdOprMenu,
        global_dispatch_menu: &'db VdGlobalDispatchMenu,
    ) -> impl IntoIterator<
        Item = (
            (VdZfcType, VdBaseSeparator, VdZfcType),
            &'db VdSeparatorGlobalDispatch,
        ),
    > {
        let VdZfcTypeMenu {
            nat,
            int,
            rat,
            real,
            complex,
            set,
            prop,
        } = *zfc_ty_menu;
        let VdOprMenu { add, eq, r#in } = *vd_opr_menu;
        let VdGlobalDispatchMenu {
            nat_add,
            int_add,
            rat_add,
            real_add,
            nat_eq,
            int_eq,
            rat_eq,
            real_eq,
            in_set,
        } = global_dispatch_menu;
        [
            // ## add
            ((nat, add, nat), nat_add),
            ((nat, add, int), int_add),
            ((int, add, nat), int_add),
            ((int, add, int), int_add),
            ((nat, add, rat), rat_add),
            ((int, add, rat), rat_add),
            ((rat, add, nat), rat_add),
            ((rat, add, int), rat_add),
            ((rat, add, rat), rat_add),
            ((nat, add, real), real_add),
            ((rat, add, real), real_add),
            ((int, add, real), real_add),
            ((real, add, nat), real_add),
            ((real, add, int), real_add),
            ((real, add, rat), real_add),
            ((real, add, real), real_add),
            // ## eq
            ((nat, eq, nat), nat_eq),
            ((nat, eq, int), int_eq),
            ((int, eq, nat), int_eq),
            ((int, eq, int), int_eq),
            ((nat, eq, rat), rat_eq),
            ((int, eq, rat), rat_eq),
            ((rat, eq, nat), rat_eq),
            ((rat, eq, int), rat_eq),
            ((rat, eq, rat), rat_eq),
            ((nat, eq, real), real_eq),
            ((rat, eq, real), real_eq),
            ((int, eq, real), real_eq),
            ((real, eq, nat), real_eq),
            ((real, eq, int), real_eq),
            ((real, eq, rat), real_eq),
            ((real, eq, real), real_eq),
            // ## in
            ((nat, r#in, set), in_set),
            ((int, r#in, set), in_set),
            ((rat, r#in, set), in_set),
            ((real, r#in, set), in_set),
        ]
    }
}
