use crate::{
    expr::{VmirExprArena, VmirExprIdx},
    region::VmirRegion,
    stmt::{VmirStmtArena, VmirStmtIdx, VmirStmtIdxRange},
};
use husky_linket::template_argument::qual::LinQual;
use husky_linket_impl::{linket_impl::IsLinketImpl, LinketImplVmControlFlow};
use husky_place::place::idx::PlaceIdx;

pub trait EvalVmir<'comptime, LinketImpl: IsLinketImpl> {
    fn db(&self) -> &'comptime ::salsa::Db;
    fn vmir_region(&self) -> &'comptime VmirRegion<LinketImpl>;
    fn vmir_expr_arena(&self) -> &'comptime VmirExprArena<LinketImpl> {
        self.vmir_region().vmir_expr_arena()
    }
    fn vmir_stmt_arena(&self) -> &'comptime VmirStmtArena<LinketImpl> {
        self.vmir_region().vmir_stmt_arena()
    }

    /// wrap the expression evaluation process
    fn eval_expr(
        &mut self,
        expr: VmirExprIdx<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlow<LinketImpl>,
    ) -> LinketImplVmControlFlow<LinketImpl>;

    /// wrap the inner expression evaluation process, which exclude evalution of subexpressions
    fn eval_expr_itself(
        &mut self,
        expr: VmirExprIdx<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlow<LinketImpl>,
    ) -> LinketImplVmControlFlow<LinketImpl>;

    /// wrap the statements evaluation process
    fn eval_stmts(
        &mut self,
        stmts: VmirStmtIdxRange<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlow<LinketImpl>,
    ) -> LinketImplVmControlFlow<LinketImpl>;

    /// wrap the statement evaluation process
    fn eval_stmt(
        &mut self,
        stmt: VmirStmtIdx<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlow<LinketImpl>,
    ) -> LinketImplVmControlFlow<LinketImpl>;

    /// access place
    fn access_place(&mut self, place_idx: PlaceIdx, qual: LinQual) -> LinketImpl::Value;

    fn init_place(&mut self, place_idx: PlaceIdx, value: LinketImpl::Value);
}
