use smallvec::SmallVec;
use visored_coword::namae::VdNamae;

pub struct ZfsType {
    data: ZfsTypeData,
    refinements: SmallVec<[(); 2]>,
}

pub enum ZfsTypeData {
    Named(VdNamae), // TODO: do we need a path here?
}
