use visored_opr::separator::VdBaseSeparator;
use visored_signature::{
    menu::{vd_signature_menu, VdSignatureMenu},
    signature::separator::base::VdBaseSeparatorSignature,
};
use visored_zfc_ty::menu::{vd_zfc_ty_menu, VdZfcTypeMenu};

use crate::dispatch::separator::VdSeparatorGlobalDispatch;

#[derive(Debug, PartialEq, Eq)]
pub struct VdGlobalDispatchMenu {
    // ## add
    pub nat_add: VdSeparatorGlobalDispatch,
    pub int_add: VdSeparatorGlobalDispatch,
    pub rat_add: VdSeparatorGlobalDispatch,
    pub real_add: VdSeparatorGlobalDispatch,
    // ## eq
    pub nat_eq: VdSeparatorGlobalDispatch,
    pub int_eq: VdSeparatorGlobalDispatch,
    pub rat_eq: VdSeparatorGlobalDispatch,
    pub real_eq: VdSeparatorGlobalDispatch,
    // ## in
    pub in_set: VdSeparatorGlobalDispatch,
}

#[salsa::tracked(return_ref)]
pub fn vd_global_dispatch_menu(db: &::salsa::Db) -> VdGlobalDispatchMenu {
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
    let VdSignatureMenu {
        nat_add,
        int_add,
        rat_add,
        real_add,
        nat_eq,
        int_eq,
        rat_eq,
        real_eq,
    } = vd_signature_menu(db);
    VdGlobalDispatchMenu {
        nat_add: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Add,
            signature: nat_add.clone(),
        },
        int_add: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Add,
            signature: int_add.clone(),
        },
        rat_add: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Add,
            signature: rat_add.clone(),
        },
        real_add: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Add,
            signature: real_add.clone(),
        },
        nat_eq: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Eq,
            signature: nat_eq.clone(),
        },
        int_eq: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Eq,
            signature: int_eq.clone(),
        },
        rat_eq: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Eq,
            signature: rat_eq.clone(),
        },
        real_eq: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Eq,
            signature: real_eq.clone(),
        },
        in_set: VdSeparatorGlobalDispatch::InSet { expr_ty: prop_ty },
    }
}
