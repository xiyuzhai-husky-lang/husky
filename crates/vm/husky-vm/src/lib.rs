#[cfg(test)]
mod tests;

#[cfg(test)]
use self::tests::*;
use husky_linkage::linkage::Linkage;
use husky_linkage::template_argument::qual::LinQual;
use husky_place::place::idx::PlaceIdx;
use husky_place::PlaceRegistry;
use husky_task::linktime::IsLinktime;
use husky_task_interface::{vm_control_flow::LinkageImplVmControlFlow, IsLinkageImpl};
use husky_vmir::storage::IsVmirStorage;
use husky_vmir::{
    eval::EvalVmir,
    expr::VmirExprIdx,
    region::VmirRegion,
    stmt::{VmirStmtIdx, VmirStmtIdxRange},
};

struct StandardVm<'a, Linktime: IsLinktime, VmirStorage: IsVmirStorage<Linktime::LinkageImpl>> {
    vmir_region: &'a VmirRegion<Linktime::LinkageImpl>,
    place_registry: &'a PlaceRegistry,
    place_values: Vec<<Linktime::LinkageImpl as IsLinkageImpl>::Value>,
    db: &'a ::salsa::Db,
    linktime: &'a Linktime,
    vmir_storage: &'a VmirStorage,
}

pub fn eval_linkage_on_arguments<
    LinkageImpl: IsLinkageImpl,
    Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
>(
    linkage: Linkage,
    arguments: Vec<LinkageImpl::Value>,
    db: &::salsa::Db,
    linktime: &Linktime,
    vmir_storage: &impl IsVmirStorage<LinkageImpl>,
) -> Option<LinkageImplVmControlFlow<LinkageImpl>> {
    let vmir_region = vmir_storage.linkage_vmir_region(linkage, db, linktime)?;
    let mut vm = StandardVm::new(linkage, arguments, &vmir_region, db, linktime, vmir_storage);
    Some(vmir_region.root_expr().eval(None, &mut vm))
}

impl<'a, Linktime, VmirStorage> StandardVm<'a, Linktime, VmirStorage>
where
    Linktime: IsLinktime,
    VmirStorage: IsVmirStorage<Linktime::LinkageImpl>,
{
    fn new(
        linkage: Linkage,
        arguments: Vec<<Linktime::LinkageImpl as IsLinkageImpl>::Value>,
        vmir_region: &'a VmirRegion<Linktime::LinkageImpl>,
        db: &'a ::salsa::Db,
        linktime: &'a Linktime,
        vmir_storage: &'a VmirStorage,
    ) -> Self {
        use husky_value_interface::IsValue;

        let place_registry = linkage
            .place_registry(db)
            .expect("has vmir_region implies that this is some");
        let mut place_values = vec![];

        for _ in place_values.len()..place_registry.len() {
            place_values.push(<Linktime::LinkageImpl as IsLinkageImpl>::Value::new_uninit())
        }
        Self {
            vmir_region,
            place_registry,
            place_values,
            db,
            linktime,
            vmir_storage,
        }
    }
}

impl<'a, Linktime, VmirStorage> EvalVmir<'a, Linktime::LinkageImpl>
    for StandardVm<'a, Linktime, VmirStorage>
where
    Linktime: IsLinktime,
    VmirStorage: IsVmirStorage<Linktime::LinkageImpl>,
{
    fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    fn vmir_region(&self) -> &'a VmirRegion<Linktime::LinkageImpl> {
        self.vmir_region
    }

    fn eval_expr(
        &mut self,
        expr: VmirExprIdx<Linktime::LinkageImpl>,
        f: impl FnOnce(&mut Self) -> LinkageImplVmControlFlow<Linktime::LinkageImpl>,
    ) -> LinkageImplVmControlFlow<Linktime::LinkageImpl> {
        f(self)
    }

    fn eval_expr_itself(
        &mut self,
        expr: VmirExprIdx<Linktime::LinkageImpl>,
        f: impl FnOnce(&mut Self) -> LinkageImplVmControlFlow<Linktime::LinkageImpl>,
    ) -> LinkageImplVmControlFlow<Linktime::LinkageImpl> {
        f(self)
    }

    fn eval_stmts(
        &mut self,
        stmts: VmirStmtIdxRange<Linktime::LinkageImpl>,
        f: impl FnOnce(&mut Self) -> LinkageImplVmControlFlow<Linktime::LinkageImpl>,
    ) -> LinkageImplVmControlFlow<Linktime::LinkageImpl> {
        f(self)
    }

    fn eval_stmt(
        &mut self,
        stmt: VmirStmtIdx<Linktime::LinkageImpl>,
        f: impl FnOnce(&mut Self) -> LinkageImplVmControlFlow<Linktime::LinkageImpl>,
    ) -> LinkageImplVmControlFlow<Linktime::LinkageImpl> {
        f(self)
    }

    fn access_place(
        &mut self,
        place_idx: PlaceIdx,
        qual: LinQual,
    ) -> <Linktime::LinkageImpl as IsLinkageImpl>::Value {
        match qual {
            LinQual::Ref => todo!(),
            LinQual::RefMut => todo!(),
            LinQual::Transient => todo!(),
        }
    }

    fn init_place(
        &mut self,
        place_idx: PlaceIdx,
        value: <Linktime::LinkageImpl as IsLinkageImpl>::Value,
    ) {
        self.place_values[place_idx.index()] = value
    }
}

#[test]
fn run_test_linkage_works() {
    DB::vfs_plain_test(
        |db, test_linkage: TestLinkage| run_test_linkage(test_linkage, db),
        &VfsTestConfig::new(
            "run_test_linkage_works",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::TOML,
        ),
    );
}

#[cfg(test)]
fn run_test_linkage(test_linkage: TestLinkage, db: &::salsa::Db) {
    use husky_task::linktime::VirtualLinktime;
    use husky_vmir::storage::VirtualVmirStorage;

    let linktime = VirtualLinktime;
    let vmir_storage = VirtualVmirStorage;
    eval_linkage_on_arguments(*test_linkage, vec![], db, &linktime, &vmir_storage);
}
