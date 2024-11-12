use smallvec::SmallVec;
use visored_coword::namae::VdNamae;
use visored_item_path::path::VdItemPath;

#[salsa::interned]
pub struct VdZfcType {
    data: VdZfcTypeData,
    refinements: SmallVec<[(); 2]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdZfcTypeData {
    ItemPath(VdItemPath), // TODO: do we need a path here?
}
