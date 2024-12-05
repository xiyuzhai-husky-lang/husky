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
    Int128(i128),
    Float(String),
    SpecialConstant(VdSpecialConstant),
}

impl std::fmt::Display for VdLiteralData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VdLiteralData::Int128(n) => write!(f, "{}", n),
            VdLiteralData::Float(n) => write!(f, "{}", n),
            VdLiteralData::SpecialConstant(n) => todo!(),
        }
    }
}

impl VdLiteral {
    pub fn new(data: VdLiteralData, db: &EternerDb) -> Self {
        Self(VdTermId::new(data.into(), db))
    }

    pub fn data(self) -> &'static VdLiteralData {
        match self.0.data() {
            VdTermData::Literal(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn ty(self, db: &EternerDb) -> VdType {
        zfc_literal_ty(self, db)
    }
}

fn zfc_literal_ty(literal: VdLiteral, db: &EternerDb) -> VdType {
    let data = literal.data();
    let menu = vd_ty_menu(db);
    match data {
        VdLiteralData::Int128(_) => menu.int,
        VdLiteralData::Float(_) => menu.rat,
        VdLiteralData::SpecialConstant(special_constant) => todo!(),
    }
}
