use eterned::db::EternerDb;

use super::{VdTerm, VdTermData, VdTermId, ZfcTerms};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdAbstraction(VdTermId);

impl std::ops::Deref for VdAbstraction {
    type Target = VdTermId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdAbstractionData {
    pub parameters: VdTerm,
    pub arguments: ZfcTerms,
}

impl VdAbstraction {
    pub fn data(&self, db: &EternerDb) -> &VdAbstractionData {
        match self.0.data(db) {
            VdTermData::Abstraction(data) => data,
            _ => unreachable!(),
        }
    }
}
