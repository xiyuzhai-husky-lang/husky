#![feature(associated_type_bounds)]
pub mod ascension;
pub mod helpers;
pub mod link;

use self::ascension::*;
use self::link::*;
use husky_linkage_path::LinkagePath;
use husky_vfs::CratePath;

pub trait IsTask: Send + 'static {
    type DevAscension: IsDevAscension;
}
