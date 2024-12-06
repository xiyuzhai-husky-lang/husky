use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdSymbolicVariable(VdTermId);

impl std::ops::Deref for VdSymbolicVariable {
    type Target = VdTermId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdSymbolicVariableData {
    // Add appropriate fields here
}
