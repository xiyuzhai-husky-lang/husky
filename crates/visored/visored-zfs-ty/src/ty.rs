use smallvec::SmallVec;
use visored_coword::namae::VdNamae;

pub struct VdZfsType {
    data: VdZfsTypeData,
    refinements: SmallVec<[(); 2]>,
}

pub enum VdZfsTypeData {
    Named(VdNamae), // TODO: do we need a path here?
}
