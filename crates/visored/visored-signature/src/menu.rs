use visored_zfc_ty::{
    instantiation::menu::{vd_zfc_instantiation_menu, VdZfcInstantiationMenu},
    menu::{vd_zfc_ty_menu, VdZfcTypeMenu},
};

use crate::signature::{
    binary_opr::base::VdBaseBinaryOprSignature, separator::base::VdBaseSeparatorSignature,
};

#[derive(Debug, PartialEq, Eq)]
pub struct VdSignatureMenu {
    // ## add
    pub nat_add: VdBaseSeparatorSignature,
    pub int_add: VdBaseSeparatorSignature,
    pub rat_add: VdBaseSeparatorSignature,
    pub real_add: VdBaseSeparatorSignature,
    pub complex_add: VdBaseSeparatorSignature,
    // ## mul
    pub nat_mul: VdBaseSeparatorSignature,
    pub int_mul: VdBaseSeparatorSignature,
    pub rat_mul: VdBaseSeparatorSignature,
    pub real_mul: VdBaseSeparatorSignature,
    pub complex_mul: VdBaseSeparatorSignature,
    // ## eq
    pub nat_eq: VdBaseSeparatorSignature,
    pub int_eq: VdBaseSeparatorSignature,
    pub rat_eq: VdBaseSeparatorSignature,
    pub real_eq: VdBaseSeparatorSignature,
    pub complex_eq: VdBaseSeparatorSignature,
    // ## le
    pub nat_le: VdBaseSeparatorSignature,
    pub int_le: VdBaseSeparatorSignature,
    pub rat_le: VdBaseSeparatorSignature,
    pub real_le: VdBaseSeparatorSignature,
    // ## ge
    pub nat_ge: VdBaseSeparatorSignature,
    pub int_ge: VdBaseSeparatorSignature,
    pub rat_ge: VdBaseSeparatorSignature,
    pub real_ge: VdBaseSeparatorSignature,
}

impl VdSignatureMenu {
    fn new(db: &::salsa::Db) -> Self {
        let VdZfcTypeMenu {
            nat,
            int,
            rat,
            real,
            complex,
            set,
            prop,
        } = *vd_zfc_ty_menu(db);
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
            nat_add: VdBaseSeparatorSignature::new(db, nat_add.clone(), nat, nat),
            int_add: VdBaseSeparatorSignature::new(db, int_add.clone(), int, int),
            rat_add: VdBaseSeparatorSignature::new(db, rat_add.clone(), rat, rat),
            real_add: VdBaseSeparatorSignature::new(db, real_add.clone(), real, real),
            complex_add: VdBaseSeparatorSignature::new(db, complex_add.clone(), complex, complex),
            nat_mul: VdBaseSeparatorSignature::new(db, nat_mul.clone(), nat, nat),
            int_mul: VdBaseSeparatorSignature::new(db, int_mul.clone(), int, int),
            rat_mul: VdBaseSeparatorSignature::new(db, rat_mul.clone(), rat, rat),
            real_mul: VdBaseSeparatorSignature::new(db, real_mul.clone(), real, real),
            complex_mul: VdBaseSeparatorSignature::new(db, complex_mul.clone(), complex, complex),
            nat_eq: VdBaseSeparatorSignature::new(db, nat_eq.clone(), nat, nat),
            int_eq: VdBaseSeparatorSignature::new(db, int_eq.clone(), int, int),
            rat_eq: VdBaseSeparatorSignature::new(db, rat_eq.clone(), rat, rat),
            real_eq: VdBaseSeparatorSignature::new(db, real_eq.clone(), real, real),
            complex_eq: VdBaseSeparatorSignature::new(db, complex_eq.clone(), complex, complex),
            nat_le: VdBaseSeparatorSignature::new(db, nat_le.clone(), nat, nat),
            int_le: VdBaseSeparatorSignature::new(db, int_le.clone(), int, int),
            rat_le: VdBaseSeparatorSignature::new(db, rat_le.clone(), rat, rat),
            real_le: VdBaseSeparatorSignature::new(db, real_le.clone(), real, real),
            nat_ge: VdBaseSeparatorSignature::new(db, nat_ge.clone(), nat, nat),
            int_ge: VdBaseSeparatorSignature::new(db, int_ge.clone(), int, int),
            rat_ge: VdBaseSeparatorSignature::new(db, rat_ge.clone(), rat, rat),
            real_ge: VdBaseSeparatorSignature::new(db, real_ge.clone(), real, real),
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_signature_menu(db: &::salsa::Db) -> VdSignatureMenu {
    VdSignatureMenu::new(db)
}
