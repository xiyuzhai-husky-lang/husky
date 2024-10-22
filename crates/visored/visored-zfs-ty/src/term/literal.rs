pub mod special_constant;

use self::special_constant::VdZfsSpecialConstant;
use super::*;

#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsLiteral(VdZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdZfsLiteralData {
    NaturalNumber(String),
    NegativeInteger(String),
    FiniteDecimalRepresentation(String),
    SpecialConstant(VdZfsSpecialConstant),
}

impl VdZfsLiteral {
    pub fn data(self, db: &::salsa::Db) -> &VdZfsLiteralData {
        match self.0.data(db) {
            VdZfsTermData::Literal(data) => data,
            _ => unreachable!(),
        }
    }

    pub(crate) fn new(data: VdZfsLiteralData, db: &::salsa::Db) -> Self {
        Self(VdZfsTermId::new(db, data.into()))
    }
}
