use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdLimit(VdTermId);

impl std::ops::Deref for VdLimit {
    type Target = VdTermId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdLimitData {
    // Add appropriate fields here
}

impl VdLimit {
    pub fn data(self) -> &'static VdLimitData {
        match self.0.data() {
            VdTermData::Limit(data) => data,
            _ => unreachable!(),
        }
    }
}
