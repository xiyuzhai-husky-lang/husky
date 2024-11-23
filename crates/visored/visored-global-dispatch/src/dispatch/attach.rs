use super::*;
use crate::{default_table::VdAttachKey, menu::VdGlobalDispatchMenu, *};
use default_table::VdDefaultGlobalDispatchTable;
use menu::vd_global_dispatch_menu;
use visored_signature::signature::attach::VdAttachSignature;
use visored_term::{
    instantiation::VdInstantiation,
    menu::{vd_ty_menu, VdTypeMenu},
    ty::VdType,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdAttachGlobalDispatch {
    Normal { signature: VdAttachSignature },
}

impl VdAttachGlobalDispatch {
    pub fn standard_defaults<'db>(
        ty_menu: &'db VdTypeMenu,
        global_dispatch_menu: &'db VdGlobalDispatchMenu,
    ) -> impl IntoIterator<Item = (VdAttachKey, VdAttachGlobalDispatch)> {
        let VdTypeMenu {
            nat,
            int,
            rat,
            real,
            complex,
            ..
        } = *ty_menu;
        let VdGlobalDispatchMenu {
            nat_to_the_power_of_nat,
            int_to_the_power_of_nat,
            rat_to_the_power_of_nat,
            real_to_the_power_of_nat,
            complex_to_the_power_of_nat,
            ..
        } = *global_dispatch_menu;
        let power = |base_ty, exponent_ty| VdAttachKey::Power {
            base_ty,
            exponent_ty,
        };
        [
            (power(nat, nat), nat_to_the_power_of_nat),
            (power(int, nat), int_to_the_power_of_nat),
            (power(rat, nat), rat_to_the_power_of_nat),
            (power(real, nat), real_to_the_power_of_nat),
            (power(complex, nat), complex_to_the_power_of_nat),
        ]
    }

    pub fn expr_ty(self) -> VdType {
        match self {
            VdAttachGlobalDispatch::Normal { signature } => signature.expr_ty(),
        }
    }
}

#[test]
fn vd_attach_global_dispatch_standard_defaults_works() {
    let db = &DB::default();
    let table = VdDefaultGlobalDispatchTable::from_standard_lisp_csv_file_dir(db);
    let zfc_ty_menu = vd_ty_menu(db);
    let global_dispatch_menu = vd_global_dispatch_menu(db);
    for (key, dispatch) in
        VdAttachGlobalDispatch::standard_defaults(zfc_ty_menu, global_dispatch_menu)
    {
        match key {
            VdAttachKey::Power {
                base_ty,
                exponent_ty,
            } => {
                assert_eq!(
                    table.power_default_dispatch(base_ty, exponent_ty),
                    Some(dispatch)
                );
            }
        }
    }
}
