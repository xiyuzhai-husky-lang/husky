pub mod menu;

use crate::term::VdZfcTerm;
use smallvec::SmallVec;
use visored_item_path::path::VdItemPath;

#[salsa::interned]
pub struct VdInstantiation {
    pub path: VdItemPath,
    #[return_ref]
    pub arguments: SmallVec<[VdZfcTerm; 4]>,
}
