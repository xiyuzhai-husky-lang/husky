use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsLiteral(VdZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdZfsLiteralData {
    Unit,
    // ...
}

impl VdZfsLiteral {
    pub fn data(self, db: &::salsa::Db) -> VdZfsLiteralData {
        match self.0.data(db) {
            VdZfsTermData::Literal(data) => data,
            _ => unreachable!(),
        }
    }
}
