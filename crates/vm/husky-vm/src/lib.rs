#[cfg(test)]
mod tests;

#[cfg(test)]
use self::tests::*;
use husky_linkage::linkage::Linkage;
use husky_linkage::template_argument::qual::LinQual;
use husky_linkage::test_utils::TestLinkage;
use husky_place::place::idx::PlaceIdx;
use husky_place::PlaceRegistry;
use husky_task::linktime::IsLinktime;
use husky_task_interface::{vm_control_flow::LinkageImplVmControlFlow, IsLinkageImpl};
use husky_vmir::storage::VmirStorage;
use husky_vmir::{
    eval::EvalVmir,
    expr::VmirExprIdx,
    region::VmirRegion,
    stmt::{VmirStmtIdx, VmirStmtIdxRange},
};

struct StandardVm<'a, Linktime: IsLinktime> {
    vmir_region: &'a VmirRegion<Linktime::LinkageImpl>,
    place_registry: &'a PlaceRegistry,
    place_values: Vec<<Linktime::LinkageImpl as IsLinkageImpl>::Value>,
    db: &'a ::salsa::Db,
    linktime: &'a Linktime,
    vmir_storage: &'a VmirStorage<Linktime::LinkageImpl>,
}

pub fn eval_linkage_on_arguments<
    LinkageImpl: IsLinkageImpl,
    Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
>(
    linkage: Linkage,
    arguments: Vec<LinkageImpl::Value>,
    db: &::salsa::Db,
    linktime: &Linktime,
    vmir_storage: &VmirStorage<LinkageImpl>,
) -> Option<LinkageImplVmControlFlow<LinkageImpl>> {
    let vmir_region = vmir_storage.linkage_vmir_region(linkage, db, linktime)?;
    let mut vm = StandardVm::new(linkage, arguments, &vmir_region, db, linktime, vmir_storage);
    Some(vmir_region.root_expr().eval(None, &mut vm))
}

impl<'a, Linktime: IsLinktime> StandardVm<'a, Linktime> {
    fn new(
        linkage: Linkage,
        arguments: Vec<<Linktime::LinkageImpl as IsLinkageImpl>::Value>,
        vmir_region: &'a VmirRegion<Linktime::LinkageImpl>,
        db: &'a ::salsa::Db,
        linktime: &'a Linktime,
        vmir_storage: &'a VmirStorage<Linktime::LinkageImpl>,
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

impl<'a, Linktime: IsLinktime> EvalVmir<'a, Linktime::LinkageImpl> for StandardVm<'a, Linktime> {
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

    fn eval_expr_inner(
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
}

#[test]
fn vm_works_on_all_tests() {
    DB::vfs_expect_test_debug_with_db(
        |db, test_linkage: TestLinkage| run_test_linkage(test_linkage),
        &VfsTestConfig::new(
            "package_manifest_ast_sheet_sheet",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::TOML,
        ),
    );
}

fn run_test_linkage(test_linkage: TestLinkage) {
    todo!()
}
