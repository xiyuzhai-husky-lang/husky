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
        } = *vd_zfc_ty_menu(db);
        let VdZfcInstantiationMenu {
            nat_add_instantiation,
            int_add_instantiation,
            rat_add_instantiation,
            real_add_instantiation,
            nat_eq_instantiation,
            int_eq_instantiation,
            rat_eq_instantiation,
            real_eq_instantiation,
        } = *vd_zfc_instantiation_menu(db);
        Self {
            nat_add: VdBaseSeparatorSignature::new(nat_add_instantiation.clone(), nat_ty, nat_ty),
            int_add: VdBaseSeparatorSignature::new(int_add_instantiation.clone(), int_ty, int_ty),
            rat_add: VdBaseSeparatorSignature::new(rat_add_instantiation.clone(), rat_ty, rat_ty),
            real_add: VdBaseSeparatorSignature::new(
                real_add_instantiation.clone(),
                real_ty,
                real_ty,
            ),
            nat_eq: VdBaseSeparatorSignature::new(nat_eq_instantiation.clone(), nat_ty, nat_ty),
            int_eq: VdBaseSeparatorSignature::new(int_eq_instantiation.clone(), int_ty, int_ty),
            rat_eq: VdBaseSeparatorSignature::new(rat_eq_instantiation.clone(), rat_ty, rat_ty),
            real_eq: VdBaseSeparatorSignature::new(real_eq_instantiation.clone(), real_ty, real_ty),
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_signature_menu(db: &::salsa::Db) -> VdSignatureMenu {
    VdSignatureMenu::new(db)
}
