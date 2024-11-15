use super::*;
use visored_zfc_ty::instantiation::menu::{vd_zfc_instantiation_menu, VdZfcInstantiationMenu};

#[derive(Debug, PartialEq, Eq)]
pub struct VdMirFuncKeyMenu {
    /// ## add  
    pub nat_add: VdMirFuncKey,
    pub int_add: VdMirFuncKey,
    pub rat_add: VdMirFuncKey,
    pub real_add: VdMirFuncKey,
    pub complex_add: VdMirFuncKey,
    /// ## mul
    pub nat_mul: VdMirFuncKey,
    pub int_mul: VdMirFuncKey,
    pub rat_mul: VdMirFuncKey,
    pub real_mul: VdMirFuncKey,
    pub complex_mul: VdMirFuncKey,
    /// ## pow
    pub nat_to_the_power_of_nat: VdMirFuncKey,
    pub int_to_the_power_of_nat: VdMirFuncKey,
    pub rat_to_the_power_of_nat: VdMirFuncKey,
    pub real_to_the_power_of_nat: VdMirFuncKey,
    pub complex_to_the_power_of_nat: VdMirFuncKey,
    /// ## eq
    pub nat_eq: VdMirFuncKey,
    pub int_eq: VdMirFuncKey,
    pub rat_eq: VdMirFuncKey,
    pub real_eq: VdMirFuncKey,
    pub complex_eq: VdMirFuncKey,
    /// ## le
    pub nat_le: VdMirFuncKey,
    pub int_le: VdMirFuncKey,
    pub rat_le: VdMirFuncKey,
    pub real_le: VdMirFuncKey,
    /// ## ge
    pub nat_ge: VdMirFuncKey,
    pub int_ge: VdMirFuncKey,
    pub rat_ge: VdMirFuncKey,
    pub real_ge: VdMirFuncKey,
    /// ## in
    pub in_set: VdMirFuncKey,
}

impl VdMirFuncKeyMenu {
    pub fn new(db: &::salsa::Db) -> Self {
        let VdZfcInstantiationMenu {
            nat_add,
            int_add,
            rat_add,
            real_add,
            complex_add,
            nat_mul,
            int_mul,
            rat_mul,
            real_mul,
            complex_mul,
            nat_to_the_power_of_nat,
            int_to_the_power_of_nat,
            rat_to_the_power_of_nat,
            real_to_the_power_of_nat,
            complex_to_the_power_of_nat,
            nat_eq,
            int_eq,
            rat_eq,
            real_eq,
            complex_eq,
            nat_le,
            int_le,
            rat_le,
            real_le,
            nat_ge,
            int_ge,
            rat_ge,
            real_ge,
        } = *vd_zfc_instantiation_menu(db);
        Self {
            nat_add: VdMirFuncKey::NormalBaseSeparator(nat_add),
            int_add: VdMirFuncKey::NormalBaseSeparator(int_add),
            rat_add: VdMirFuncKey::NormalBaseSeparator(rat_add),
            real_add: VdMirFuncKey::NormalBaseSeparator(real_add),
            complex_add: VdMirFuncKey::NormalBaseSeparator(complex_add),
            nat_mul: VdMirFuncKey::NormalBaseSeparator(nat_mul),
            int_mul: VdMirFuncKey::NormalBaseSeparator(int_mul),
            rat_mul: VdMirFuncKey::NormalBaseSeparator(rat_mul),
            real_mul: VdMirFuncKey::NormalBaseSeparator(real_mul),
            complex_mul: VdMirFuncKey::NormalBaseSeparator(complex_mul),
            nat_to_the_power_of_nat: VdMirFuncKey::NormalBaseSeparator(nat_to_the_power_of_nat),
            int_to_the_power_of_nat: VdMirFuncKey::NormalBaseSeparator(int_to_the_power_of_nat),
            rat_to_the_power_of_nat: VdMirFuncKey::NormalBaseSeparator(rat_to_the_power_of_nat),
            real_to_the_power_of_nat: VdMirFuncKey::NormalBaseSeparator(real_to_the_power_of_nat),
            complex_to_the_power_of_nat: VdMirFuncKey::NormalBaseSeparator(
                complex_to_the_power_of_nat,
            ),
            nat_eq: VdMirFuncKey::NormalBaseSeparator(nat_eq),
            int_eq: VdMirFuncKey::NormalBaseSeparator(int_eq),
            rat_eq: VdMirFuncKey::NormalBaseSeparator(rat_eq),
            real_eq: VdMirFuncKey::NormalBaseSeparator(real_eq),
            complex_eq: VdMirFuncKey::NormalBaseSeparator(complex_eq),
            nat_le: VdMirFuncKey::NormalBaseSeparator(nat_le),
            int_le: VdMirFuncKey::NormalBaseSeparator(int_le),
            rat_le: VdMirFuncKey::NormalBaseSeparator(rat_le),
            real_le: VdMirFuncKey::NormalBaseSeparator(real_le),
            nat_ge: VdMirFuncKey::NormalBaseSeparator(nat_ge),
            int_ge: VdMirFuncKey::NormalBaseSeparator(int_ge),
            rat_ge: VdMirFuncKey::NormalBaseSeparator(rat_ge),
            real_ge: VdMirFuncKey::NormalBaseSeparator(real_ge),
            in_set: VdMirFuncKey::InSet,
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_mir_func_key_menu(db: &::salsa::Db) -> VdMirFuncKeyMenu {
    VdMirFuncKeyMenu::new(db)
}
