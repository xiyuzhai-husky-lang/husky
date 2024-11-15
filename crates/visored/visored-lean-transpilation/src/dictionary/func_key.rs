use lean_mir_expr::expr::application::{ln_mir_func_key_menu, LnMirFuncKey, LnMirFuncKeyMenu};
use rustc_hash::FxHashMap;
use visored_mir_expr::expr::application::{
    menu::{vd_mir_func_key_menu, VdMirFuncKeyMenu},
    VdMirFunc, VdMirFuncKey,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdFuncKeyTranslation {
    BinaryOprAsSeparator(LnMirFuncKey),
    Power(LnMirFuncKey),
    InSet,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VdFuncKeyDictionary {
    translations: FxHashMap<VdMirFuncKey, VdFuncKeyTranslation>,
}

impl VdFuncKeyDictionary {
    pub fn new_standard(db: &::salsa::Db) -> Self {
        use VdFuncKeyTranslation::*;

        let VdMirFuncKeyMenu {
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
            in_set,
        } = *vd_mir_func_key_menu(db);
        let LnMirFuncKeyMenu {
            int_sub: ln_int_sub,
            rat_sub: ln_rat_sub,
            real_sub: ln_real_sub,
            complex_sub: ln_complex_sub,
            nat_add: ln_nat_add,
            int_add: ln_int_add,
            rat_add: ln_rat_add,
            real_add: ln_real_add,
            complex_add: ln_complex_add,
            nat_mul: ln_nat_mul,
            int_mul: ln_int_mul,
            rat_mul: ln_rat_mul,
            real_mul: ln_real_mul,
            complex_mul: ln_complex_mul,
            nat_to_the_power_of_nat: ln_nat_to_the_power_of_nat,
            int_to_the_power_of_nat: ln_int_to_the_power_of_nat,
            rat_to_the_power_of_nat: ln_rat_to_the_power_of_nat,
            real_to_the_power_of_nat: ln_real_to_the_power_of_nat,
            complex_to_the_power_of_nat: ln_complex_to_the_power_of_nat,
            nat_eq: ln_nat_eq,
            int_eq: ln_int_eq,
            rat_eq: ln_rat_eq,
            real_eq: ln_real_eq,
            complex_eq: ln_complex_eq,
            nat_le: ln_nat_le,
            int_le: ln_int_le,
            rat_le: ln_rat_le,
            real_le: ln_real_le,
            nat_ge: ln_nat_ge,
            int_ge: ln_int_ge,
            rat_ge: ln_rat_ge,
            real_ge: ln_real_ge,
        } = *ln_mir_func_key_menu(db);
        Self::new([
            (int_sub, BinaryOprAsSeparator(ln_int_sub)),
            (rat_sub, BinaryOprAsSeparator(ln_rat_sub)),
            (real_sub, BinaryOprAsSeparator(ln_real_sub)),
            (complex_sub, BinaryOprAsSeparator(ln_complex_sub)),
            (nat_add, BinaryOprAsSeparator(ln_nat_add)),
            (int_add, BinaryOprAsSeparator(ln_int_add)),
            (rat_add, BinaryOprAsSeparator(ln_rat_add)),
            (real_add, BinaryOprAsSeparator(ln_real_add)),
            (complex_add, BinaryOprAsSeparator(ln_complex_add)),
            (nat_mul, BinaryOprAsSeparator(ln_nat_mul)),
            (int_mul, BinaryOprAsSeparator(ln_int_mul)),
            (rat_mul, BinaryOprAsSeparator(ln_rat_mul)),
            (real_mul, BinaryOprAsSeparator(ln_real_mul)),
            (complex_mul, BinaryOprAsSeparator(ln_complex_mul)),
            (nat_to_the_power_of_nat, Power(ln_nat_to_the_power_of_nat)),
            (int_to_the_power_of_nat, Power(ln_int_to_the_power_of_nat)),
            (rat_to_the_power_of_nat, Power(ln_rat_to_the_power_of_nat)),
            (real_to_the_power_of_nat, Power(ln_real_to_the_power_of_nat)),
            (
                complex_to_the_power_of_nat,
                Power(ln_complex_to_the_power_of_nat),
            ),
            (nat_eq, BinaryOprAsSeparator(ln_nat_eq)),
            (int_eq, BinaryOprAsSeparator(ln_int_eq)),
            (rat_eq, BinaryOprAsSeparator(ln_rat_eq)),
            (real_eq, BinaryOprAsSeparator(ln_real_eq)),
            (complex_eq, BinaryOprAsSeparator(ln_complex_eq)),
            (nat_le, BinaryOprAsSeparator(ln_nat_le)),
            (int_le, BinaryOprAsSeparator(ln_int_le)),
            (rat_le, BinaryOprAsSeparator(ln_rat_le)),
            (real_le, BinaryOprAsSeparator(ln_real_le)),
            (nat_ge, BinaryOprAsSeparator(ln_nat_ge)),
            (int_ge, BinaryOprAsSeparator(ln_int_ge)),
            (rat_ge, BinaryOprAsSeparator(ln_rat_ge)),
            (real_ge, BinaryOprAsSeparator(ln_real_ge)),
            (in_set, InSet),
        ])
    }

    pub fn new(
        translations: impl IntoIterator<Item = (VdMirFuncKey, VdFuncKeyTranslation)>,
    ) -> Self {
        Self {
            translations: translations.into_iter().collect(),
        }
    }
}

impl VdFuncKeyDictionary {
    pub fn get(&self, func_key: VdMirFuncKey) -> Option<&VdFuncKeyTranslation> {
        self.translations.get(&func_key)
    }
}
