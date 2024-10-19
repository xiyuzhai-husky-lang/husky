use smallvec::SmallVec;
use visored_coword::namae::VisoredNamae;

pub struct ZfsType {
    data: ZfsTypeData,
    refinements: SmallVec<[(); 2]>,
}

pub enum ZfsTypeData {
    Named(VisoredNamae), // TODO: do we need a path here?
}
