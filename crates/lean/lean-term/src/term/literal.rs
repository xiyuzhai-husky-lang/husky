use eterned::db::EternerDb;

use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LnLiteral(LnTermId);

impl std::fmt::Debug for LnLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // write!(f, "LnLiteral({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LnLiteralData {
    Nat(String),
    Float(String),
}

impl LnLiteral {
    pub fn new(data: LnLiteralData, db: &EternerDb) -> Self {
        Self(LnTermId::new(data.into(), db))
    }

    pub(crate) fn show(&self, db: &EternerDb) -> String {
        todo!()
    }
}

impl LnLiteral {
    pub fn data(self, db: &EternerDb) -> &LnLiteralData {
        match self.0.data(db) {
            LnTermData::Literal(data) => data,
            _ => unreachable!(),
        }
    }
}

impl LnLiteralData {
    pub fn str(&self) -> &str {
        match self {
            LnLiteralData::Nat(s) => s,
            LnLiteralData::Float(s) => s,
        }
    }
}

impl AsRef<str> for LnLiteralData {
    fn as_ref(&self) -> &str {
        self.str()
    }
}

impl std::fmt::Display for LnLiteralData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())
    }
}
