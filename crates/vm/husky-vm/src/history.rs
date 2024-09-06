use crate::snapshot::{VmSnapshot, VmSnapshotKey, VmSnapshots, VmSnapshotsData};
use husky_linket_impl::{linket_impl::IsLinketImpl, LinketImplVmControlFlowFrozen};
use husky_vmir::{expr::VmirExprMap, stmt::VmirStmtMap};
use std::sync::Mutex;
use vec_like::ordered_vec_map::OrderedVecPairMap;

pub struct VmHistory<LinketImpl: IsLinketImpl> {
    expr_control_flows: VmirExprMap<LinketImpl, VmRecord<LinketImpl>>,
    stmt_control_flows: VmirStmtMap<LinketImpl, VmRecord<LinketImpl>>,
    snapshots: Mutex<VmSnapshots<LinketImpl>>,
}

impl<LinketImpl> VmHistory<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    pub(crate) fn new(
        expr_control_flows: VmirExprMap<LinketImpl, VmRecord<LinketImpl>>,
        stmt_control_flows: VmirStmtMap<LinketImpl, VmRecord<LinketImpl>>,
        snapshots: VmSnapshotsData<LinketImpl>,
    ) -> Self {
        Self {
            expr_control_flows,
            stmt_control_flows,
            snapshots: Mutex::new(VmSnapshots::new(snapshots)),
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
