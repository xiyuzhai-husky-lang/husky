use husky_devsoul_interface::{IsDevRuntimeDyn, IsLinketImpl};
use husky_linket::linket::{virtual_linket_impl::VirtualLinketImpl, Linket};
use husky_vfs::path::linktime_target_path::LinktimeTargetPath;

pub trait IsLinktime: Sized + Send {
    type LinketImpl: IsLinketImpl;
    // linktime has the responsibility to guarantee that the linket provided is up to date.
    fn linket_impl(&self, linket: Linket, db: &::salsa::Db) -> Self::LinketImpl;
    fn new(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self;
    fn init(&self, runtime: &'static dyn IsDevRuntimeDyn<Self::LinketImpl>);
}

pub struct VirtualLinktime;

impl IsLinktime for VirtualLinktime {
    type LinketImpl = VirtualLinketImpl;

    #[inline(always)]
    fn linket_impl(&self, linket: Linket, _db: &salsa::Db) -> Self::LinketImpl {
        linket.into()
    }

    fn new(_target_path: LinktimeTargetPath, _db: &salsa::Db) -> Self {
        VirtualLinktime
    }

    fn init(&self, runtime_pinned: &dyn IsDevRuntimeDyn<Self::LinketImpl>) {}
}
