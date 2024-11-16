use visored_opr::{
    opr::{binary::VdBaseBinaryOpr, prefix::VdBasePrefixOpr},
    separator::VdBaseSeparator,
};
use visored_signature::{
    menu::{vd_signature_menu, VdSignatureMenu},
    signature::{
        attach::{VdAttachSignature, VdPowerSignature},
        prefix_opr::VdBasePrefixOprSignature,
        separator::base::VdBaseSeparatorSignature,
    },
};
use visored_term::menu::{vd_ty_menu, VdTypeMenu};

use crate::dispatch::{
    attach::VdAttachGlobalDispatch, binary_opr::VdBinaryOprGlobalDispatch,
    prefix_opr::VdPrefixOprGlobalDispatch, separator::VdSeparatorGlobalDispatch,
};

#[derive(Debug, PartialEq, Eq)]
pub struct VdGlobalDispatchMenu {
    /// # prefix oprs
    /// ## pos
    pub int_pos: VdPrefixOprGlobalDispatch,
    pub rat_pos: VdPrefixOprGlobalDispatch,
    pub real_pos: VdPrefixOprGlobalDispatch,
    pub complex_pos: VdPrefixOprGlobalDispatch,
    /// ## neg
    pub int_neg: VdPrefixOprGlobalDispatch,
    pub rat_neg: VdPrefixOprGlobalDispatch,
    pub real_neg: VdPrefixOprGlobalDispatch,
    pub complex_neg: VdPrefixOprGlobalDispatch,
    /// # binary oprs
    /// ## sub
    pub int_sub: VdBinaryOprGlobalDispatch,
    pub rat_sub: VdBinaryOprGlobalDispatch,
    pub real_sub: VdBinaryOprGlobalDispatch,
    pub complex_sub: VdBinaryOprGlobalDispatch,
    /// # separators
    /// ## add
    pub nat_add: VdSeparatorGlobalDispatch,
    pub int_add: VdSeparatorGlobalDispatch,
    pub rat_add: VdSeparatorGlobalDispatch,
    pub real_add: VdSeparatorGlobalDispatch,
    pub complex_add: VdSeparatorGlobalDispatch,
    /// ## mul
    pub nat_space_mul: VdSeparatorGlobalDispatch,
    pub int_space_mul: VdSeparatorGlobalDispatch,
    pub rat_space_mul: VdSeparatorGlobalDispatch,
    pub real_space_mul: VdSeparatorGlobalDispatch,
    pub complex_space_mul: VdSeparatorGlobalDispatch,
    /// ## power
    pub nat_to_the_power_of_nat: VdAttachGlobalDispatch,
    pub int_to_the_power_of_nat: VdAttachGlobalDispatch,
    pub rat_to_the_power_of_nat: VdAttachGlobalDispatch,
    pub real_to_the_power_of_nat: VdAttachGlobalDispatch,
    pub complex_to_the_power_of_nat: VdAttachGlobalDispatch,
    /// ## eq
    pub nat_eq: VdSeparatorGlobalDispatch,
    pub int_eq: VdSeparatorGlobalDispatch,
    pub rat_eq: VdSeparatorGlobalDispatch,
    pub real_eq: VdSeparatorGlobalDispatch,
    pub complex_eq: VdSeparatorGlobalDispatch,
    /// ## ne
    pub nat_ne: VdSeparatorGlobalDispatch,
    pub int_ne: VdSeparatorGlobalDispatch,
    pub rat_ne: VdSeparatorGlobalDispatch,
    pub real_ne: VdSeparatorGlobalDispatch,
    pub complex_ne: VdSeparatorGlobalDispatch,
    /// ## lt
    pub nat_lt: VdSeparatorGlobalDispatch,
    pub int_lt: VdSeparatorGlobalDispatch,
    pub rat_lt: VdSeparatorGlobalDispatch,
    pub real_lt: VdSeparatorGlobalDispatch,
    /// ## gt
    pub nat_gt: VdSeparatorGlobalDispatch,
    pub int_gt: VdSeparatorGlobalDispatch,
    pub rat_gt: VdSeparatorGlobalDispatch,
    pub real_gt: VdSeparatorGlobalDispatch,
    /// ## le
    pub nat_le: VdSeparatorGlobalDispatch,
    pub int_le: VdSeparatorGlobalDispatch,
    pub rat_le: VdSeparatorGlobalDispatch,
    pub real_le: VdSeparatorGlobalDispatch,
    /// ## ge
    pub nat_ge: VdSeparatorGlobalDispatch,
    pub int_ge: VdSeparatorGlobalDispatch,
    pub rat_ge: VdSeparatorGlobalDispatch,
    pub real_ge: VdSeparatorGlobalDispatch,
    /// ## in
    pub in_set: VdSeparatorGlobalDispatch,
}

