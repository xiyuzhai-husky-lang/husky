#![feature(associated_type_bounds)]
pub mod ascension;
pub mod helpers;
pub mod linkage;

use self::ascension::*;
use self::linkage::*;
use husky_linkage_path::LinkagePath;
use husky_vfs::CratePath;

pub trait IsTask: Send + 'static {
    type DevAscension: IsDevAscension;
}
