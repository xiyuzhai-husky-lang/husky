pub mod eager_call;
pub mod eager_call_input;
pub mod eager_expr;
pub mod eager_loop_group;
pub mod eager_pattern_expr;
pub mod eager_stmt;
pub mod lazy_call;
pub mod lazy_call_input;
pub mod lazy_expr;
mod lazy_loop_group;
pub mod lazy_pattern_expr;
pub mod lazy_stmt;
pub mod submodule;
pub mod val_item;

use self::eager_call::*;
use self::eager_call_input::*;
use self::eager_expr::*;
use self::eager_pattern_expr::*;
use self::eager_stmt::*;
use self::lazy_call::*;
use self::lazy_call_input::*;
use self::lazy_expr::*;
use self::lazy_pattern_expr::*;
use self::lazy_stmt::*;
use self::submodule::*;
use self::val_item::*;
use crate::{
    registry::trace_path::{TracePathDisambiguator, TracePathRegistry},
    *,
};
use husky_entity_kind::FugitiveKind;
use husky_entity_path::MajorItemPath;
use husky_entity_path::{FugitivePath, ItemPath};
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_sema_expr::SemaExprIdx;
use husky_trace_protocol::{id::TraceKind, protocol::IsTrace, view::TraceViewData};
use husky_val_repr::expansion::ValReprExpansion;
use husky_val_repr::repr::ValRepr;
use vec_like::VecPairMap;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct TracePath {
    #[return_ref]
    data: TracePathData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = TraceDb)]
#[enum_class::from_variants]
pub enum TracePathData {
    Submodule(SubmoduleTracePathData),
    ValItem(ValItemTracePathData),
    LazyCallInput(LazyCallInputTracePathData),
    LazyCall(LazyCallTracePathData),
    LazyExpr(LazyExprTracePathData),
    LazyPatternExpr(LazyPatternExprTracePathData),
    LazyStmt(LazyStmtTracePathData),
    EagerCallInput(EagerCallInputTracePathData),
    EagerCall(EagerCallTracePathData),
    EagerExpr(EagerExprTracePathData),
    EagerPatternExpr(EagerPatternExprTracePathData),
    EagerStmt(EagerStmtTracePathData),
}

