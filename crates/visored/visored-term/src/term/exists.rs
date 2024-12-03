use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdExists(VdTermId);

impl std::ops::Deref for VdExists {
    type Target = VdTermId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdExistsData {
    // Add appropriate fields here
}

impl VdExists {
    pub fn data(self) -> &'static VdExistsData {
        match self.0.data() {
            VdTermData::Exists(data) => data,
            _ => unreachable!(),
        }
    }
}
