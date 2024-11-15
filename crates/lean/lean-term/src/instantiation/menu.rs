use crate::menu::{ln_term_menu, LnTermMenu};
use lean_item_path::menu::{ln_item_path_menu, LnItemPathMenu};
use smallvec::*;

use super::LnInstantiation;

#[derive(Debug, PartialEq, Eq)]
pub struct LnInstantiationMenu {
    pub nat_add: LnInstantiation,
    pub int_add: LnInstantiation,
    pub rat_add: LnInstantiation,
    pub real_add: LnInstantiation,
    pub complex_add: LnInstantiation,
    pub nat_eq: LnInstantiation,
    pub int_eq: LnInstantiation,
    pub rat_eq: LnInstantiation,
    pub real_eq: LnInstantiation,
    pub complex_eq: LnInstantiation,
}

impl LnInstantiationMenu {
    pub fn new(db: &::salsa::Db) -> Self {
        let LnItemPathMenu { ring_add, eq, .. } = *ln_item_path_menu(db);
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
            nat_add: t(ring_add, smallvec![nat]),
            int_add: t(ring_add, smallvec![int]),
            rat_add: t(ring_add, smallvec![rat]),
            real_add: t(ring_add, smallvec![real]),
            complex_add: t(ring_add, smallvec![complex]),
            nat_eq: t(eq, smallvec![nat]),
            int_eq: t(eq, smallvec![int]),
            rat_eq: t(eq, smallvec![rat]),
            real_eq: t(eq, smallvec![real]),
            complex_eq: t(eq, smallvec![complex]),
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn ln_instantiation_menu(db: &::salsa::Db) -> LnInstantiationMenu {
    LnInstantiationMenu::new(db)
}
