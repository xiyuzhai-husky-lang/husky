use crate::*;

pub trait IsLinkTime: Sized + Default {
    type ComptimeDb: ?Sized;
    type Linkage: IsLinkage;
    // linkage table has the responsibility to guarantee that the linkage provided is up to date.
    fn get_linkage(&self, path: LinkagePath, db: &Self::ComptimeDb) -> Self::Linkage;
    fn new_linkage_table(target_crate: CratePath, db: &Self::ComptimeDb) -> Self;
}

pub trait IsLinkage: Copy {
    type Value;
    fn eval_fn() -> Self::Value;
    fn eval_gn() -> Self::Value;
}
