use crate::{default_table::VdAttachKey, menu::VdGlobalDispatchMenu};
use visored_signature::signature::attach::VdAttachSignature;
use visored_zfc_ty::{instantiation::VdInstantiation, menu::VdZfcTypeMenu};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdAttachGlobalDispatch {
    Normal { signature: VdAttachSignature },
}

impl VdAttachGlobalDispatch {
    pub fn standard_defaults<'db>(
        zfc_ty_menu: &'db VdZfcTypeMenu,
        global_dispatch_menu: &'db VdGlobalDispatchMenu,
    ) -> impl IntoIterator<Item = (VdAttachKey, VdAttachGlobalDispatch)> {
        let VdZfcTypeMenu {
            nat,
            int,
            rat,
            real,
            complex,
            ..
        } = *zfc_ty_menu;
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
}
