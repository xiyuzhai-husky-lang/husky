pub mod special_constant;

use self::special_constant::VdZfcSpecialConstant;
use super::*;

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfcLiteral(VdZfcTermId);

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdZfcLiteralData {
    NaturalNumber(String),
    NegativeInteger(String),
    FiniteDecimalRepresentation(String),
    SpecialConstant(VdZfcSpecialConstant),
}

impl VdZfcLiteral {
    pub fn data(self, db: &::salsa::Db) -> &VdZfcLiteralData {
        match self.0.data(db) {
            VdZfcTermData::Literal(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn new(data: VdZfcLiteralData, db: &::salsa::Db) -> Self {
        Self(VdZfcTermId::new(db, data.into()))
    }
}
