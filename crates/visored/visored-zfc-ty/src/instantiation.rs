use crate::term::VdZfcTerm;
use smallvec::SmallVec;
use visored_item_path::path::VdItemPath;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdInstantiation {
    path: VdItemPath,
    arguments: SmallVec<[VdZfcTerm; 4]>,
}

impl VdInstantiation {
    pub(crate) fn new(path: VdItemPath, arguments: SmallVec<[VdZfcTerm; 4]>) -> Self {
        Self { path, arguments }
    }
}

impl VdInstantiation {
    pub fn path(&self) -> VdItemPath {
        self.path
    }

    pub fn arguments(&self) -> &SmallVec<[VdZfcTerm; 4]> {
        &self.arguments
    }
}
