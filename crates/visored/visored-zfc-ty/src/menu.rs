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
    /// # terms
    /// ## literals
    pub zero_literal: VdZfcLiteral,
    pub one_literal: VdZfcLiteral,
    pub two_literal: VdZfcLiteral,
    /// ## types as terms
    pub nat_term: VdZfcTerm,
    pub int_term: VdZfcTerm,
    pub rat_term: VdZfcTerm,
    pub real_term: VdZfcTerm,
    /// # types
    /// natural numbers as a type
    pub nat_ty: VdZfcType,
    /// integers as a type
    pub int_ty: VdZfcType,
    /// rational numbers as a type
    pub rat_ty: VdZfcType,
    /// real numbers as a type
    pub real_ty: VdZfcType,
    /// - the category of sets as a type
    pub set_ty: VdZfcType,
    /// - the category of propositions as a type
    pub prop_ty: VdZfcType,
}

impl VdZfcTypeMenu {
    fn new(db: &::salsa::Db) -> Self {
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

        let zero_literal = VdZfcLiteral::new(VdZfcLiteralData::NaturalNumber("0".to_string()), db);
        let one_literal = VdZfcLiteral::new(VdZfcLiteralData::NaturalNumber("1".to_string()), db);
        let two_literal = VdZfcLiteral::new(VdZfcLiteralData::NaturalNumber("2".to_string()), db);
        let nat_term = VdZfcTerm::new_item_path(nat.into(), db);
        let int_term = VdZfcTerm::new_item_path(int.into(), db);
        let rat_term = VdZfcTerm::new_item_path(rat.into(), db);
        let real_term = VdZfcTerm::new_item_path(real.into(), db);
        let nat_ty = VdZfcType::new_item_path(nat.into(), db);
        let int_ty = VdZfcType::new_item_path(int.into(), db);
        let rat_ty = VdZfcType::new_item_path(rat.into(), db);
        let real_ty = VdZfcType::new_item_path(real.into(), db);
        let set_ty = VdZfcType::new_item_path(set.into(), db);
        let prop_ty = VdZfcType::new_item_path(proposition.into(), db);

        Self {
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
        }
    }
}

impl VdZfcTypeMenu {
    pub fn zero_literal(&self) -> VdZfcLiteral {
        self.zero_literal
    }

    pub fn one_literal(&self) -> VdZfcLiteral {
        self.one_literal
    }

    pub fn two_literal(&self) -> VdZfcLiteral {
        self.two_literal
    }

    pub fn natural_number_ty(&self) -> VdZfcType {
        self.nat_ty
    }

    pub fn proposition_ty(&self) -> VdZfcType {
        self.prop_ty
    }

    pub fn set_category_ty(&self) -> VdZfcType {
        self.set_ty
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_zfc_ty_menu(db: &::salsa::Db) -> VdZfcTypeMenu {
    VdZfcTypeMenu::new(db)
}
