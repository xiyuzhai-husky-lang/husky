use husky_place::place::idx::PlaceIdx;
use husky_task_interface::{vm_control_flow::LinkageImplVmControlFlow, IsLinkageImpl};
use husky_vmir::{
    eval::EvalVmir,
    expr::VmirExprIdx,
    region::VmirRegion,
    stmt::{VmirStmtIdx, VmirStmtIdxRange},
};

pub struct StandardVm<'comptime, LinkageImpl: IsLinkageImpl> {
    vmir_region: &'comptime VmirRegion<LinkageImpl>,
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
        qual: husky_linkage::template_argument::qual::LinQual,
    ) -> <LinkageImpl as IsLinkageImpl>::Value {
        todo!()
    }
}
