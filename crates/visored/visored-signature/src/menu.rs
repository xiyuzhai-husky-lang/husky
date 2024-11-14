use visored_zfc_ty::menu::{vd_zfc_ty_menu, VdZfcTypeMenu};

use crate::signature::{
    binary_opr::base::VdBaseBinaryOprSignature, separator::base::VdBaseSeparatorSignature,
};

#[derive(Debug, PartialEq, Eq)]
pub struct VdSignatureMenu {
    pub int_add: VdBaseSeparatorSignature,
    pub rat_add: VdBaseSeparatorSignature,
    pub real_add: VdBaseSeparatorSignature,
}

impl VdSignatureMenu {
    fn new(db: &::salsa::Db) -> Self {
        let VdZfcTypeMenu {
            zero_literal,
            one_literal,
            two_literal,
            nat_ty,
            int_ty,
            rat_ty,
            real_ty,
            set_ty,
            prop_ty,
        } = *vd_zfc_ty_menu(db);
        Self {
            int_add: VdBaseSeparatorSignature::new(int_ty, int_ty),
            rat_add: VdBaseSeparatorSignature::new(rat_ty, rat_ty),
            real_add: VdBaseSeparatorSignature::new(real_ty, real_ty),
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_signature_menu(db: &::salsa::Db) -> VdSignatureMenu {
    VdSignatureMenu::new(db)
}
