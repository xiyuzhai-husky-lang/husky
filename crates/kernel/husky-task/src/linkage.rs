use crate::*;

pub trait IsLinktime<ComptimeDb>: Sized + Send {
    type Linkage: IsLinkage;
    // linkage table has the responsibility to guarantee that the linkage provided is up to date.
    fn get_linkage(&self, path: LinkagePath, db: &ComptimeDb) -> Self::Linkage;
    fn new_linktime(target_crate: CratePath, db: &ComptimeDb) -> Self;
}

pub trait IsLinkage: Send + Copy {
    type Value;
    fn eval_fn() -> Self::Value;
    fn eval_gn() -> Self::Value;
}
