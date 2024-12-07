use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdEval(VdTermId);

impl std::ops::Deref for VdEval {
    type Target = VdTermId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdEvalData {
    // Add appropriate fields here
}

impl VdEval {
    pub fn data(&self, db: &EternerDb) -> &VdEvalData {
        match self.0.data(db) {
            VdTermData::Eval(data) => data,
            _ => unreachable!(),
        }
    }
}
