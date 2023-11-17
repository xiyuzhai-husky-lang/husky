use crate::*;
use husky_vfs::linktime_target_path::LinktimeTargetPath;

pub trait IsLinktime<ComptimeDb>: Sized + Send {
    type Linkage: IsLinkage;
    // linktime has the responsibility to guarantee that the linkage provided is up to date.
    fn get_linkage(&self, path: LinkagePath, db: &ComptimeDb) -> Self::Linkage;
    fn new_linktime(target_path: LinktimeTargetPath, db: &ComptimeDb) -> Self;
}

pub trait IsLinkage: Send + Copy {
    type Value;
    fn eval_fn() -> Self::Value;
    fn eval_gn() -> Self::Value;
}
