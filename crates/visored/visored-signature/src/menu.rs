use visored_term::{
    instantiation::menu::{vd_instantiation_menu, VdInstantiationMenu},
    menu::{vd_ty_menu, VdTypeMenu},
};

use crate::signature::{
    attach::VdPowerSignature, binary_opr::base::VdBaseBinaryOprSignature,
    prefix_opr::VdBasePrefixOprSignature, separator::base::VdBaseSeparatorSignature,
};

#[derive(Debug, PartialEq, Eq)]
pub struct VdSignatureMenu {
    /// # prefix
    /// ## pos
    pub int_pos: VdBasePrefixOprSignature,
    pub rat_pos: VdBasePrefixOprSignature,
    pub real_pos: VdBasePrefixOprSignature,
    pub complex_pos: VdBasePrefixOprSignature,
    /// ## neg
    pub int_neg: VdBasePrefixOprSignature,
    pub rat_neg: VdBasePrefixOprSignature,
    pub real_neg: VdBasePrefixOprSignature,
    pub complex_neg: VdBasePrefixOprSignature,
    /// # binary
    /// ## sub
    pub int_sub: VdBaseBinaryOprSignature,
    pub rat_sub: VdBaseBinaryOprSignature,
    pub real_sub: VdBaseBinaryOprSignature,
    pub complex_sub: VdBaseBinaryOprSignature,
    /// ## add
    pub nat_add: VdBaseSeparatorSignature,
    pub int_add: VdBaseSeparatorSignature,
    pub rat_add: VdBaseSeparatorSignature,
    pub real_add: VdBaseSeparatorSignature,
    pub complex_add: VdBaseSeparatorSignature,
    /// ## mul
    pub nat_mul: VdBaseSeparatorSignature,
    pub int_mul: VdBaseSeparatorSignature,
    pub rat_mul: VdBaseSeparatorSignature,
    pub real_mul: VdBaseSeparatorSignature,
    pub complex_mul: VdBaseSeparatorSignature,
    /// ## power
    pub nat_to_the_power_of_nat: VdPowerSignature,
    pub int_to_the_power_of_nat: VdPowerSignature,
    pub rat_to_the_power_of_nat: VdPowerSignature,
    pub real_to_the_power_of_nat: VdPowerSignature,
    pub complex_to_the_power_of_nat: VdPowerSignature,
    /// ## eq
    pub nat_eq: VdBaseSeparatorSignature,
    pub int_eq: VdBaseSeparatorSignature,
    pub rat_eq: VdBaseSeparatorSignature,
    pub real_eq: VdBaseSeparatorSignature,
    pub complex_eq: VdBaseSeparatorSignature,
    /// ## le
    pub nat_le: VdBaseSeparatorSignature,
    pub int_le: VdBaseSeparatorSignature,
    pub rat_le: VdBaseSeparatorSignature,
    pub real_le: VdBaseSeparatorSignature,
    /// ## ge
    pub nat_ge: VdBaseSeparatorSignature,
    pub int_ge: VdBaseSeparatorSignature,
    pub rat_ge: VdBaseSeparatorSignature,
    pub real_ge: VdBaseSeparatorSignature,
}

impl VdSignatureMenu {
    fn new(db: &::salsa::Db) -> Self {
        let VdTypeMenu {
            nat,
            int,
            rat,
            real,
            complex,
            set,
            prop,
        } = *vd_ty_menu(db);
        let VdInstantiationMenu {
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
            nat_le,
            int_le,
            rat_le,
            real_le,
            nat_ge,
            int_ge,
            rat_ge,
            real_ge,
        } = *vd_instantiation_menu(db);
        let pre = VdBasePrefixOprSignature::new;
        let bin = VdBaseBinaryOprSignature::new;
        let sep = VdBaseSeparatorSignature::new;
        let pow = VdPowerSignature::new;
        Self {
            int_pos: pre(int_pos, int, int, int),
            rat_pos: pre(rat_pos, rat, rat, rat),
            real_pos: pre(real_pos, real, real, real),
            complex_pos: pre(complex_pos, complex, complex, complex),
            int_neg: pre(int_neg, int, int, int),
            rat_neg: pre(rat_neg, rat, rat, rat),
            real_neg: pre(real_neg, real, real, real),
            complex_neg: pre(complex_neg, complex, complex, complex),
            int_sub: bin(int_sub, int, int, int),
            rat_sub: bin(rat_sub, rat, rat, rat),
            real_sub: bin(real_sub, real, real, real),
            complex_sub: bin(complex_sub, complex, complex, complex),
            nat_add: sep(nat_add, nat, nat),
            int_add: sep(int_add, int, int),
            rat_add: sep(rat_add, rat, rat),
            real_add: sep(real_add, real, real),
            complex_add: sep(complex_add, complex, complex),
            nat_mul: sep(nat_mul, nat, nat),
            int_mul: sep(int_mul, int, int),
            rat_mul: sep(rat_mul, rat, rat),
            real_mul: sep(real_mul, real, real),
            complex_mul: sep(complex_mul, complex, complex),
            nat_to_the_power_of_nat: pow(nat_to_the_power_of_nat, nat, nat, nat),
            int_to_the_power_of_nat: pow(int_to_the_power_of_nat, int, nat, int),
            rat_to_the_power_of_nat: pow(rat_to_the_power_of_nat, rat, nat, rat),
            real_to_the_power_of_nat: pow(real_to_the_power_of_nat, real, nat, real),
            complex_to_the_power_of_nat: pow(complex_to_the_power_of_nat, complex, nat, complex),
            nat_eq: sep(nat_eq, nat, nat),
            int_eq: sep(int_eq, int, int),
            rat_eq: sep(rat_eq, rat, rat),
            real_eq: sep(real_eq, real, real),
            complex_eq: sep(complex_eq, complex, complex),
            nat_le: sep(nat_le, nat, nat),
            int_le: sep(int_le, int, int),
            rat_le: sep(rat_le, rat, rat),
            real_le: sep(real_le, real, real),
            nat_ge: sep(nat_ge, nat, nat),
            int_ge: sep(int_ge, int, int),
            rat_ge: sep(rat_ge, rat, rat),
            real_ge: sep(real_ge, real, real),
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_signature_menu(db: &::salsa::Db) -> VdSignatureMenu {
    VdSignatureMenu::new(db)
}
