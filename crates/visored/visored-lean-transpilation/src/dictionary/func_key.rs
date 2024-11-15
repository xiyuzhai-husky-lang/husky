use lean_mir_expr::expr::application::{ln_mir_func_key_menu, LnMirFuncKey, LnMirFuncKeyMenu};
use rustc_hash::FxHashMap;
use visored_mir_expr::expr::application::{
    menu::{vd_mir_func_key_menu, VdMirFuncKeyMenu},
    VdMirFunc, VdMirFuncKey,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdFuncKeyTranslation {
    BinaryOprAsSeparator(LnMirFuncKey),
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
            nat_add,
            int_add,
            rat_add,
            real_add,
            nat_eq,
            int_eq,
            rat_eq,
            real_eq,
            complex_eq,
            in_set,
        } = *vd_mir_func_key_menu(db);
        let LnMirFuncKeyMenu {
            nat_add: ln_nat_add,
            int_add: ln_int_add,
            rat_add: ln_rat_add,
            real_add: ln_real_add,
            complex_add: ln_complex_add,
            nat_eq: ln_nat_eq,
            int_eq: ln_int_eq,
            rat_eq: ln_rat_eq,
            real_eq: ln_real_eq,
            complex_eq: ln_complex_eq,
        } = *ln_mir_func_key_menu(db);
        Self::new([
            (nat_add, BinaryOprAsSeparator(ln_nat_add)),
            (int_add, BinaryOprAsSeparator(ln_int_add)),
            (rat_add, BinaryOprAsSeparator(ln_rat_add)),
            (real_add, BinaryOprAsSeparator(ln_real_add)),
            (nat_eq, BinaryOprAsSeparator(ln_nat_eq)),
            (int_eq, BinaryOprAsSeparator(ln_int_eq)),
            (rat_eq, BinaryOprAsSeparator(ln_rat_eq)),
            (real_eq, BinaryOprAsSeparator(ln_real_eq)),
            (complex_eq, BinaryOprAsSeparator(ln_complex_eq)),
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
