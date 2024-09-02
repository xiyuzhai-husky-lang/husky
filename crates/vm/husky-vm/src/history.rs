use husky_linket_impl::{linket_impl::IsLinketImpl, LinketImplVmControlFlowFrozen};
use husky_vmir::{expr::VmirExprMap, stmt::VmirStmtMap};

pub struct VmHistory<LinketImpl: IsLinketImpl> {
    expr_control_flows: VmirExprMap<LinketImpl, VmRecord<LinketImpl>>,
    stmt_control_flows: VmirStmtMap<LinketImpl, VmRecord<LinketImpl>>,
}

impl<LinketImpl> VmHistory<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    pub(crate) fn new(
        expr_control_flows: VmirExprMap<LinketImpl, VmRecord<LinketImpl>>,
        stmt_control_flows: VmirStmtMap<LinketImpl, VmRecord<LinketImpl>>,
    ) -> Self {
        Self {
            expr_control_flows,
            stmt_control_flows,
        }
    }
}

pub struct VmRecord<LinketImpl: IsLinketImpl> {
    control_flow: LinketImplVmControlFlowFrozen<LinketImpl>,
}

impl<LinketImpl: IsLinketImpl> VmRecord<LinketImpl> {
    pub fn new(control_flow: LinketImplVmControlFlowFrozen<LinketImpl>) -> Self {
        Self { control_flow }
    }
}
