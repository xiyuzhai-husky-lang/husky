use crate::{
    expr::{VmirExprArena, VmirExprIdx},
    region::VmirRegion,
    stmt::{VmirStmtArena, VmirStmtIdx, VmirStmtIdxRange},
};
use husky_linkage::template_argument::qual::LinQual;
use husky_place::place::idx::PlaceIdx;
use husky_task_interface::vm_control_flow::LinkageImplVmControlFlow;
use husky_task_interface::IsLinkageImpl;

pub trait EvalVmir<'comptime, LinkageImpl: IsLinkageImpl> {
    fn vmir_region(&self) -> &'comptime VmirRegion<LinkageImpl>;
    fn vmir_expr_arena(&self) -> &'comptime VmirExprArena<LinkageImpl> {
        self.vmir_region().vmir_expr_arena()
    }
    fn vmir_stmt_arena(&self) -> &'comptime VmirStmtArena<LinkageImpl> {
        self.vmir_region().vmir_stmt_arena()
    }

    /// wrap the expression evaluation process
    fn eval_expr(
        &mut self,
        expr: VmirExprIdx<LinkageImpl>,
        f: impl FnOnce(&mut Self) -> LinkageImplVmControlFlow<LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl>;

    /// wrap the inner expression evaluation process, which exclude evalution of subexpressions
    fn eval_expr_inner(
        &mut self,
        expr: VmirExprIdx<LinkageImpl>,
        f: impl FnOnce(&mut Self) -> LinkageImplVmControlFlow<LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl>;

    /// wrap the statements evaluation process
    fn eval_stmts(
        &mut self,
        stmts: VmirStmtIdxRange<LinkageImpl>,
        f: impl FnOnce(&mut Self) -> LinkageImplVmControlFlow<LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl>;

    /// wrap the statement evaluation process
    fn eval_stmt(
        &mut self,
        stmt: VmirStmtIdx<LinkageImpl>,
        f: impl FnOnce(&mut Self) -> LinkageImplVmControlFlow<LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl>;

    /// access place
    fn access_place(&mut self, place_idx: PlaceIdx, qual: LinQual) -> LinkageImpl::Value;
}
