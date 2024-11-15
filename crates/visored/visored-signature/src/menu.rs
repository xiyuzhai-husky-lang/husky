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
    // ## eq
    pub nat_eq: VdBaseSeparatorSignature,
    pub int_eq: VdBaseSeparatorSignature,
    pub rat_eq: VdBaseSeparatorSignature,
    pub real_eq: VdBaseSeparatorSignature,
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
            nat_eq,
            int_eq,
            rat_eq,
            real_eq,
        } = *vd_zfc_instantiation_menu(db);
        Self {
            nat_add: VdBaseSeparatorSignature::new(db, nat_add.clone(), nat, nat),
            int_add: VdBaseSeparatorSignature::new(db, int_add.clone(), int, int),
            rat_add: VdBaseSeparatorSignature::new(db, rat_add.clone(), rat, rat),
            real_add: VdBaseSeparatorSignature::new(db, real_add.clone(), real, real),
            nat_eq: VdBaseSeparatorSignature::new(db, nat_eq.clone(), nat, nat),
            int_eq: VdBaseSeparatorSignature::new(db, int_eq.clone(), int, int),
            rat_eq: VdBaseSeparatorSignature::new(db, rat_eq.clone(), rat, rat),
            real_eq: VdBaseSeparatorSignature::new(db, real_eq.clone(), real, real),
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_signature_menu(db: &::salsa::Db) -> VdSignatureMenu {
    VdSignatureMenu::new(db)
}
