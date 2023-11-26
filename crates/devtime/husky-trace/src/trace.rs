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
use husky_trace_protocol::{protocol::IsTrace, view::TraceViewData};
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
pub struct Trace {
    #[id]
    path: TracePath,
    #[return_ref]
    data: TraceData,
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

impl Trace {
    fn from_item_path(item_path: ItemPath, db: &::salsa::Db) -> Option<Self> {
        match item_path {
            ItemPath::Submodule(submodule_path) => {
                Trace::new_submodule(submodule_path, db).map(Into::into)
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
            FugitiveKind::Val => Some(Trace::from_val_item_path(fugitive_path, db).into()),
            FugitiveKind::FunctionFn | FugitiveKind::FunctionGn | FugitiveKind::AliasType => None,
        }
    }

    #[cfg(test)]
    fn associated_traces(self, db: &::salsa::Db) -> Vec<Trace> {
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
        TraceViewData::new(trace_view_lines(db, self).data(), self.have_subtraces(db))
    }

    pub fn have_subtraces(self, db: &::salsa::Db) -> bool {
        trace_have_subtraces(db, self)
    }

    pub fn subtraces(self, db: &::salsa::Db) -> &[Trace] {
        trace_subtraces(db, self)
    }

    pub fn val_repr(self, db: &::salsa::Db) -> Option<ValRepr> {
        todo!()
        // match self {
        //     Trace::ValItem(slf) => Some(slf.val_repr(db)),
        //     Trace::LazyExpr(slf) => slf.val_repr(db),
        //     Trace::LazyPatternExpr(slf) => slf.val_repr(db),
        //     Trace::LazyCall(slf) => Some(slf.val_repr(db)),
        //     Trace::LazyCallInput(slf) => Some(slf.val_repr(db)),
        //     Trace::LazyStmt(slf) => slf.val_repr(db),
        //     Trace::Submodule(_) => None,
        //     Trace::EagerExpr(_) => None,
        //     Trace::EagerPatternExpr(_) => None,
        //     Trace::EagerCallInput(_) => None,
        //     Trace::EagerCall(_) => None,
        //     Trace::EagerStmt(_) => None,
        // }
    }

    pub fn val_repr_expansion(self, db: &::salsa::Db) -> ValReprExpansion {
        trace_val_repr_expansion(db, self)
    }
}

#[salsa::tracked(jar = TraceJar)]
fn trace_view_lines(db: &::salsa::Db, trace_id: Trace) -> TraceViewLines {
    trace_id.data(db).view_tokens(db)
}

#[salsa::tracked(jar = TraceJar)]
fn trace_have_subtraces(db: &::salsa::Db, trace_id: Trace) -> bool {
    trace_id.data(db).have_subtraces(db)
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn trace_subtraces(db: &::salsa::Db, trace_id: Trace) -> Vec<Trace> {
    trace_id.data(db).subtraces(db)
}

#[salsa::tracked(jar = TraceJar)]
fn trace_val_repr_expansion(db: &::salsa::Db, trace_id: Trace) -> ValReprExpansion {
    trace_id.data(db).val_repr_expansion(trace_id, db)
}

impl TraceData {
    fn view_tokens(&self, db: &::salsa::Db) -> TraceViewLines {
        match self {
            TraceData::Submodule(slf) => slf.view_tokens(db),
            TraceData::ValItem(_) => todo!(),
            TraceData::LazyCallInput(_) => todo!(),
            TraceData::LazyCall(_) => todo!(),
            TraceData::LazyExpr(_) => todo!(),
            TraceData::LazyPatternExpr(_) => todo!(),
            TraceData::LazyStmt(_) => todo!(),
            TraceData::EagerCallInput(_) => todo!(),
            TraceData::EagerCall(_) => todo!(),
            TraceData::EagerExpr(_) => todo!(),
            TraceData::EagerPatternExpr(_) => todo!(),
            TraceData::EagerStmt(_) => todo!(),
        }
    }

    fn have_subtraces(&self, db: &::salsa::Db) -> bool {
        match self {
            TraceData::Submodule(_) => todo!(),
            TraceData::ValItem(_) => todo!(),
            TraceData::LazyCallInput(_) => todo!(),
            TraceData::LazyCall(_) => todo!(),
            TraceData::LazyExpr(_) => todo!(),
            TraceData::LazyPatternExpr(_) => todo!(),
            TraceData::LazyStmt(_) => todo!(),
            TraceData::EagerCallInput(_) => todo!(),
            TraceData::EagerCall(_) => todo!(),
            TraceData::EagerExpr(_) => todo!(),
            TraceData::EagerPatternExpr(_) => todo!(),
            TraceData::EagerStmt(_) => todo!(),
        }
    }

    fn subtraces(&self, db: &::salsa::Db) -> Vec<Trace> {
        match self {
            TraceData::Submodule(_) => todo!(),
            TraceData::ValItem(_) => todo!(),
            TraceData::LazyCallInput(_) => todo!(),
            TraceData::LazyCall(_) => todo!(),
            TraceData::LazyExpr(_) => todo!(),
            TraceData::LazyPatternExpr(_) => todo!(),
            TraceData::LazyStmt(_) => todo!(),
            TraceData::EagerCallInput(_) => todo!(),
            TraceData::EagerCall(_) => todo!(),
            TraceData::EagerExpr(_) => todo!(),
            TraceData::EagerPatternExpr(_) => todo!(),
            TraceData::EagerStmt(_) => todo!(),
        }
    }

    fn val_repr_expansion(&self, trace_id: Trace, db: &::salsa::Db) -> ValReprExpansion {
        match self {
            TraceData::Submodule(_) => unreachable!(),
            TraceData::ValItem(slf) => slf.val_repr_expansion(trace_id, db),
            TraceData::LazyCallInput(_) => todo!(),
            TraceData::LazyCall(_) => todo!(),
            TraceData::LazyExpr(_) => todo!(),
            TraceData::LazyPatternExpr(_) => todo!(),
            TraceData::LazyStmt(_) => todo!(),
            TraceData::EagerCallInput(_) => todo!(),
            TraceData::EagerCall(_) => todo!(),
            TraceData::EagerExpr(_) => todo!(),
            TraceData::EagerPatternExpr(_) => todo!(),
            TraceData::EagerStmt(_) => todo!(),
        }
    }
}

impl From<husky_trace_protocol::id::TraceId> for Trace {
    fn from(trace_id: husky_trace_protocol::id::TraceId) -> Self {
        todo!()
        // match trace_id.kind() {
        //     TraceKind::Submodule => {
        //         Trace::Submodule(unsafe { std::mem::transmute(trace_id.value()) })
        //     }
        //     TraceKind::ValItem => Trace::ValItem(unsafe { std::mem::transmute(trace_id.value()) }),
        //     TraceKind::LazyCallInput => {
        //         Trace::LazyCallInput(unsafe { std::mem::transmute(trace_id.value()) })
        //     }
        //     TraceKind::LazyCall => {
        //         Trace::LazyCall(unsafe { std::mem::transmute(trace_id.value()) })
        //     }
        //     TraceKind::LazyExpr => {
        //         Trace::LazyExpr(unsafe { std::mem::transmute(trace_id.value()) })
        //     }
        //     TraceKind::LazyPatternExpr => {
        //         Trace::LazyPatternExpr(unsafe { std::mem::transmute(trace_id.value()) })
        //     }
        //     TraceKind::LazyStmt => {
        //         Trace::LazyStmt(unsafe { std::mem::transmute(trace_id.value()) })
        //     }
        //     TraceKind::EagerCallInput => {
        //         Trace::EagerCallInput(unsafe { std::mem::transmute(trace_id.value()) })
        //     }
        //     TraceKind::EagerCall => {
        //         Trace::EagerCall(unsafe { std::mem::transmute(trace_id.value()) })
        //     }
        //     TraceKind::EagerExpr => {
        //         Trace::EagerExpr(unsafe { std::mem::transmute(trace_id.value()) })
        //     }
        //     TraceKind::EagerPatternExpr => {
        //         Trace::EagerPatternExpr(unsafe { std::mem::transmute(trace_id.value()) })
        //     }
        //     TraceKind::EagerStmt => {
        //         Trace::EagerStmt(unsafe { std::mem::transmute(trace_id.value()) })
        //     }
        // }
    }
}

impl Into<husky_trace_protocol::id::TraceId> for Trace {
    fn into(self) -> husky_trace_protocol::id::TraceId {
        todo!()
        // match self {
        //     Trace::Submodule(trace) => {
        //         TraceId::new(TraceKind::Submodule, trace.as_id().as_nonzero_u32())
        //     }
        //     Trace::ValItem(trace) => {
        //         TraceId::new(TraceKind::ValItem, trace.as_id().as_nonzero_u32())
        //     }
        //     Trace::LazyCall(trace) => {
        //         TraceId::new(TraceKind::LazyCall, trace.as_id().as_nonzero_u32())
        //     }
        //     Trace::LazyCallInput(trace) => {
        //         TraceId::new(TraceKind::LazyCallInput, trace.as_id().as_nonzero_u32())
        //     }
        //     Trace::LazyExpr(trace) => {
        //         TraceId::new(TraceKind::LazyExpr, trace.as_id().as_nonzero_u32())
        //     }
        //     Trace::LazyPatternExpr(trace) => {
        //         TraceId::new(TraceKind::LazyPatternExpr, trace.as_id().as_nonzero_u32())
        //     }
        //     Trace::LazyStmt(trace) => {
        //         TraceId::new(TraceKind::LazyStmt, trace.as_id().as_nonzero_u32())
        //     }
        //     Trace::EagerCall(trace) => {
        //         TraceId::new(TraceKind::EagerCall, trace.as_id().as_nonzero_u32())
        //     }
        //     Trace::EagerCallInput(trace) => {
        //         TraceId::new(TraceKind::EagerCallInput, trace.as_id().as_nonzero_u32())
        //     }
        //     Trace::EagerExpr(trace) => {
        //         TraceId::new(TraceKind::EagerExpr, trace.as_id().as_nonzero_u32())
        //     }
        //     Trace::EagerPatternExpr(trace) => {
        //         TraceId::new(TraceKind::EagerPatternExpr, trace.as_id().as_nonzero_u32())
        //     }
        //     Trace::EagerStmt(trace) => {
        //         TraceId::new(TraceKind::EagerStmt, trace.as_id().as_nonzero_u32())
        //     }
        // }
    }
}

impl IsTrace for Trace {}

#[salsa::tracked(jar = TraceJar, return_ref)]
pub(crate) fn root_traces(db: &::salsa::Db, crate_path: CratePath) -> Vec<Trace> {
    let root_module_path = crate_path.root_module_path(db);
    module_item_paths(db, root_module_path)
        .iter()
        .filter_map(|&item_path| Trace::from_item_path(item_path, db))
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
    f: impl Fn(Trace) -> R,
) -> Vec<(Trace, R)> {
    let mut traces: Vec<(Trace, R)> = vec![];
    for &root_trace in root_traces(db, crate_path) {
        find_traces_aux(root_trace, max_depth - 1, &f, &mut traces, db)
    }
    traces
}

#[cfg(test)]
fn find_traces_aux<R>(
    trace: Trace,
    max_depth: u8,
    f: &impl Fn(Trace) -> R,
    traces: &mut Vec<(Trace, R)>,
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
