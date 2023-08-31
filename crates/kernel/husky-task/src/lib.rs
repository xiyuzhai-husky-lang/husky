mod ascension;
pub mod helpers;

pub use self::ascension::*;

use husky_linkage_path::LinkagePath;
use small_cell_stack::SmallCellStack;
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    thread::LocalKey,
};

pub trait IsTask: RefUnwindSafe + UnwindSafe {
    type DevAscension: IsDevAscension;
}

pub trait IsLinkageTable: RefUnwindSafe + UnwindSafe {
    type ComptimeDb: ?Sized;
    type Linkage: IsLinkage;
    // linkage table has the responsibility to guarantee that the linkage provided is up to date.
    fn get_linkage(&self, path: LinkagePath, db: &Self::ComptimeDb) -> Self::Linkage;
}

pub trait IsLinkage: RefUnwindSafe + UnwindSafe + Copy {
    type Value;
    fn eval_fn() -> Self::Value;
    fn eval_gn() -> Self::Value;
}
