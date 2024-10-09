use crate::{
    expr::{VmirExprArena, VmirExprIdx},
    region::VmirRegion,
    stmt::{VmirStmtArena, VmirStmtIdx, VmirStmtIdxRange},
};
use husky_entity_path::path::major_item::form::MajorFormPath;
use husky_hir_eager_expr::variable::runtime::HirEagerRuntimeVariableIdx;
use husky_linket::template_argument::qual::LinQual;
use husky_linket_impl::{
    linket_impl::{IsLinketImpl, LinketImplThawedValue},
    LinketImplVmControlFlowThawed,
};
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

    fn eval_val(&self, major_form_path: MajorFormPath)
        -> LinketImplVmControlFlowThawed<LinketImpl>;

    /// wrap the expression evaluation process
    fn eval_expr(
        &mut self,
        expr: VmirExprIdx<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl>;

    /// wrap the inner expression evaluation process, which exclude evalution of subexpressions
    fn eval_expr_itself(
        &mut self,
        expr: VmirExprIdx<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl>;

    /// wrap the statements evaluation process
    fn eval_stmts(
        &mut self,
        stmts: VmirStmtIdxRange<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl>;

    fn eval_loop_inner(
        &mut self,
        stmt: VmirStmtIdx<LinketImpl>,
        stmts: VmirStmtIdxRange<LinketImpl>,
        loop_index: usize,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<LinketImpl, ()>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl, ()>;

    /// wrap the statement evaluation process
    fn eval_stmt(
        &mut self,
        stmt: VmirStmtIdx<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl>;

    /// access variable
    fn access_variable(
        &mut self,
        variable_idx: HirEagerRuntimeVariableIdx,
        qual: LinQual,
    ) -> LinketImplThawedValue<LinketImpl>;

    fn init_variable(
        &mut self,
        variable_idx: HirEagerRuntimeVariableIdx,
        value: LinketImplThawedValue<LinketImpl>,
    );
    fn set_variable(
        &mut self,
        variable_idx: HirEagerRuntimeVariableIdx,
        value: LinketImplThawedValue<LinketImpl>,
    );
}
