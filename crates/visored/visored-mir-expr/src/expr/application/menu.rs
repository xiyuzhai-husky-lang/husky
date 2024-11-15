use super::*;
use visored_zfc_ty::instantiation::menu::{vd_zfc_instantiation_menu, VdZfcInstantiationMenu};

#[derive(Debug, PartialEq, Eq)]
pub struct VdMirFuncKeyMenu {
    pub nat_add: VdMirFuncKey,
    pub int_add: VdMirFuncKey,
    pub rat_add: VdMirFuncKey,
    pub real_add: VdMirFuncKey,
    pub nat_eq: VdMirFuncKey,
    pub int_eq: VdMirFuncKey,
    pub rat_eq: VdMirFuncKey,
    pub real_eq: VdMirFuncKey,
    pub complex_eq: VdMirFuncKey,
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
            nat_eq,
            int_eq,
            rat_eq,
            real_eq,
            complex_eq,
        } = *vd_zfc_instantiation_menu(db);
        Self {
            nat_add: VdMirFuncKey::NormalBaseSeparator(nat_add),
            int_add: VdMirFuncKey::NormalBaseSeparator(int_add),
            rat_add: VdMirFuncKey::NormalBaseSeparator(rat_add),
            real_add: VdMirFuncKey::NormalBaseSeparator(real_add),
            nat_eq: VdMirFuncKey::NormalBaseSeparator(nat_eq),
            int_eq: VdMirFuncKey::NormalBaseSeparator(int_eq),
            rat_eq: VdMirFuncKey::NormalBaseSeparator(rat_eq),
            real_eq: VdMirFuncKey::NormalBaseSeparator(real_eq),
            complex_eq: VdMirFuncKey::NormalBaseSeparator(complex_eq),
            in_set: VdMirFuncKey::InSet,
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_mir_func_key_menu(db: &::salsa::Db) -> VdMirFuncKeyMenu {
    VdMirFuncKeyMenu::new(db)
}
