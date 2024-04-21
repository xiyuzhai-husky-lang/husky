use husky_linkage::template_argument::qual::LinQual;
use husky_place::place::idx::PlaceIdx;
use husky_place::PlaceRegistry;
use husky_task_interface::{vm_control_flow::LinkageImplVmControlFlow, IsLinkageImpl};
use husky_vmir::{
    eval::EvalVmir,
    expr::VmirExprIdx,
    region::VmirRegion,
    stmt::{VmirStmtIdx, VmirStmtIdxRange},
};

pub struct StandardVm<'comptime, LinkageImpl: IsLinkageImpl> {
    vmir_region: &'comptime VmirRegion<LinkageImpl>,
    place_registry: &'comptime PlaceRegistry,
    place_values: Vec<LinkageImpl::Value>,
}

impl<'comptime, LinkageImpl: IsLinkageImpl> StandardVm<'comptime, LinkageImpl> {
    fn new(
        vmir_region: &'comptime VmirRegion<LinkageImpl>,
        place_registry: &'comptime PlaceRegistry,
    ) -> Self {
        use husky_value_interface::IsValue;

        Self {
            vmir_region,
            place_registry,
            place_values: (0..place_registry.len())
                .map(|_| LinkageImpl::Value::new_uninit())
                .collect(),
        }
    }
}

impl<'comptime, LinkageImpl: IsLinkageImpl> EvalVmir<'comptime, LinkageImpl>
    for StandardVm<'comptime, LinkageImpl>
{
    fn vmir_region(&self) -> &'comptime VmirRegion<LinkageImpl> {
        &self.vmir_region
    }

    fn eval_expr(
        &mut self,
        expr: VmirExprIdx<LinkageImpl>,
        f: impl FnOnce(&mut Self) -> LinkageImplVmControlFlow<LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl> {
        f(self)
    }

    fn eval_expr_inner(
        &mut self,
        expr: VmirExprIdx<LinkageImpl>,
        f: impl FnOnce(&mut Self) -> LinkageImplVmControlFlow<LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl> {
        f(self)
    }

    fn eval_stmts(
        &mut self,
        stmts: VmirStmtIdxRange<LinkageImpl>,
        f: impl FnOnce(&mut Self) -> LinkageImplVmControlFlow<LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl> {
        f(self)
    }

    fn eval_stmt(
        &mut self,
        stmt: VmirStmtIdx<LinkageImpl>,
        f: impl FnOnce(&mut Self) -> LinkageImplVmControlFlow<LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl> {
        f(self)
    }

    fn access_place(
        &mut self,
        place_idx: PlaceIdx,
        qual: LinQual,
    ) -> <LinkageImpl as IsLinkageImpl>::Value {
        match qual {
            LinQual::Ref => todo!(),
            LinQual::RefMut => todo!(),
            LinQual::Transient => todo!(),
        }
    }
}
