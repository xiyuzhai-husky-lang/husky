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
            zero_literal,
            one_literal,
            two_literal,
            nat_term,
            int_term,
            rat_term,
            real_term,
            nat_ty,
            int_ty,
            rat_ty,
            real_ty,
            set_ty,
            prop_ty,
            ..
        } = *zfc_ty_menu;
        let VdOprMenu { add, r#in } = *vd_opr_menu;
        let VdGlobalDispatchMenu {
            nat_add,
            int_add,
            rat_add,
            real_add,
            in_set,
        } = global_dispatch_menu;
        [
            // ## add
            ((nat_ty, add, nat_ty), nat_add),
            ((nat_ty, add, int_ty), int_add),
            ((int_ty, add, nat_ty), int_add),
            ((int_ty, add, int_ty), int_add),
            ((nat_ty, add, rat_ty), rat_add),
            ((int_ty, add, rat_ty), rat_add),
            ((rat_ty, add, nat_ty), rat_add),
            ((rat_ty, add, int_ty), rat_add),
            ((rat_ty, add, rat_ty), rat_add),
            ((nat_ty, add, real_ty), real_add),
            ((rat_ty, add, real_ty), real_add),
            ((int_ty, add, real_ty), real_add),
            ((real_ty, add, nat_ty), real_add),
            ((real_ty, add, int_ty), real_add),
            ((real_ty, add, rat_ty), real_add),
            ((real_ty, add, real_ty), real_add),
            // ## in
            ((nat_ty, r#in, set_ty), in_set),
            ((int_ty, r#in, set_ty), in_set),
            ((rat_ty, r#in, set_ty), in_set),
            ((real_ty, r#in, set_ty), in_set),
        ]
    }
}
