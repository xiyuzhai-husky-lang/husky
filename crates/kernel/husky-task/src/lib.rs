#![feature(associated_type_bounds)]
pub mod ascension;
pub mod helpers;
pub mod linktime;

use self::ascension::*;
use self::linktime::*;
use husky_vfs::CratePath;

pub trait IsTask: Send + 'static {
    type DevAscension: IsDevAscension;
}
