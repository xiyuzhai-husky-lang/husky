use super::*;
use visored_item_path::menu::{vd_item_path_menu, VdItemPathMenu};

#[derive(Debug, PartialEq, Eq)]
pub struct VdZfcTermMenu {
    /// ## literals
    pub zero: VdZfcLiteral,
    pub one: VdZfcLiteral,
    pub two: VdZfcLiteral,
    /// ## types as terms
    pub nat: VdZfcTerm,
    pub int: VdZfcTerm,
    pub rat: VdZfcTerm,
    pub real: VdZfcTerm,
    pub complex: VdZfcTerm,
}

impl VdZfcTermMenu {
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

        let zero = VdZfcLiteral::new(VdZfcLiteralData::NaturalNumber("0".to_string()), db);
        let one = VdZfcLiteral::new(VdZfcLiteralData::NaturalNumber("1".to_string()), db);
        let two = VdZfcLiteral::new(VdZfcLiteralData::NaturalNumber("2".to_string()), db);
        let nat = VdZfcTerm::new_item_path(nat.into(), db);
        let int = VdZfcTerm::new_item_path(int.into(), db);
        let rat = VdZfcTerm::new_item_path(rat.into(), db);
        let real = VdZfcTerm::new_item_path(real.into(), db);
        let complex = VdZfcTerm::new_item_path(complex.into(), db);
        Self {
            zero,
            one,
            two,
            nat,
            int,
            rat,
            real,
            complex,
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_zfc_term_menu(db: &::salsa::Db) -> VdZfcTermMenu {
    VdZfcTermMenu::new(db)
}
