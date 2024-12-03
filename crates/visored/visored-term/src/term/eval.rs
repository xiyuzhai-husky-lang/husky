use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdEval(VdTermId);

impl std::ops::Deref for VdEval {
    type Target = VdTermId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdEvalData {
    // Add appropriate fields here
}

impl VdEval {
    pub fn data(self) -> &'static VdEvalData {
        match self.0.data() {
            VdTermData::Eval(data) => data,
            _ => unreachable!(),
        }
    }
}
