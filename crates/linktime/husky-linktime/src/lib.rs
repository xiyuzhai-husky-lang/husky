use husky_linket::linket::Linket;
use husky_linket_impl::eval_context::IsDevRuntimeDyn;
use husky_linket_impl::linket_impl::IsLinketImpl;
use husky_vfs::path::linktime_target_path::LinktimeTargetPath;

pub trait IsLinktime: Sized + Send {
    type LinketImpl: IsLinketImpl;
    // linktime has the responsibility to guarantee that the linket provided is up to date.
    fn linket_impl(&self, linket: Linket, db: &::salsa::Db) -> Self::LinketImpl;
    fn new(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self;
    fn init(&self, runtime: &'static dyn IsDevRuntimeDyn<Self::LinketImpl>);
}
