use super::*;
use crate::{
    menu::{vd_ty_menu, VdTypeMenu},
    term::menu::{vd_term_menu, VdTermMenu},
};
use smallvec::{smallvec, SmallVec};
use visored_item_path::menu::{vd_item_path_menu, VdItemPathMenu};

#[derive(Debug, PartialEq, Eq)]
pub struct VdInstantiationMenu {
    /// # binary operators
    /// ## sub
    pub int_sub: VdInstantiation,
    pub rat_sub: VdInstantiation,
    pub real_sub: VdInstantiation,
    pub complex_sub: VdInstantiation,
    /// # separators
    /// ## add
    pub nat_add: VdInstantiation,
    pub int_add: VdInstantiation,
    pub rat_add: VdInstantiation,
    pub real_add: VdInstantiation,
    pub complex_add: VdInstantiation,
    /// ## mul
    pub nat_mul: VdInstantiation,
    pub int_mul: VdInstantiation,
    pub rat_mul: VdInstantiation,
    pub real_mul: VdInstantiation,
    pub complex_mul: VdInstantiation,
    /// ## power
    pub nat_to_the_power_of_nat: VdInstantiation,
    pub int_to_the_power_of_nat: VdInstantiation,
    pub rat_to_the_power_of_nat: VdInstantiation,
    pub real_to_the_power_of_nat: VdInstantiation,
    pub complex_to_the_power_of_nat: VdInstantiation,
    /// ## eq
    pub nat_eq: VdInstantiation,
    pub int_eq: VdInstantiation,
    pub rat_eq: VdInstantiation,
    pub real_eq: VdInstantiation,
    pub complex_eq: VdInstantiation,
    /// ## le
    pub nat_le: VdInstantiation,
    pub int_le: VdInstantiation,
    pub rat_le: VdInstantiation,
    pub real_le: VdInstantiation,
    /// ## ge
    pub nat_ge: VdInstantiation,
    pub int_ge: VdInstantiation,
    pub rat_ge: VdInstantiation,
    pub real_ge: VdInstantiation,
}

impl VdInstantiationMenu {
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
            ring_power,
            eq,
            le,
            ge,
        } = *vd_item_path_menu(db);
        let VdTermMenu {
            zero,
            one,
            two,
            nat,
            int,
            rat,
            real,
            complex,
        } = *vd_term_menu(db);
        let int_sub = VdInstantiation::new(db, ring_add.into(), smallvec![int, int]);
        let rat_sub = VdInstantiation::new(db, ring_add.into(), smallvec![rat, rat]);
        let real_sub = VdInstantiation::new(db, ring_add.into(), smallvec![real, real]);
        let complex_sub = VdInstantiation::new(db, ring_add.into(), smallvec![complex, complex]);
        let nat_add = VdInstantiation::new(db, ring_add.into(), smallvec![nat]);
        let int_add = VdInstantiation::new(db, ring_add.into(), smallvec![int]);
        let rat_add = VdInstantiation::new(db, ring_add.into(), smallvec![rat]);
        let real_add = VdInstantiation::new(db, ring_add.into(), smallvec![real]);
        let complex_add = VdInstantiation::new(db, ring_add.into(), smallvec![complex]);
        let nat_mul = VdInstantiation::new(db, ring_mul.into(), smallvec![nat]);
        let int_mul = VdInstantiation::new(db, ring_mul.into(), smallvec![int]);
        let rat_mul = VdInstantiation::new(db, ring_mul.into(), smallvec![rat]);
        let real_mul = VdInstantiation::new(db, ring_mul.into(), smallvec![real]);
        let complex_mul = VdInstantiation::new(db, ring_mul.into(), smallvec![complex]);
        let nat_to_the_power_of_nat =
            VdInstantiation::new(db, ring_power.into(), smallvec![nat, nat]);
        let int_to_the_power_of_nat =
            VdInstantiation::new(db, ring_power.into(), smallvec![int, nat]);
        let rat_to_the_power_of_nat =
            VdInstantiation::new(db, ring_power.into(), smallvec![rat, nat]);
        let real_to_the_power_of_nat =
            VdInstantiation::new(db, ring_power.into(), smallvec![real, nat]);
        let complex_to_the_power_of_nat =
            VdInstantiation::new(db, ring_power.into(), smallvec![complex, nat]);
        let nat_eq = VdInstantiation::new(db, eq.into(), smallvec![nat]);
        let int_eq = VdInstantiation::new(db, eq.into(), smallvec![int]);
        let rat_eq = VdInstantiation::new(db, eq.into(), smallvec![rat]);
        let real_eq = VdInstantiation::new(db, eq.into(), smallvec![real]);
        let complex_eq = VdInstantiation::new(db, eq.into(), smallvec![complex]);
        let nat_le = VdInstantiation::new(db, le.into(), smallvec![nat]);
        let int_le = VdInstantiation::new(db, le.into(), smallvec![int]);
        let rat_le = VdInstantiation::new(db, le.into(), smallvec![rat]);
        let real_le = VdInstantiation::new(db, le.into(), smallvec![real]);
        let nat_ge = VdInstantiation::new(db, ge.into(), smallvec![nat]);
        let int_ge = VdInstantiation::new(db, ge.into(), smallvec![int]);
        let rat_ge = VdInstantiation::new(db, ge.into(), smallvec![rat]);
        let real_ge = VdInstantiation::new(db, ge.into(), smallvec![real]);
        Self {
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
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_instantiation_menu(db: &::salsa::Db) -> VdInstantiationMenu {
    VdInstantiationMenu::new(db)
}
