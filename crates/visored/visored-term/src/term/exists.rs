use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdExists(VdTermId);

impl std::ops::Deref for VdExists {
    type Target = VdTermId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdExistsData {
    // Add appropriate fields here
}

impl VdExists {
    pub fn data(&self, db: &EternerDb) -> &VdExistsData {
        match self.0.data(db) {
            VdTermData::Exists(data) => data,
            _ => unreachable!(),
        }
    }
}