impl TracePath {
    fn new(data: impl Into<TracePathData>, db: &::salsa::Db) -> Self {
        Self::new_inner(db, data.into())
    }
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct TraceId {
    #[id]
    path: TracePath,
    #[return_ref]
    data: TraceData,
}

impl From<::husky_trace_protocol::id::TraceId> for TraceId {
    fn from(id: ::husky_trace_protocol::id::TraceId) -> Self {
        unsafe { std::mem::transmute(id) }
    }
}

impl Into<::husky_trace_protocol::id::TraceId> for TraceId {
    fn into(self) -> ::husky_trace_protocol::id::TraceId {
        unsafe { std::mem::transmute(self) }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[enum_class::from_variants]
pub enum TraceData {
    Submodule(SubmoduleTraceData),
    ValItem(ValItemTraceData),
    LazyCallInput(LazyCallInputTraceData),
    LazyCall(LazyCallTraceData),
    LazyExpr(LazyExprTraceData),
    LazyPatternExpr(LazyPatternExprTraceData),
    LazyStmt(LazyStmtTraceData),
    EagerCallInput(EagerCallInputTraceData),
    EagerCall(EagerCallTraceData),
    EagerExpr(EagerExprTraceData),
    EagerPatternExpr(EagerPatternExprTraceData),
    EagerStmt(EagerStmtTraceData),
}

impl TraceId {
    fn from_item_path(item_path: ItemPath, db: &::salsa::Db) -> Option<Self> {
        match item_path {
            ItemPath::Submodule(_, submodule_path) => {
                TraceId::new_submodule(submodule_path, db).map(Into::into)
            }
            ItemPath::MajorItem(major_item_path) => Self::from_major_item_path(major_item_path, db),
            _ => None,
        }
    }

    fn from_major_item_path(major_item_path: MajorItemPath, db: &::salsa::Db) -> Option<Self> {
        match major_item_path {
            MajorItemPath::Fugitive(fugitive_path) => Self::from_fugitive_path(fugitive_path, db),
            _ => None,
        }
    }

    fn from_fugitive_path(fugitive_path: FugitivePath, db: &::salsa::Db) -> Option<Self> {
        match fugitive_path.fugitive_kind(db) {
            FugitiveKind::Val => Some(TraceId::from_val_item_path(fugitive_path, db).into()),
            FugitiveKind::FunctionFn | FugitiveKind::FunctionGn | FugitiveKind::AliasType => None,
        }
    }

    #[cfg(test)]
    fn associated_traces(self, db: &::salsa::Db) -> Vec<TraceId> {
        self.view_data(db)
            .associated_trace_ids()
            .into_iter()
            .map(Into::into)
            .collect()
    }

    pub(crate) fn new(path: TracePath, data: TraceData, db: &::salsa::Db) -> Self {
        Self::new_inner(db, path.into(), data.into())
    }

    pub fn view_data(self, db: &::salsa::Db) -> TraceViewData {
        TraceViewData::new(
            self.trace_kind(db),
            trace_view_lines(db, self).data(),
            self.have_subtraces(db),
        )
    }

    pub fn trace_kind(self, db: &::salsa::Db) -> TraceKind {
        self.data(db).trace_kind()
    }

    pub fn have_subtraces(self, db: &::salsa::Db) -> bool {
        trace_have_subtraces(db, self)
    }

    pub fn subtraces(self, db: &::salsa::Db) -> &[TraceId] {
        trace_subtraces(db, self)
    }

    pub fn val_repr(self, db: &::salsa::Db) -> Option<ValRepr> {
        self.data(db).val_repr(self, db)
    }

    pub fn val_repr_expansion(self, db: &::salsa::Db) -> ValReprExpansion {
        trace_val_repr_expansion(db, self)
    }
}

impl TraceData {
    pub fn trace_kind(&self) -> TraceKind {
        match self {
            TraceData::Submodule(_) => TraceKind::Submodule,
            TraceData::ValItem(_) => TraceKind::ValItem,
            TraceData::LazyCallInput(_) => TraceKind::LazyCallInput,
            TraceData::LazyCall(_) => TraceKind::LazyCall,
            TraceData::LazyExpr(_) => TraceKind::LazyExpr,
            TraceData::LazyPatternExpr(_) => TraceKind::LazyPatternExpr,
            TraceData::LazyStmt(_) => TraceKind::LazyStmt,
            TraceData::EagerCallInput(_) => TraceKind::EagerCallInput,
            TraceData::EagerCall(_) => TraceKind::EagerCall,
            TraceData::EagerExpr(_) => TraceKind::EagerExpr,
            TraceData::EagerPatternExpr(_) => TraceKind::EagerPatternExpr,
            TraceData::EagerStmt(_) => TraceKind::EagerStmt,
        }
    }

    pub fn val_repr(&self, trace_id: TraceId, db: &::salsa::Db) -> Option<ValRepr> {
        match self {
            TraceData::ValItem(slf) => Some(slf.val_repr(db)),
            TraceData::LazyExpr(slf) => slf.val_repr(trace_id, db),
            TraceData::LazyPatternExpr(slf) => slf.val_repr(trace_id, db),
            TraceData::LazyCall(slf) => Some(slf.val_repr(db)),
            TraceData::LazyCallInput(slf) => Some(slf.val_repr(db)),
            TraceData::LazyStmt(slf) => slf.val_repr(trace_id, db),
            TraceData::Submodule(_) => None,
            TraceData::EagerExpr(_) => None,
            TraceData::EagerPatternExpr(_) => None,
            TraceData::EagerCallInput(_) => None,
            TraceData::EagerCall(_) => None,
            TraceData::EagerStmt(_) => None,
        }
    }
}

#[salsa::tracked(jar = TraceJar)]
fn trace_view_lines(db: &::salsa::Db, trace_id: TraceId) -> TraceViewLines {
    trace_id.data(db).view_lines(trace_id, db)
}

#[salsa::tracked(jar = TraceJar)]
fn trace_have_subtraces(db: &::salsa::Db, trace_id: TraceId) -> bool {
    trace_id.data(db).have_subtraces(db)
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn trace_subtraces(db: &::salsa::Db, trace_id: TraceId) -> Vec<TraceId> {
    trace_id.data(db).subtraces(trace_id, db)
}

#[salsa::tracked(jar = TraceJar)]
fn trace_val_repr_expansion(db: &::salsa::Db, trace_id: TraceId) -> ValReprExpansion {
    trace_id.data(db).val_repr_expansion(trace_id, db)
}

impl TraceData {
    fn view_lines(&self, trace_id: TraceId, db: &::salsa::Db) -> TraceViewLines {
        match self {
            TraceData::Submodule(slf) => slf.view_lines(db),
            TraceData::ValItem(slf) => slf.view_lines(db),
            TraceData::LazyCallInput(slf) => slf.view_lines(db),
            TraceData::LazyCall(slf) => slf.view_lines(db),
            TraceData::LazyExpr(slf) => slf.view_lines(db),
            TraceData::LazyPatternExpr(slf) => slf.view_lines(db),
            TraceData::LazyStmt(slf) => slf.view_lines(trace_id, db),
            TraceData::EagerCallInput(slf) => slf.view_lines(db),
            TraceData::EagerCall(slf) => slf.view_lines(db),
            TraceData::EagerExpr(slf) => slf.view_lines(db),
            TraceData::EagerPatternExpr(slf) => slf.view_lines(db),
            TraceData::EagerStmt(slf) => slf.view_lines(trace_id, db),
        }
    }

    fn have_subtraces(&self, db: &::salsa::Db) -> bool {
        match self {
            TraceData::Submodule(slf) => slf.have_subtraces(),
            TraceData::ValItem(slf) => slf.have_subtraces(db),
            TraceData::LazyCallInput(slf) => slf.have_subtraces(),
            TraceData::LazyCall(slf) => slf.have_subtraces(db),
            TraceData::LazyExpr(slf) => slf.have_subtraces(db),
            TraceData::LazyPatternExpr(slf) => slf.have_subtraces(),
            TraceData::LazyStmt(slf) => slf.have_subtraces(db),
            TraceData::EagerCallInput(slf) => slf.have_subtraces(db),
            TraceData::EagerCall(slf) => slf.have_subtraces(db),
            TraceData::EagerExpr(slf) => slf.have_subtraces(db),
            TraceData::EagerPatternExpr(slf) => slf.have_subtraces(db),
            TraceData::EagerStmt(slf) => slf.have_subtraces(db),
        }
    }

    fn subtraces(&self, trace_id: TraceId, db: &::salsa::Db) -> Vec<TraceId> {
        match self {
            TraceData::Submodule(slf) => slf.subtraces(db),
            TraceData::ValItem(slf) => slf.subtraces(trace_id, db),
            TraceData::LazyCallInput(slf) => slf.subtraces(),
            TraceData::LazyCall(slf) => slf.subtraces(trace_id, db),
            TraceData::LazyExpr(slf) => slf.subtraces(trace_id, db),
            TraceData::LazyPatternExpr(slf) => slf.subtraces(),
            TraceData::LazyStmt(slf) => slf.subtraces(trace_id, db),
            TraceData::EagerCallInput(slf) => slf.subtraces(),
            TraceData::EagerCall(slf) => slf.subtraces(trace_id, db),
            TraceData::EagerExpr(slf) => slf.subtraces(trace_id, db),
            TraceData::EagerPatternExpr(slf) => slf.subtraces(),
            TraceData::EagerStmt(slf) => slf.subtraces(trace_id, db),
        }
    }

    fn val_repr_expansion(&self, trace_id: TraceId, db: &::salsa::Db) -> ValReprExpansion {
        match self {
            TraceData::Submodule(_) => unreachable!(),
            TraceData::ValItem(slf) => slf.val_repr_expansion(trace_id, db),
            TraceData::LazyCallInput(_) => todo!(),
            TraceData::LazyCall(_) => todo!(),
            TraceData::LazyExpr(slf) => slf.val_repr_expansion(db),
            TraceData::LazyPatternExpr(slf) => slf.val_repr_expansion(db),
            TraceData::LazyStmt(slf) => slf.val_repr_expansion(db),
            TraceData::EagerCallInput(_) => todo!(),
            TraceData::EagerCall(_) => todo!(),
            TraceData::EagerExpr(_) => todo!(),
            TraceData::EagerPatternExpr(_) => todo!(),
            TraceData::EagerStmt(_) => todo!(),
        }
    }
}

impl IsTrace for TraceId {}

#[salsa::tracked(jar = TraceJar, return_ref)]
pub(crate) fn root_traces(db: &::salsa::Db, crate_path: CratePath) -> Vec<TraceId> {
    let root_module_path = crate_path.root_module_path(db);
    module_item_paths(db, root_module_path)
        .iter()
        .filter_map(|&item_path| TraceId::from_item_path(item_path, db))
        .collect()
}

#[test]
fn root_traces_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug_with_db(
        |db, crate_path| root_traces(db, crate_path),
        &AstTestConfig::new("root_traces"),
    )
}

// utility function for finding all traces under a crate within certain depth
#[cfg(test)]
fn find_traces<R>(
    crate_path: CratePath,
    max_depth: u8,
    db: &::salsa::Db,
    f: impl Fn(TraceId) -> R,
) -> Vec<(TraceId, R)> {
    let mut traces: Vec<(TraceId, R)> = vec![];
    for &root_trace in root_traces(db, crate_path) {
        find_traces_aux(root_trace, max_depth - 1, &f, &mut traces, db)
    }
    traces
}

#[cfg(test)]
fn find_traces_aux<R>(
    trace: TraceId,
    max_depth: u8,
    f: &impl Fn(TraceId) -> R,
    traces: &mut Vec<(TraceId, R)>,
    db: &::salsa::Db,
) {
    traces.push((trace, f(trace)));
    if max_depth == 0 {
        return;
    }
    for &subtrace in trace.subtraces(db) {
        find_traces_aux(subtrace, max_depth - 1, f, traces, db)
    }
    for associated_trace in trace.associated_traces(db) {
        find_traces_aux(associated_trace, max_depth - 1, f, traces, db)
    }
}

#[test]
fn find_traces_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug_with_db(
        |db, crate_path| find_traces(crate_path, 5, db, |_| ()),
        &AstTestConfig::new("find_traces"),
    )
}

#[test]
fn trace_view_data_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug(
        |db, crate_path| find_traces(crate_path, 5, db, |trace| trace.view_data(db)),
        &AstTestConfig::new("trace_view_data"),
    )
}

#[test]
fn trace_val_repr_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug_with_db(
        |db, crate_path| find_traces(crate_path, 5, db, |trace| trace.val_repr(db)),
        &AstTestConfig::new("trace_val_repr"),
    )
}
