use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsEval(VdZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsEvalData {
    // Add appropriate fields here
}

impl VdZfsEval {
    pub fn data(self, db: &::salsa::Db) -> VdZfsEvalData {
        match self.0.data(db) {
            VdZfsTermData::Eval(data) => data,
            _ => unreachable!(),
        }
    }
}
