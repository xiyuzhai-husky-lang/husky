pub mod special_constant;

use self::special_constant::VdSpecialConstant;
use super::*;
use crate::{menu::vd_ty_menu, ty::VdType};

// #[salsa::derive_debug_with_db]
// #[salsa::as_id]
// #[salsa::deref_id]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VdLiteralData {
    NaturalNumber(String),
    NegativeInteger(String),
    FiniteDecimalRepresentation(String),
    SpecialConstant(VdSpecialConstant),
}

impl VdLiteral {
    pub fn new(data: VdLiteralData) -> Self {
        #[cfg(test)]
        {
            match data {
                VdLiteralData::NaturalNumber(ref n) => {
                    debug_assert!(!n.is_empty());
                    debug_assert!(n.chars().all(|c| c.is_digit(10)));
                }
                VdLiteralData::NegativeInteger(_) => todo!(),
                VdLiteralData::FiniteDecimalRepresentation(_) => todo!(),
                VdLiteralData::SpecialConstant(vd_special_constant) => todo!(),
            }
        }
        Self(VdTermId::new(data.into()))
    }

    pub fn data(self) -> &'static VdLiteralData {
        match self.0.data() {
            VdTermData::Literal(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn ty(self) -> VdType {
        zfc_literal_ty(self)
    }
}

fn zfc_literal_ty(literal: VdLiteral) -> VdType {
    todo!()
    // let data = literal.data();
    // let menu = vd_ty_menu();
    // match data {
    //     VdLiteralData::NaturalNumber(_) => menu.nat,
    //     VdLiteralData::NegativeInteger(_) => todo!(),
    //     VdLiteralData::FiniteDecimalRepresentation(_) => todo!(),
    //     VdLiteralData::SpecialConstant(special_constant) => todo!(),
    // }
}

impl VdLiteralData {
    pub fn as_str(&self) -> &str {
        match self {
            VdLiteralData::NaturalNumber(n) => n,
            VdLiteralData::NegativeInteger(n) => n,
            VdLiteralData::FiniteDecimalRepresentation(n) => n,
            VdLiteralData::SpecialConstant(_) => {
                todo!()
            }
        }
    }
}
