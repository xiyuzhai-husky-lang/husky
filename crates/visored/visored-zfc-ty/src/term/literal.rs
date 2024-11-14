pub mod special_constant;

use self::special_constant::VdZfcSpecialConstant;
use super::*;
use crate::{menu::vd_zfc_ty_menu, ty::VdZfcType};

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdZfcLiteral(VdZfcTermId);

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VdZfcLiteralData {
    NaturalNumber(String),
    NegativeInteger(String),
    FiniteDecimalRepresentation(String),
    SpecialConstant(VdZfcSpecialConstant),
}

impl VdZfcLiteral {
    pub fn new(data: VdZfcLiteralData, db: &::salsa::Db) -> Self {
        #[cfg(test)]
        {
            match data {
                VdZfcLiteralData::NaturalNumber(ref n) => {
                    debug_assert!(!n.is_empty());
                    debug_assert!(n.chars().all(|c| c.is_digit(10)));
                }
                VdZfcLiteralData::NegativeInteger(_) => todo!(),
                VdZfcLiteralData::FiniteDecimalRepresentation(_) => todo!(),
                VdZfcLiteralData::SpecialConstant(vd_zfc_special_constant) => todo!(),
            }
        }
        Self(VdZfcTermId::new(db, data.into()))
    }

    pub fn data(self, db: &::salsa::Db) -> &VdZfcLiteralData {
        match self.0.data(db) {
            VdZfcTermData::Literal(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn ty(self, db: &::salsa::Db) -> VdZfcType {
        zfc_literal_ty(db, self)
    }
}

#[salsa::tracked]
fn zfc_literal_ty(db: &::salsa::Db, literal: VdZfcLiteral) -> VdZfcType {
    let data = literal.data(db);
    let menu = vd_zfc_ty_menu(db);
    match data {
        VdZfcLiteralData::NaturalNumber(_) => menu.nat,
        VdZfcLiteralData::NegativeInteger(_) => todo!(),
        VdZfcLiteralData::FiniteDecimalRepresentation(_) => todo!(),
        VdZfcLiteralData::SpecialConstant(special_constant) => todo!(),
    }
}

impl VdZfcLiteralData {
    pub fn as_str(&self) -> &str {
        match self {
            VdZfcLiteralData::NaturalNumber(n) => n,
            VdZfcLiteralData::NegativeInteger(n) => n,
            VdZfcLiteralData::FiniteDecimalRepresentation(n) => n,
            VdZfcLiteralData::SpecialConstant(_) => {
                todo!()
            }
        }
    }
}
