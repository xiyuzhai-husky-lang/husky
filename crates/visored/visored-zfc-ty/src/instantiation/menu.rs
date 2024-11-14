use super::*;
use crate::menu::{vd_zfc_ty_menu, VdZfcTypeMenu};
use smallvec::{smallvec, SmallVec};
use visored_item_path::menu::{vd_item_path_menu, VdItemPathMenu};

#[derive(Debug, PartialEq, Eq)]
pub struct VdZfcInstantiationMenu {
    /// ## add
    pub nat_add_instantiation: VdInstantiation,
    pub int_add_instantiation: VdInstantiation,
    pub rat_add_instantiation: VdInstantiation,
    pub real_add_instantiation: VdInstantiation,
    /// ## eq
    pub nat_eq_instantiation: VdInstantiation,
    pub int_eq_instantiation: VdInstantiation,
    pub rat_eq_instantiation: VdInstantiation,
    pub real_eq_instantiation: VdInstantiation,
}

impl VdZfcInstantiationMenu {
    pub fn new(db: &salsa::Db) -> Self {
        let VdItemPathMenu {
            set,
            proposition,
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
        let nat_add_instantiation = VdInstantiation::new(db, ring_add.into(), smallvec![nat_term]);
        let int_add_instantiation = VdInstantiation::new(db, ring_add.into(), smallvec![int_term]);
        let rat_add_instantiation = VdInstantiation::new(db, ring_add.into(), smallvec![rat_term]);
        let real_add_instantiation =
            VdInstantiation::new(db, ring_add.into(), smallvec![real_term]);
        let nat_eq_instantiation = VdInstantiation::new(db, ring_add.into(), smallvec![nat_term]);
        let int_eq_instantiation = VdInstantiation::new(db, ring_add.into(), smallvec![int_term]);
        let rat_eq_instantiation = VdInstantiation::new(db, ring_add.into(), smallvec![rat_term]);
        let real_eq_instantiation = VdInstantiation::new(db, ring_add.into(), smallvec![real_term]);
        Self {
            nat_add_instantiation,
            int_add_instantiation,
            rat_add_instantiation,
            real_add_instantiation,
            nat_eq_instantiation,
            int_eq_instantiation,
            rat_eq_instantiation,
            real_eq_instantiation,
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_zfc_instantiation_menu(db: &::salsa::Db) -> VdZfcInstantiationMenu {
    VdZfcInstantiationMenu::new(db)
}
