use rustc_hash::FxHashMap;
use visored_mir_expr::expr::application::{
    menu::{vd_mir_func_key_menu, VdMirFuncKeyMenu},
    VdMirFunc, VdMirFuncKey,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdFuncKeyTranslation {
    NormalSeparator,
    InSet,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VdFuncKeyDictionary {
    translations: FxHashMap<VdMirFuncKey, VdFuncKeyTranslation>,
}

impl VdFuncKeyDictionary {
    pub fn new_standard(db: &::salsa::Db) -> Self {
        let VdMirFuncKeyMenu {
            nat_add,
            int_add,
            rat_add,
            real_add,
            nat_eq,
            int_eq,
            rat_eq,
            real_eq,
            in_set,
        } = *vd_mir_func_key_menu(db);
        Self::new([
            (nat_add, VdFuncKeyTranslation::NormalSeparator),
            (int_add, VdFuncKeyTranslation::NormalSeparator),
            (rat_add, VdFuncKeyTranslation::NormalSeparator),
            (real_add, VdFuncKeyTranslation::NormalSeparator),
            (nat_eq, VdFuncKeyTranslation::NormalSeparator),
            (int_eq, VdFuncKeyTranslation::NormalSeparator),
            (rat_eq, VdFuncKeyTranslation::NormalSeparator),
            (real_eq, VdFuncKeyTranslation::NormalSeparator),
            (in_set, VdFuncKeyTranslation::InSet),
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
