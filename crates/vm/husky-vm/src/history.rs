use crate::snapshot::{VmSnapshot, VmSnapshotKey, VmSnapshots, VmSnapshotsData};
use husky_linket::linket::Linket;
use husky_linket_impl::{linket_impl::IsLinketImpl, LinketImplVmControlFlowFrozen};
use husky_vmir::{
    expr::{VmirExprIdx, VmirExprMap},
    stmt::{VmirStmtIdx, VmirStmtMap},
};
use std::sync::Mutex;
use vec_like::ordered_vec_map::OrderedVecPairMap;

pub struct VmHistory<LinketImpl: IsLinketImpl> {
    linket: Linket,
    expr_control_flows: VmirExprMap<LinketImpl, VmRecord<LinketImpl>>,
    stmt_control_flows: VmirStmtMap<LinketImpl, VmRecord<LinketImpl>>,
    snapshots: Mutex<VmSnapshots<LinketImpl>>,
}

impl<LinketImpl: IsLinketImpl> std::ops::Index<VmirExprIdx<LinketImpl>> for VmHistory<LinketImpl> {
    type Output = VmRecord<LinketImpl>;

    fn index(&self, expr: VmirExprIdx<LinketImpl>) -> &Self::Output {
        &self.expr_control_flows[*expr]
    }
}

impl<LinketImpl: IsLinketImpl> std::ops::Index<VmirStmtIdx<LinketImpl>> for VmHistory<LinketImpl> {
    type Output = VmRecord<LinketImpl>;

    fn index(&self, stmt: VmirStmtIdx<LinketImpl>) -> &Self::Output {
        &self.stmt_control_flows[*stmt]
    }
}

impl<LinketImpl> VmHistory<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    pub(crate) fn new(
        linket: Linket,
        expr_control_flows: VmirExprMap<LinketImpl, VmRecord<LinketImpl>>,
        stmt_control_flows: VmirStmtMap<LinketImpl, VmRecord<LinketImpl>>,
        snapshots: VmSnapshotsData<LinketImpl>,
    ) -> Self {
        Self {
            linket,
            expr_control_flows,
            stmt_control_flows,
            snapshots: Mutex::new(VmSnapshots::new(snapshots)),
        }
    }

    pub fn linket(&self) -> Linket {
        self.linket
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

impl<LinketImpl: IsLinketImpl> VmRecord<LinketImpl> {
    pub fn control_flow(&self) -> &LinketImplVmControlFlowFrozen<LinketImpl> {
        &self.control_flow
    }
}
