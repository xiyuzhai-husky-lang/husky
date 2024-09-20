use husky_linket::linket::Linket;
use husky_linket_impl::eval_context::IsDevRuntimeInterfaceDyn;
use husky_linktime::IsLinktime;
use husky_vfs::path::linktime_target_path::LinktimeTargetPath;
use husky_virtual_linket_impl::VirtualLinketImpl;

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

    fn init(&self, runtime_pinned: &dyn IsDevRuntimeInterfaceDyn<Self::LinketImpl>) {}
}
