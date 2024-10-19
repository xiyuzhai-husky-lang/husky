use smallvec::SmallVec;
use visored_coword::namae::VisoredNamae;

pub struct VisoredZfsType {
    data: VisoredZfsTypeData,
    refinements: SmallVec<[(); 2]>,
}

pub enum VisoredZfsTypeData {
    Named(VisoredNamae), // TODO: do we need a path here?
}
