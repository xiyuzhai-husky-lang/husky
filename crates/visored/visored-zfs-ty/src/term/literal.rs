use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsLiteral(ZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ZfsLiteralData {
    Unit,
    // ...
}

impl ZfsLiteral {
    pub fn data(self, db: &::salsa::Db) -> ZfsLiteralData {
        match self.0.data(db) {
            ZfsTermData::Literal(data) => data,
            _ => unreachable!(),
        }
    }
}
