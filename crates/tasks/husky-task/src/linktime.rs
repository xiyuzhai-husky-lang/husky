use crate::*;
use husky_linkage::linkage::Linkage;
use husky_linkage_impl::IsLinkageImpl;
use husky_vfs::linktime_target_path::LinktimeTargetPath;

pub trait IsLinktime: Sized + Send {
    type LinkageImpl: IsLinkageImpl;
    // linktime has the responsibility to guarantee that the linkage provided is up to date.
    fn linkage_impl(&self, linkage: Linkage, db: &::salsa::Db) -> Self::LinkageImpl;
    fn new_linktime(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self;
}
