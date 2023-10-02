mod ascension;
pub mod helpers;
pub mod linkage;
pub mod visual;

pub use self::ascension::*;

use self::linkage::*;
use husky_linkage_path::LinkagePath;
use husky_vfs::CratePath;
use small_cell_stack::SmallCellStack;
use std::thread::LocalKey;

pub trait IsTask {
    type DevAscension: IsDevAscension;
}
