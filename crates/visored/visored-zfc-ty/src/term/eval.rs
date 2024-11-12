use super::*;

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdZfcEval(VdZfcTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdZfcEvalData {
    // Add appropriate fields here
}

impl VdZfcEval {
    pub fn data(self, db: &::salsa::Db) -> &VdZfcEvalData {
        match self.0.data(db) {
            VdZfcTermData::Eval(data) => data,
            _ => unreachable!(),
        }
    }
}
