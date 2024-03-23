use crate::{
    expr::{VmirExprArena, VmirExprIdx},
    stmt::{VmirStmtArena, VmirStmtIdx, VmirStmtIdxRange},
};
use husky_linkage::template_argument::qual::LinQual;
use husky_place::place::idx::PlaceIdx;
use husky_task_interface::vm_control_flow::LinkageImplVmControlFlow;
use husky_task_interface::IsLinkageImpl;

pub trait EvalVmir<'comptime, LinkageImpl: IsLinkageImpl> {
    fn vmir_expr_arena(&self) -> &'comptime VmirExprArena<LinkageImpl>;
    fn vmir_stmt_arena(&self) -> &'comptime VmirStmtArena<LinkageImpl>;

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

    /// access variable
    fn access_variable(&mut self, place_idx: PlaceIdx, qual: LinQual) -> LinkageImpl::Value;
}
