use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsEval(ZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsEvalData {
    // Add appropriate fields here
}

impl ZfsEval {
    pub fn data(self, db: &::salsa::Db) -> ZfsEvalData {
        match self.0.data(db) {
            ZfsTermData::Eval(data) => data,
            _ => unreachable!(),
        }
    }
}
