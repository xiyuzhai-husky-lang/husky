use crate::{
    instantiation::VdInstantiation,
    term::{
        literal::{VdZfcLiteral, VdZfcLiteralData},
        VdZfcTerm,
    },
    ty::{VdZfcType, VdZfcTypeData},
};
use smallvec::{smallvec, SmallVec};
use visored_item_path::{
    menu::{vd_item_path_menu, VdItemPathMenu},
    path::VdItemPath,
};

#[derive(Debug, PartialEq, Eq)]
pub struct VdZfcTypeMenu {
    /// natural numbers as a type
    pub nat: VdZfcType,
    /// integers as a type
    pub int: VdZfcType,
    /// rational numbers as a type
    pub rat: VdZfcType,
    /// real numbers as a type
    pub real: VdZfcType,
    /// complex numbers as a type
    pub complex: VdZfcType,
    /// the category of sets as a type
    pub set: VdZfcType,
    /// the category of propositions as a type
    pub prop: VdZfcType,
}

impl VdZfcTypeMenu {
    fn new(db: &::salsa::Db) -> Self {
        let VdItemPathMenu {
            set,
            prop,
            nat,
            rat,
            int,
            real,
            complex,
            sin,
            cos,
            group,
            ring,
            group_mul,
            abelian_group_add,
            ring_add,
            ring_mul,
            ring_power,
        } = *vd_item_path_menu(db);

        let nat = VdZfcType::new_item_path(nat.into(), db);
        let int = VdZfcType::new_item_path(int.into(), db);
        let rat = VdZfcType::new_item_path(rat.into(), db);
        let real = VdZfcType::new_item_path(real.into(), db);
        let complex = VdZfcType::new_item_path(complex.into(), db);
        let set = VdZfcType::new_item_path(set.into(), db);
        let prop = VdZfcType::new_item_path(prop.into(), db);

        Self {
            nat,
            int,
            rat,
            real,
            complex,
            set,
            prop,
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_zfc_ty_menu(db: &::salsa::Db) -> VdZfcTypeMenu {
    VdZfcTypeMenu::new(db)
}
