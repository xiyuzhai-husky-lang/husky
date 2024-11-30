use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdAbstractVariable(VdTermId);

impl std::ops::Deref for VdAbstractVariable {
    type Target = VdTermId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdAbstractVariableData {
    // Add appropriate fields here
}

impl VdAbstractVariable {
    pub fn data(&self, db: &InternerDb) -> &VdAbstractVariableData {
        match self.0.data(db) {
            VdTermData::AbstractVariable(data) => data,
            _ => unreachable!(),
        }
    }
}
