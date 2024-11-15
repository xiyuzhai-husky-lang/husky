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
    pub complex_add: VdSeparatorGlobalDispatch,
    // ## mul
    pub nat_space_mul: VdSeparatorGlobalDispatch,
    pub int_space_mul: VdSeparatorGlobalDispatch,
    pub rat_space_mul: VdSeparatorGlobalDispatch,
    pub real_space_mul: VdSeparatorGlobalDispatch,
    pub complex_space_mul: VdSeparatorGlobalDispatch,
    // ## eq
    pub nat_eq: VdSeparatorGlobalDispatch,
    pub int_eq: VdSeparatorGlobalDispatch,
    pub rat_eq: VdSeparatorGlobalDispatch,
    pub real_eq: VdSeparatorGlobalDispatch,
    pub complex_eq: VdSeparatorGlobalDispatch,
    // ## le
    pub nat_le: VdSeparatorGlobalDispatch,
    pub int_le: VdSeparatorGlobalDispatch,
    pub rat_le: VdSeparatorGlobalDispatch,
    pub real_le: VdSeparatorGlobalDispatch,
    // ## ge
    pub nat_ge: VdSeparatorGlobalDispatch,
    pub int_ge: VdSeparatorGlobalDispatch,
    pub rat_ge: VdSeparatorGlobalDispatch,
    pub real_ge: VdSeparatorGlobalDispatch,
    // ## in
    pub in_set: VdSeparatorGlobalDispatch,
}

#[salsa::tracked(return_ref)]
pub fn vd_global_dispatch_menu(db: &::salsa::Db) -> VdGlobalDispatchMenu {
    let VdZfcTypeMenu {
        nat,
        int,
        rat,
        real,
        complex,
        set,
        prop,
    } = *vd_zfc_ty_menu(db);
    let VdSignatureMenu {
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
        complex_add: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Add,
            signature: complex_add.clone(),
        },
        nat_space_mul: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Space,
            signature: nat_mul.clone(),
        },
        int_space_mul: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Space,
            signature: int_mul.clone(),
        },
        rat_space_mul: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Space,
            signature: rat_mul.clone(),
        },
        real_space_mul: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Space,
            signature: real_mul.clone(),
        },
        complex_space_mul: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Space,
            signature: complex_mul.clone(),
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
        complex_eq: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Eq,
            signature: complex_eq.clone(),
        },
        nat_le: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Le,
            signature: nat_le.clone(),
        },
        int_le: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Le,
            signature: int_le.clone(),
        },
        rat_le: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Le,
            signature: rat_le.clone(),
        },
        real_le: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Le,
            signature: real_le.clone(),
        },
        nat_ge: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Ge,
            signature: nat_ge.clone(),
        },
        int_ge: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Ge,
            signature: int_ge.clone(),
        },
        rat_ge: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Ge,
            signature: rat_ge.clone(),
        },
        real_ge: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Ge,
            signature: real_ge.clone(),
        },
        in_set: VdSeparatorGlobalDispatch::InSet { expr_ty: prop },
    }
}
