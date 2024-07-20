#[cfg(test)]
mod tests;

#[cfg(test)]
use self::tests::*;
use husky_devsoul::linktime::IsLinktime;
use husky_devsoul_interface::{vm_control_flow::LinketImplVmControlFlow, IsLinketImpl};
use husky_linket::linket::Linket;
use husky_linket::template_argument::qual::LinQual;
use husky_place::place::idx::PlaceIdx;
use husky_place::PlaceRegistry;
use husky_vmir::storage::IsVmirStorage;
use husky_vmir::{
    eval::EvalVmir,
    expr::VmirExprIdx,
    region::VmirRegion,
    stmt::{VmirStmtIdx, VmirStmtIdxRange},
};

struct StandardVm<'a, Linktime: IsLinktime, VmirStorage: IsVmirStorage<Linktime::LinketImpl>> {
    vmir_region: &'a VmirRegion<Linktime::LinketImpl>,
    place_registry: &'a PlaceRegistry,
    place_values: Vec<<Linktime::LinketImpl as IsLinketImpl>::Value>,
    db: &'a ::salsa::Db,
    linktime: &'a Linktime,
    vmir_storage: &'a VmirStorage,
}

pub fn eval_linket_on_arguments<
    LinketImpl: IsLinketImpl,
    Linktime: IsLinktime<LinketImpl = LinketImpl>,
>(
    linket: Linket,
    arguments: Vec<LinketImpl::Value>,
    db: &::salsa::Db,
    linktime: &Linktime,
    vmir_storage: &impl IsVmirStorage<LinketImpl>,
) -> Option<LinketImplVmControlFlow<LinketImpl>> {
    let vmir_region = vmir_storage.linket_vmir_region(linket, db, linktime)?;
    let mut vm = StandardVm::new(linket, arguments, &vmir_region, db, linktime, vmir_storage);
    Some(vmir_region.root_expr().eval(None, &mut vm))
}

impl<'a, Linktime, VmirStorage> StandardVm<'a, Linktime, VmirStorage>
where
    Linktime: IsLinktime,
    VmirStorage: IsVmirStorage<Linktime::LinketImpl>,
{
    fn new(
        linket: Linket,
        arguments: Vec<<Linktime::LinketImpl as IsLinketImpl>::Value>,
        vmir_region: &'a VmirRegion<Linktime::LinketImpl>,
        db: &'a ::salsa::Db,
        linktime: &'a Linktime,
        vmir_storage: &'a VmirStorage,
    ) -> Self {
        use husky_value_interface::IsValue;

        let place_registry = linket
            .place_registry(db)
            .expect("has vmir_region implies that this is some");
        let mut place_values = vec![];

        for _ in place_values.len()..place_registry.len() {
            place_values.push(<Linktime::LinketImpl as IsLinketImpl>::Value::new_uninit())
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

impl<'a, Linktime, VmirStorage> EvalVmir<'a, Linktime::LinketImpl>
    for StandardVm<'a, Linktime, VmirStorage>
where
    Linktime: IsLinktime,
    VmirStorage: IsVmirStorage<Linktime::LinketImpl>,
{
    fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    fn vmir_region(&self) -> &'a VmirRegion<Linktime::LinketImpl> {
        self.vmir_region
    }

    fn eval_expr(
        &mut self,
        expr: VmirExprIdx<Linktime::LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlow<Linktime::LinketImpl>,
    ) -> LinketImplVmControlFlow<Linktime::LinketImpl> {
        f(self)
    }

    fn eval_expr_itself(
        &mut self,
        expr: VmirExprIdx<Linktime::LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlow<Linktime::LinketImpl>,
    ) -> LinketImplVmControlFlow<Linktime::LinketImpl> {
        f(self)
    }

    fn eval_stmts(
        &mut self,
        stmts: VmirStmtIdxRange<Linktime::LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlow<Linktime::LinketImpl>,
    ) -> LinketImplVmControlFlow<Linktime::LinketImpl> {
        f(self)
    }

    fn eval_stmt(
        &mut self,
        stmt: VmirStmtIdx<Linktime::LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlow<Linktime::LinketImpl>,
    ) -> LinketImplVmControlFlow<Linktime::LinketImpl> {
        f(self)
    }

    fn access_place(
        &mut self,
        place_idx: PlaceIdx,
        qual: LinQual,
    ) -> <Linktime::LinketImpl as IsLinketImpl>::Value {
        match qual {
            LinQual::Ref => todo!(),
            LinQual::RefMut => todo!(),
            LinQual::Transient => todo!(),
        }
    }

    fn init_place(
        &mut self,
        place_idx: PlaceIdx,
        value: <Linktime::LinketImpl as IsLinketImpl>::Value,
    ) {
        self.place_values[place_idx.index()] = value
    }
}

#[test]
fn run_test_linket_works() {
    DB::vfs_plain_test(
        |db, test_linket: TestLinket| run_test_linket(test_linket, db),
        &VfsTestConfig::new(
            "run_test_linket_works",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::TOML,
        ),
    );
}

#[cfg(test)]
fn run_test_linket(test_linket: TestLinket, db: &::salsa::Db) {
    use husky_devsoul::linktime::VirtualLinktime;
    use husky_vmir::storage::VirtualVmirStorage;

    let linktime = VirtualLinktime;
    let vmir_storage = VirtualVmirStorage;
    eval_linket_on_arguments(*test_linket, vec![], db, &linktime, &vmir_storage);
}
