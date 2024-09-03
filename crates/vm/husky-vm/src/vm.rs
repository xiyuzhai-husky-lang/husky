use crate::snapshot::VmSnapshotKey;
use husky_linket::{linket::Linket, template_argument::qual::LinQual};
use husky_linket_impl::{linket_impl::IsLinketImpl, LinketImplVmControlFlowThawed};
use husky_linktime::{
    helpers::{
        LinktimeSlushValue, LinktimeThawedValue, LinktimeValue, LinktimeVmControlFlow,
        LinktimeVmControlFlowFrozen,
    },
    IsLinktime,
};
use husky_place::{place::idx::PlaceIdx, PlaceRegistry};
use husky_value::IsFrozenValue;
use husky_vmir::{
    eval::EvalVmir,
    expr::{VmirExprIdx, VmirExprMap},
    region::VmirRegion,
    stmt::{VmirStmtIdx, VmirStmtIdxRange, VmirStmtMap},
    storage::IsVmirStorage,
};
use rustc_hash::FxHashMap;

use crate::{
    history::{VmHistory, VmRecord},
    snapshot::VmSnapshot,
};

pub(crate) struct Vm<'a, Linktime: IsLinktime, VmirStorage: IsVmirStorage<Linktime::LinketImpl>> {
    place_slush_values: Vec<LinktimeSlushValue<Linktime>>,
    pub(crate) place_thawed_values: Vec<LinktimeThawedValue<Linktime>>,
    mode: VmMode,
    expr_records: VmirExprMap<Linktime::LinketImpl, VmRecord<Linktime::LinketImpl>>,
    stmt_records: VmirStmtMap<Linktime::LinketImpl, VmRecord<Linktime::LinketImpl>>,
    snapshots: FxHashMap<VmSnapshotKey, VmSnapshot<Linktime::LinketImpl>>,
    pub(crate) vmir_region: &'a VmirRegion<Linktime::LinketImpl>,
    pub(crate) place_registry: &'a PlaceRegistry,
    pub(crate) db: &'a ::salsa::Db,
    pub(crate) linktime: &'a Linktime,
    // is this always useful?
    pub(crate) vmir_storage: &'a VmirStorage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmMode {
    Quick,
    Record,
}

impl<'a, Linktime, VmirStorage> Vm<'a, Linktime, VmirStorage>
where
    Linktime: IsLinktime,
    VmirStorage: IsVmirStorage<Linktime::LinketImpl>,
{
    pub(crate) fn new_fresh(
        linket: Linket,
        arguments: Vec<<Linktime::LinketImpl as IsLinketImpl>::Value>,
        mode: VmMode,
        vmir_region: &'a VmirRegion<Linktime::LinketImpl>,
        db: &'a ::salsa::Db,
        linktime: &'a Linktime,
        vmir_storage: &'a VmirStorage, // used to access others
    ) -> Self {
        use husky_value::IsValue;

        let place_registry = linket
            .place_registry(db)
            .expect("has vmir_region implies that this is some");
        let mut place_values = vec![];
        for _ in place_values.len()..place_registry.len() {
            todo!()
            // place_values.push(<Linktime::LinketImpl as IsLinketImpl>::Value::new_uninit())
        }
        Self {
            mode,
            place_slush_values: vec![],
            place_thawed_values: place_values,
            expr_records: VmirExprMap::new(vmir_region.vmir_expr_arena()),
            stmt_records: VmirStmtMap::new(vmir_region.vmir_stmt_arena()),
            snapshots: Default::default(),
            vmir_region,
            place_registry,
            db,
            linktime,
            vmir_storage,
        }
    }

    pub(crate) fn from_snapshot(
        snapshot: VmSnapshot<Linktime::LinketImpl>,
        mode: VmMode,
        vmir_region: &'a VmirRegion<Linktime::LinketImpl>,
        db: &'a ::salsa::Db,
        linktime: &'a Linktime,
        vmir_storage: &'a VmirStorage, // used to access others
    ) -> Self {
        let mut place_slush_values: Vec<LinktimeSlushValue<Linktime>> = vec![];
        let mut place_thawed_values: Vec<LinktimeThawedValue<Linktime>> = vec![];
        for place_frozen_value in snapshot.place_frozen_values() {
            let (slush_value, thawed_value) = place_frozen_value.thaw();
            place_slush_values.push(slush_value);
            place_thawed_values.push(thawed_value);
        }
        Self {
            place_slush_values,
            place_thawed_values,
            mode,
            expr_records: VmirExprMap::new(vmir_region.vmir_expr_arena()),
            stmt_records: VmirStmtMap::new(vmir_region.vmir_stmt_arena()),
            vmir_region,
            place_registry: snapshot
                .linket()
                .place_registry(db)
                .expect("has vmir_region implies that this is some"),
            db,
            linktime,
            vmir_storage,
            snapshots: Default::default(),
        }
    }
}

/// # getters
///
impl<'a, Linktime, VmirStorage> Vm<'a, Linktime, VmirStorage>
where
    Linktime: IsLinktime,
    VmirStorage: IsVmirStorage<Linktime::LinketImpl>,
{
    pub(crate) fn mode(&self) -> VmMode {
        self.mode
    }
}

/// # setters
impl<'a, Linktime, VmirStorage> Vm<'a, Linktime, VmirStorage>
where
    Linktime: IsLinktime,
    VmirStorage: IsVmirStorage<Linktime::LinketImpl>,
{
    fn set_expr_record(
        &mut self,
        expr: VmirExprIdx<Linktime::LinketImpl>,
        control_flow: LinktimeVmControlFlowFrozen<Linktime>,
    ) {
        self.expr_records
            .insert_new(*expr, VmRecord::new(control_flow))
    }

    fn set_stmt_record(
        &mut self,
        stmt: VmirStmtIdx<Linktime::LinketImpl>,
        control_flow: LinktimeVmControlFlowFrozen<Linktime>,
    ) {
        self.stmt_records
            .insert_new(*stmt, VmRecord::new(control_flow))
    }
}

/// # actions
///
impl<'a, Linktime, VmirStorage> Vm<'a, Linktime, VmirStorage>
where
    Linktime: IsLinktime,
    VmirStorage: IsVmirStorage<Linktime::LinketImpl>,
{
    pub(crate) fn record_expr(
        &mut self,
        expr: VmirExprIdx<Linktime::LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<Linktime::LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<Linktime::LinketImpl> {
        let cf = f(self);
        let frozen_value = cf.freeze();
        self.set_expr_record(expr, frozen_value);
        cf
    }

    pub(crate) fn record_stmt(
        &mut self,
        stmt: VmirStmtIdx<Linktime::LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<Linktime::LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<Linktime::LinketImpl> {
        let cf = f(self);
        let frozen_value = cf.freeze();
        self.set_stmt_record(stmt, frozen_value);
        cf
    }

    pub(crate) fn to_history(self) -> VmHistory<Linktime::LinketImpl> {
        VmHistory::new(self.expr_records, self.stmt_records)
    }
}
