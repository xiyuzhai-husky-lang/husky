use super::*;
use crate::{
    menu::{vd_zfc_ty_menu, VdZfcTypeMenu},
    term::menu::{vd_zfc_term_menu, VdZfcTermMenu},
};
use smallvec::{smallvec, SmallVec};
use visored_item_path::menu::{vd_item_path_menu, VdItemPathMenu};

#[derive(Debug, PartialEq, Eq)]
pub struct VdZfcInstantiationMenu {
    /// ## add
    pub nat_add: VdInstantiation,
    pub int_add: VdInstantiation,
    pub rat_add: VdInstantiation,
    pub real_add: VdInstantiation,
    pub complex_add: VdInstantiation,
    /// ## eq
    pub nat_eq: VdInstantiation,
    pub int_eq: VdInstantiation,
    pub rat_eq: VdInstantiation,
    pub real_eq: VdInstantiation,
    pub complex_eq: VdInstantiation,
}

impl VdZfcInstantiationMenu {
    pub fn new(db: &salsa::Db) -> Self {
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
        } = *vd_item_path_menu(db);
        let VdZfcTermMenu {
            zero,
            one,
            two,
            nat,
            int,
            rat,
            real,
            complex,
        } = *vd_zfc_term_menu(db);
        let nat_add = VdInstantiation::new(db, ring_add.into(), smallvec![nat]);
        let int_add = VdInstantiation::new(db, ring_add.into(), smallvec![int]);
        let rat_add = VdInstantiation::new(db, ring_add.into(), smallvec![rat]);
        let real_add = VdInstantiation::new(db, ring_add.into(), smallvec![real]);
        let complex_add = VdInstantiation::new(db, ring_add.into(), smallvec![complex]);
        let nat_eq = VdInstantiation::new(db, ring_add.into(), smallvec![nat]);
        let int_eq = VdInstantiation::new(db, ring_add.into(), smallvec![int]);
        let rat_eq = VdInstantiation::new(db, ring_add.into(), smallvec![rat]);
        let real_eq = VdInstantiation::new(db, ring_add.into(), smallvec![real]);
        let complex_eq = VdInstantiation::new(db, ring_add.into(), smallvec![complex]);
        Self {
            nat_add,
            int_add,
            rat_add,
            real_add,
            complex_add,
            nat_eq,
            int_eq,
            rat_eq,
            real_eq,
            complex_eq,
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_zfc_instantiation_menu(db: &::salsa::Db) -> VdZfcInstantiationMenu {
    VdZfcInstantiationMenu::new(db)
}
