use eterned::db::EternerDb;

use super::{VdTerm, VdTermData, VdTermId, ZfcTerms};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdAbstraction(VdTermId);

impl std::ops::Deref for VdAbstraction {
    type Target = VdTermId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdAbstractionData {
    pub parameters: VdTerm,
    pub arguments: ZfcTerms,
}

impl VdAbstraction {
    pub fn data(self) -> &'static VdAbstractionData {
        match self.0.data() {
            VdTermData::Abstraction(data) => data,
            _ => unreachable!(),
        }
    }
}