#[salsa::tracked(return_ref)]
pub fn vd_global_dispatch_menu(db: &::salsa::Db) -> VdGlobalDispatchMenu {
    let VdTypeMenu {
        nat,
        int,
        rat,
        real,
        complex,
        set,
        prop,
    } = *vd_ty_menu(db);
    let VdSignatureMenu {
        int_pos,
        rat_pos,
        real_pos,
        complex_pos,
        int_neg,
        rat_neg,
        real_neg,
        complex_neg,
        int_sub,
        rat_sub,
        real_sub,
        complex_sub,
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
        nat_ne,
        int_ne,
        rat_ne,
        real_ne,
        complex_ne,
        nat_lt,
        int_lt,
        rat_lt,
        real_lt,
        nat_gt,
        int_gt,
        rat_gt,
        real_gt,
        nat_le,
        int_le,
        rat_le,
        real_le,
        nat_ge,
        int_ge,
        rat_ge,
        real_ge,
    } = *vd_signature_menu(db);
    let pow = |signature: VdPowerSignature| VdAttachGlobalDispatch::Normal {
        signature: VdAttachSignature::Power(signature),
    };
    VdGlobalDispatchMenu {
        // ## pos
        int_pos: VdPrefixOprGlobalDispatch::Base {
            base_opr: VdBasePrefixOpr::POS,
            signature: int_pos,
        },
        rat_pos: VdPrefixOprGlobalDispatch::Base {
            base_opr: VdBasePrefixOpr::POS,
            signature: rat_pos,
        },
        real_pos: VdPrefixOprGlobalDispatch::Base {
            base_opr: VdBasePrefixOpr::POS,
            signature: real_pos,
        },
        complex_pos: VdPrefixOprGlobalDispatch::Base {
            base_opr: VdBasePrefixOpr::POS,
            signature: complex_pos,
        },
        int_neg: VdPrefixOprGlobalDispatch::Base {
            base_opr: VdBasePrefixOpr::NEG,
            signature: int_neg,
        },
        rat_neg: VdPrefixOprGlobalDispatch::Base {
            base_opr: VdBasePrefixOpr::NEG,
            signature: rat_neg,
        },
        real_neg: VdPrefixOprGlobalDispatch::Base {
            base_opr: VdBasePrefixOpr::NEG,
            signature: real_neg,
        },
        complex_neg: VdPrefixOprGlobalDispatch::Base {
            base_opr: VdBasePrefixOpr::NEG,
            signature: complex_neg,
        },
        int_sub: VdBinaryOprGlobalDispatch::Normal {
            base_binary_opr: VdBaseBinaryOpr::Sub,
            signature: int_sub,
        },
        rat_sub: VdBinaryOprGlobalDispatch::Normal {
            base_binary_opr: VdBaseBinaryOpr::Sub,
            signature: rat_sub,
        },
        real_sub: VdBinaryOprGlobalDispatch::Normal {
            base_binary_opr: VdBaseBinaryOpr::Sub,
            signature: real_sub,
        },
        complex_sub: VdBinaryOprGlobalDispatch::Normal {
            base_binary_opr: VdBaseBinaryOpr::Sub,
            signature: complex_sub,
        },
        nat_add: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Add,
            signature: nat_add,
        },
        int_add: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Add,
            signature: int_add.clone(),
        },
        rat_add: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Add,
            signature: rat_add,
        },
        real_add: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Add,
            signature: real_add,
        },
        complex_add: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Add,
            signature: complex_add,
        },
        nat_space_mul: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Space,
            signature: nat_mul,
        },
        int_space_mul: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Space,
            signature: int_mul,
        },
        rat_space_mul: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Space,
            signature: rat_mul,
        },
        real_space_mul: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Space,
            signature: real_mul,
        },
        complex_space_mul: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Space,
            signature: complex_mul,
        },
        nat_to_the_power_of_nat: pow(nat_to_the_power_of_nat),
        int_to_the_power_of_nat: pow(int_to_the_power_of_nat),
        rat_to_the_power_of_nat: pow(rat_to_the_power_of_nat),
        real_to_the_power_of_nat: pow(real_to_the_power_of_nat),
        complex_to_the_power_of_nat: pow(complex_to_the_power_of_nat),
        nat_eq: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Eq,
            signature: nat_eq,
        },
        int_eq: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Eq,
            signature: int_eq,
        },
        rat_eq: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Eq,
            signature: rat_eq,
        },
        real_eq: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Eq,
            signature: real_eq,
        },
        complex_eq: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Eq,
            signature: complex_eq,
        },
        nat_ne: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Ne,
            signature: nat_ne,
        },
        int_ne: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Ne,
            signature: int_ne,
        },
        rat_ne: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Le,
            signature: nat_le,
        },
        real_ne: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Ne,
            signature: real_ne,
        },
        complex_ne: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Ne,
            signature: complex_ne,
        },
        nat_lt: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Lt,
            signature: nat_lt,
        },
        int_lt: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Lt,
            signature: int_lt,
        },
        rat_lt: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Lt,
            signature: rat_lt,
        },
        real_lt: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Lt,
            signature: real_lt,
        },
        nat_gt: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Gt,
            signature: nat_gt,
        },
        int_gt: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Gt,
            signature: int_gt,
        },
        rat_gt: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Gt,
            signature: rat_gt,
        },
        real_gt: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Gt,
            signature: real_gt,
        },
        nat_le: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Le,
            signature: int_le,
        },
        int_le: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Le,
            signature: int_le,
        },
        rat_le: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Le,
            signature: rat_le,
        },
        real_le: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Le,
            signature: real_le,
        },
        nat_ge: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Ge,
            signature: nat_ge,
        },
        int_ge: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Ge,
            signature: int_ge,
        },
        rat_ge: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Ge,
            signature: rat_ge,
        },
        real_ge: VdSeparatorGlobalDispatch::Normal {
            base_separator: VdBaseSeparator::Ge,
            signature: real_ge,
        },
        in_set: VdSeparatorGlobalDispatch::InSet { expr_ty: prop },
    }
}
