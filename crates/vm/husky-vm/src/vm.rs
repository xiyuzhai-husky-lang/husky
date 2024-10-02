use std::sync::Arc;

use crate::{
    runtime::IsVmRuntime,
    snapshot::{VmSnapshotKey, VmSnapshotsData},
};
use husky_linket::{linket::Linket, template_argument::qual::LinQual};
use husky_linket_impl::{linket_impl::IsLinketImpl, LinketImplVmControlFlowThawed};
use husky_linket_impl::{
    linket_impl::{LinketImplSlushValue, LinketImplThawedValue},
    LinketImplVmControlFlowFrozen,
};
use husky_linktime::{
    helpers::{
        LinktimeSlushValue, LinktimeThawedValue, LinktimeValue, LinktimeVmControlFlow,
        LinktimeVmControlFlowFrozen,
    },
    IsLinktime,
};
use husky_place::{place::idx::PlaceIdx, PlaceRegistry};
use husky_value::{IsFrozenValue, IsThawedValue};
use husky_vmir::{
    eval::EvalVmir,
    expr::{VmirExprIdx, VmirExprMap},
    region::VmirRegion,
    stmt::{VmirStmtIdx, VmirStmtIdxRange, VmirStmtMap},
    storage::IsVmirStorage,
};
use rustc_hash::FxHashMap;
use vec_like::ordered_vec_map::{OrderedVecMap, OrderedVecPairMap};

use crate::{
    history::{VmHistory, VmRecord},
    snapshot::VmSnapshot,
};

pub(crate) struct Vm<
    'a,
    LinketImpl: IsLinketImpl,
    DevRuntime: IsVmRuntime<LinketImpl>,
    VmirStorage: IsVmirStorage<LinketImpl>,
> {
    linket: Linket,
    variable_slush_values: Vec<LinketImplSlushValue<LinketImpl>>,
    pub(crate) variable_thawed_values: Vec<LinketImplThawedValue<LinketImpl>>,
    mode: VmMode,
    expr_records: VmirExprMap<LinketImpl, VmRecord<LinketImpl>>,
    stmt_records: VmirStmtMap<LinketImpl, VmRecord<LinketImpl>>,
    snapshots: VmSnapshotsData<LinketImpl>,
    pub(crate) vmir_region: &'a VmirRegion<LinketImpl>,
    pub(crate) place_registry: &'a PlaceRegistry,
    pub(crate) db: &'a ::salsa::Db,
    pub(crate) runtime: &'a DevRuntime,
    // is this always useful?
    pub(crate) vmir_storage: &'a VmirStorage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmMode {
    Quick,
    Record,
}

impl<
        'a,
        LinketImpl: IsLinketImpl,
        DevRuntime: IsVmRuntime<LinketImpl>,
        VmirStorage: IsVmirStorage<LinketImpl>,
    > Vm<'a, LinketImpl, DevRuntime, VmirStorage>
{
    pub(crate) fn new_fresh(
        linket: Linket,
        arguments: Vec<<LinketImpl as IsLinketImpl>::Value>,
        mode: VmMode,
        vmir_region: &'a VmirRegion<LinketImpl>,
        db: &'a ::salsa::Db,
        runtime: &'a DevRuntime,
        vmir_storage: &'a VmirStorage, // used to access others
    ) -> Self {
        use husky_value::IsThawedValue;

        let place_registry = linket
            .place_registry(db)
            .expect("has vmir_region implies that this is some");
        let mut place_values = vec![];
        for _ in place_values.len()..place_registry.len() {
            place_values.push(LinketImplThawedValue::<LinketImpl>::new_uninit())
        }
        Self {
            linket,
            mode,
            variable_slush_values: vec![],
            variable_thawed_values: place_values,
            expr_records: VmirExprMap::new(vmir_region.vmir_expr_arena()),
            stmt_records: VmirStmtMap::new(vmir_region.vmir_stmt_arena()),
            snapshots: Default::default(),
            vmir_region,
            place_registry,
            db,
            runtime,
            vmir_storage,
        }
    }

    pub(crate) fn from_snapshot(
        snapshot: VmSnapshot<LinketImpl>,
        mode: VmMode,
        vmir_region: &'a VmirRegion<LinketImpl>,
        db: &'a ::salsa::Db,
        runtime: &'a DevRuntime,
        vmir_storage: &'a VmirStorage, // used to access others
    ) -> Self {
        let mut place_slush_values: Vec<LinketImplSlushValue<LinketImpl>> = vec![];
        let mut place_thawed_values: Vec<LinketImplThawedValue<LinketImpl>> = vec![];
        for place_frozen_value in snapshot.place_frozen_values() {
            let (slush_value, thawed_value) = place_frozen_value.thaw();
            place_slush_values.push(slush_value);
            place_thawed_values.push(thawed_value);
        }
        Self {
            linket: snapshot.linket(),
            variable_slush_values: place_slush_values,
            variable_thawed_values: place_thawed_values,
            mode,
            expr_records: VmirExprMap::new(vmir_region.vmir_expr_arena()),
            stmt_records: VmirStmtMap::new(vmir_region.vmir_stmt_arena()),
            vmir_region,
            place_registry: snapshot
                .linket()
                .place_registry(db)
                .expect("has vmir_region implies that this is some"),
            db,
            runtime,
            vmir_storage,
            snapshots: Default::default(),
        }
    }
}

