use crate::menu::{ln_term_menu, LnTermMenu};
use lean_item_path::menu::{ln_item_path_menu, LnItemPathMenu};
use smallvec::*;

use super::LnInstantiation;

#[derive(Debug, PartialEq, Eq)]
pub struct LnInstantiationMenu {
    pub int_sub: LnInstantiation,
    pub rat_sub: LnInstantiation,
    pub real_sub: LnInstantiation,
    pub complex_sub: LnInstantiation,
    pub nat_add: LnInstantiation,
    pub int_add: LnInstantiation,
    pub rat_add: LnInstantiation,
    pub real_add: LnInstantiation,
    pub complex_add: LnInstantiation,
    pub nat_mul: LnInstantiation,
    pub int_mul: LnInstantiation,
    pub rat_mul: LnInstantiation,
    pub real_mul: LnInstantiation,
    pub complex_mul: LnInstantiation,
    pub nat_to_the_power_of_nat: LnInstantiation,
    pub int_to_the_power_of_nat: LnInstantiation,
    pub rat_to_the_power_of_nat: LnInstantiation,
    pub real_to_the_power_of_nat: LnInstantiation,
    pub complex_to_the_power_of_nat: LnInstantiation,
    pub nat_eq: LnInstantiation,
    pub int_eq: LnInstantiation,
    pub rat_eq: LnInstantiation,
    pub real_eq: LnInstantiation,
    pub complex_eq: LnInstantiation,
    pub nat_le: LnInstantiation,
    pub int_le: LnInstantiation,
    pub rat_le: LnInstantiation,
    pub real_le: LnInstantiation,
    pub nat_ge: LnInstantiation,
    pub int_ge: LnInstantiation,
    pub rat_ge: LnInstantiation,
    pub real_ge: LnInstantiation,
}

impl LnInstantiationMenu {
    pub fn new(db: &::salsa::Db) -> Self {
        let LnItemPathMenu {
            ring_add,
            ring_mul,
            eq,
            le,
            ge,
            ..
        } = *ln_item_path_menu(db);
        let LnTermMenu {
            nat,
            int,
            rat,
            real,
            complex,
            ..
        } = *ln_term_menu(db);
        let t = |path, arguments| LnInstantiation::new(db, path, arguments);
        Self {
            int_sub: t(ring_add, smallvec![int, int]),
            rat_sub: t(ring_add, smallvec![rat, rat]),
            real_sub: t(ring_add, smallvec![real, real]),
            complex_sub: t(ring_add, smallvec![complex, complex]),
            nat_add: t(ring_add, smallvec![nat]),
            int_add: t(ring_add, smallvec![int]),
            rat_add: t(ring_add, smallvec![rat]),
            real_add: t(ring_add, smallvec![real]),
            complex_add: t(ring_add, smallvec![complex]),
            nat_mul: t(ring_mul, smallvec![nat]),
            int_mul: t(ring_mul, smallvec![int]),
            rat_mul: t(ring_mul, smallvec![rat]),
            real_mul: t(ring_mul, smallvec![real]),
            complex_mul: t(ring_mul, smallvec![complex]),
            nat_to_the_power_of_nat: t(ring_mul, smallvec![nat, nat]),
            int_to_the_power_of_nat: t(ring_mul, smallvec![int, nat]),
            rat_to_the_power_of_nat: t(ring_mul, smallvec![rat, nat]),
            real_to_the_power_of_nat: t(ring_mul, smallvec![real, nat]),
            complex_to_the_power_of_nat: t(ring_mul, smallvec![complex, nat]),
            nat_eq: t(eq, smallvec![nat]),
            int_eq: t(eq, smallvec![int]),
            rat_eq: t(eq, smallvec![rat]),
            real_eq: t(eq, smallvec![real]),
            complex_eq: t(eq, smallvec![complex]),
            nat_le: t(le, smallvec![nat]),
            int_le: t(le, smallvec![int]),
            rat_le: t(le, smallvec![rat]),
            real_le: t(le, smallvec![real]),
            nat_ge: t(ge, smallvec![nat]),
            int_ge: t(ge, smallvec![int]),
            rat_ge: t(ge, smallvec![rat]),
            real_ge: t(ge, smallvec![real]),
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn ln_instantiation_menu(db: &::salsa::Db) -> LnInstantiationMenu {
    LnInstantiationMenu::new(db)
}
