use crate::{
    term::literal::{VdZfcLiteral, VdZfcLiteralData},
    ty::{VdZfcType, VdZfcTypeData},
};
use smallvec::{smallvec, SmallVec};
use visored_item_path::path::VdItemPath;

#[derive(Debug, PartialEq, Eq)]
pub struct VdZfcTypeMenu {
    pub zero_literal: VdZfcLiteral,
    pub one_literal: VdZfcLiteral,
    pub two_literal: VdZfcLiteral,
    /// natural numbers as a type
    pub nat_ty: VdZfcType,
    /// integers as a type
    pub int_ty: VdZfcType,
    /// rational numbers as a type
    pub rat_ty: VdZfcType,
    /// real numbers as a type
    pub real_ty: VdZfcType,
    /// the category of sets as a type
    pub set_ty: VdZfcType,
    /// the category of propositions as a type
    pub prop_ty: VdZfcType,
}

impl VdZfcTypeMenu {
    fn new(db: &::salsa::Db) -> Self {
        Self {
            zero_literal: VdZfcLiteral::new(VdZfcLiteralData::NaturalNumber("0".to_string()), db),
            one_literal: VdZfcLiteral::new(VdZfcLiteralData::NaturalNumber("1".to_string()), db),
            two_literal: VdZfcLiteral::new(VdZfcLiteralData::NaturalNumber("2".to_string()), db),
            nat_ty: VdZfcType::new(
                db,
                VdZfcTypeData::ItemPath(VdItemPath::NATURAL_NUMBER),
                smallvec![],
            ),
            int_ty: VdZfcType::new(
                db,
                VdZfcTypeData::ItemPath(VdItemPath::INTEGER),
                smallvec![],
            ),
            rat_ty: VdZfcType::new(
                db,
                VdZfcTypeData::ItemPath(VdItemPath::RATIONAL_NUMBER),
                smallvec![],
            ),
            real_ty: VdZfcType::new(
                db,
                VdZfcTypeData::ItemPath(VdItemPath::REAL_NUMBER),
                smallvec![],
            ),
            set_ty: VdZfcType::new(db, VdZfcTypeData::ItemPath(VdItemPath::SET), smallvec![]),
            prop_ty: VdZfcType::new(
                db,
                VdZfcTypeData::ItemPath(VdItemPath::PROPOSITION),
                smallvec![],
            ),
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
