pub mod special_constant;

use eterned::db::EternerDb;

use self::special_constant::VdSpecialConstant;
use super::*;
use crate::{menu::vd_ty_menu, ty::VdType};

// #[salsa::derive_debug_with_db]
// #[salsa::as_id]
// #[salsa::deref_id]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdLiteral(VdTermId);

impl std::ops::Deref for VdLiteral {
    type Target = VdTermId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for VdLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VdLiteralData {
    NaturalNumber(String),
    NegativeInteger(String),
    Float(String),
    SpecialConstant(VdSpecialConstant),
}

impl VdLiteral {
    pub fn new(data: VdLiteralData, db: &EternerDb) -> Self {
        #[cfg(test)]
        {
            match data {
                VdLiteralData::NaturalNumber(ref n) => {
                    debug_assert!(!n.is_empty());
                    debug_assert!(n.chars().all(|c| c.is_digit(10)));
                }
                VdLiteralData::NegativeInteger(_) => todo!(),
                VdLiteralData::Float(_) => todo!(),
                VdLiteralData::SpecialConstant(vd_special_constant) => todo!(),
            }
        }
        Self(VdTermId::new(data.into(), db))
    }

    pub fn data(self, db: &EternerDb) -> &VdLiteralData {
        match self.0.data(db) {
            VdTermData::Literal(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn ty(self, db: &EternerDb) -> VdType {
        zfc_literal_ty(self, db)
    }
}

fn zfc_literal_ty(literal: VdLiteral, db: &EternerDb) -> VdType {
    let data = literal.data(db);
    let menu = vd_ty_menu(db);
    match data {
        VdLiteralData::NaturalNumber(_) => menu.nat,
        VdLiteralData::NegativeInteger(_) => todo!(),
        VdLiteralData::Float(_) => menu.rat,
        VdLiteralData::SpecialConstant(special_constant) => todo!(),
    }
}

impl VdLiteralData {
    pub fn as_str(&self) -> &str {
        match self {
            VdLiteralData::NaturalNumber(n) => n,
            VdLiteralData::NegativeInteger(n) => n,
            VdLiteralData::Float(n) => n,
            VdLiteralData::SpecialConstant(_) => {
                todo!()
            }
        }
    }
}