/// # getters
///
impl<
        'a,
        LinketImpl: IsLinketImpl,
        DevRuntime: IsVmRuntime<LinketImpl>,
        VmirStorage: IsVmirStorage<LinketImpl>,
    > Vm<'a, LinketImpl, DevRuntime, VmirStorage>
{
    pub(crate) fn mode(&self) -> VmMode {
        self.mode
    }
}

/// # setters
impl<
        'a,
        LinketImpl: IsLinketImpl,
        DevRuntime: IsVmRuntime<LinketImpl>,
        VmirStorage: IsVmirStorage<LinketImpl>,
    > Vm<'a, LinketImpl, DevRuntime, VmirStorage>
{
    fn set_expr_record(
        &mut self,
        expr: VmirExprIdx<LinketImpl>,
        control_flow: LinketImplVmControlFlowFrozen<LinketImpl>,
    ) {
        self.expr_records
            .insert_new(*expr, VmRecord::new(control_flow))
    }

    fn set_stmt_record(
        &mut self,
        stmt: VmirStmtIdx<LinketImpl>,
        control_flow: LinketImplVmControlFlowFrozen<LinketImpl>,
    ) {
        self.stmt_records
            .insert_new(*stmt, VmRecord::new(control_flow))
    }
}

/// # actions
///
impl<
        'a,
        LinketImpl: IsLinketImpl,
        DevRuntime: IsVmRuntime<LinketImpl>,
        VmirStorage: IsVmirStorage<LinketImpl>,
    > Vm<'a, LinketImpl, DevRuntime, VmirStorage>
{
    pub(crate) fn record_expr(
        &mut self,
        expr: VmirExprIdx<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl> {
        let cf = f(self);
        let frozen_value = cf.freeze();
        self.set_expr_record(expr, frozen_value);
        cf
    }

    pub(crate) fn record_stmt(
        &mut self,
        stmt: VmirStmtIdx<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl> {
        let cf = f(self);
        let frozen_value = cf.freeze();
        self.set_stmt_record(stmt, frozen_value);
        cf
    }

    pub(crate) fn to_history(self) -> VmHistory<LinketImpl> {
        VmHistory::new(
            self.linket,
            self.expr_records,
            self.stmt_records,
            self.snapshots,
        )
    }

    pub(crate) fn quick<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let mode = self.mode;
        let r = f(self);
        self.mode = mode;
        r
    }

    pub(crate) fn snapshot(&mut self, stmt: VmirStmtIdx<LinketImpl>, key: VmSnapshotKey) {
        let t = || {
            VmSnapshot::new(
                self.linket,
                self.variable_thawed_values
                    .iter()
                    .map(|v| v.freeze())
                    .collect(),
            )
        };
        self.snapshots.update_value_or_insert_with(
            stmt,
            |map| {
                let Ok(()) = map.insert_new((key, t())) else {
                    unreachable!()
                };
            },
            || OrderedVecMap::new_one_element_map((key, t())),
        );
    }
}
