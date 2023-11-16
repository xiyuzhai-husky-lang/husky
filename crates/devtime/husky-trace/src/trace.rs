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
use husky_trace_protocol::{
    id::{TraceId, TraceKind},
    view::TraceViewData,
    IsTrace,
};
use husky_val_repr::repr::ValRepr;
use salsa::AsId;
use vec_like::VecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = TraceDb)]
#[enum_class::from_variants]
pub enum Trace {
    Submodule(SubmoduleTrace),
    ValItem(ValItemTrace),
    LazyCallInput(LazyCallInputTrace),
    LazyCall(LazyCallTrace),
    LazyExpr(LazyExprTrace),
    LazyPatternExpr(LazyPatternExprTrace),
    LazyStmt(LazyStmtTrace),
    EagerCallInput(EagerCallInputTrace),
    EagerCall(EagerCallTrace),
    EagerExpr(EagerExprTrace),
    EagerPatternExpr(EagerPatternExprTrace),
    EagerStmt(EagerStmtTrace),
}

impl Trace {
    fn from_item_path(item_path: ItemPath, db: &dyn TraceDb) -> Option<Self> {
        match item_path {
            ItemPath::Submodule(submodule_path) => {
                SubmoduleTrace::from_submodule_path(submodule_path, db).map(Into::into)
            }
            ItemPath::MajorItem(major_item_path) => Self::from_major_item_path(major_item_path, db),
            _ => None,
        }
    }

    fn from_major_item_path(major_item_path: MajorItemPath, db: &dyn TraceDb) -> Option<Self> {
        match major_item_path {
            MajorItemPath::Fugitive(fugitive_path) => Self::from_fugitive_path(fugitive_path, db),
            _ => None,
        }
    }

    fn from_fugitive_path(fugitive_path: FugitivePath, db: &dyn TraceDb) -> Option<Self> {
        match fugitive_path.fugitive_kind(db) {
            FugitiveKind::Val => Some(ValItemTrace::from_val_item_path(fugitive_path, db).into()),
            FugitiveKind::FunctionFn | FugitiveKind::FunctionGn | FugitiveKind::AliasType => None,
        }
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        TraceViewData::new(self.view_lines(db).data().to_vec(), self.have_subtraces(db))
    }

    pub fn view_lines<'a>(self, db: &'a dyn TraceDb) -> &'a TraceViewLines {
        match self {
            Trace::Submodule(slf) => slf.view_lines(db),
            Trace::ValItem(slf) => slf.view_lines(db),
            Trace::LazyCallInput(slf) => slf.view_lines(db),
            Trace::LazyCall(slf) => slf.view_lines(db),
            Trace::LazyExpr(slf) => slf.view_lines(db),
            Trace::LazyPatternExpr(slf) => slf.view_lines(db),
            Trace::LazyStmt(slf) => slf.view_lines(db),
            Trace::EagerCallInput(slf) => slf.view_lines(db),
            Trace::EagerCall(slf) => slf.view_lines(db),
            Trace::EagerExpr(slf) => slf.view_lines(db),
            Trace::EagerPatternExpr(slf) => slf.view_lines(db),
            Trace::EagerStmt(slf) => slf.view_lines(db),
        }
    }

    pub fn have_subtraces(self, db: &dyn TraceDb) -> bool {
        match self {
            Trace::Submodule(slf) => slf.have_subtraces(db),
            Trace::ValItem(slf) => slf.have_subtraces(db),
            Trace::LazyCallInput(slf) => slf.have_subtraces(db),
            Trace::LazyCall(slf) => slf.have_subtraces(db),
            Trace::LazyExpr(slf) => slf.have_subtraces(db),
            Trace::LazyPatternExpr(slf) => slf.have_subtraces(db),
            Trace::LazyStmt(slf) => slf.have_subtraces(db),
            Trace::EagerCallInput(slf) => slf.have_subtraces(db),
            Trace::EagerCall(slf) => slf.have_subtraces(db),
            Trace::EagerExpr(slf) => slf.have_subtraces(db),
            Trace::EagerPatternExpr(slf) => slf.have_subtraces(db),
            Trace::EagerStmt(slf) => slf.have_subtraces(db),
        }
    }

    pub fn subtraces<'a>(self, db: &'a dyn TraceDb) -> &'a [Trace] {
        match self {
            Trace::Submodule(slf) => slf.subtraces(db),
            Trace::ValItem(slf) => slf.subtraces(db),
            Trace::LazyCallInput(slf) => slf.subtraces(db),
            Trace::LazyCall(slf) => slf.subtraces(db),
            Trace::LazyExpr(slf) => slf.subtraces(db),
            Trace::LazyPatternExpr(slf) => slf.subtraces(db),
            Trace::LazyStmt(slf) => slf.subtraces(db),
            Trace::EagerCall(slf) => slf.subtraces(db),
            Trace::EagerCallInput(_) => todo!(),
            Trace::EagerExpr(slf) => slf.subtraces(db),
            Trace::EagerPatternExpr(slf) => slf.subtraces(db),
            Trace::EagerStmt(slf) => slf.subtraces(db),
        }
    }

    #[cfg(test)]
    fn val_repr(self, db: &dyn TraceDb) -> Option<ValRepr> {
        match self {
            Trace::ValItem(slf) => Some(slf.val_repr(db)),
            Trace::LazyExpr(slf) => slf.val_repr(db),
            Trace::LazyPatternExpr(slf) => slf.val_repr(db),
            Trace::LazyCall(slf) => Some(slf.val_repr(db)),
            Trace::LazyCallInput(slf) => Some(slf.val_repr(db)),
            Trace::LazyStmt(slf) => slf.val_repr(db),
            Trace::Submodule(_) => None,
            Trace::EagerExpr(_) => None,
            Trace::EagerPatternExpr(_) => None,
            Trace::EagerCallInput(_) => None,
            Trace::EagerCall(_) => None,
            Trace::EagerStmt(_) => None,
        }
    }

    #[cfg(test)]
    fn associated_traces(self, db: &dyn TraceDb) -> Vec<Trace> {
        self.view_data(db)
            .associated_trace_ids()
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

impl From<TraceId> for Trace {
    fn from(trace_id: TraceId) -> Self {
        match trace_id.kind() {
            TraceKind::Submodule => {
                Trace::Submodule(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::ValItem => Trace::ValItem(unsafe { std::mem::transmute(trace_id.value()) }),
            TraceKind::LazyCallInput => {
                Trace::LazyCallInput(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::LazyCall => {
                Trace::LazyCall(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::LazyExpr => {
                Trace::LazyExpr(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::LazyPatternExpr => {
                Trace::LazyPatternExpr(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::LazyStmt => {
                Trace::LazyStmt(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::EagerCallInput => {
                Trace::EagerCallInput(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::EagerCall => {
                Trace::EagerCall(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::EagerExpr => {
                Trace::EagerExpr(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::EagerPatternExpr => {
                Trace::EagerPatternExpr(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::EagerStmt => {
                Trace::EagerStmt(unsafe { std::mem::transmute(trace_id.value()) })
            }
        }
    }
}

impl Into<TraceId> for Trace {
    fn into(self) -> TraceId {
        match self {
            Trace::Submodule(trace) => {
                TraceId::new(TraceKind::Submodule, trace.as_id().as_nonzero_u32())
            }
            Trace::ValItem(trace) => {
                TraceId::new(TraceKind::ValItem, trace.as_id().as_nonzero_u32())
            }
            Trace::LazyCall(trace) => {
                TraceId::new(TraceKind::LazyCall, trace.as_id().as_nonzero_u32())
            }
            Trace::LazyCallInput(trace) => {
                TraceId::new(TraceKind::LazyCallInput, trace.as_id().as_nonzero_u32())
            }
            Trace::LazyExpr(trace) => {
                TraceId::new(TraceKind::LazyExpr, trace.as_id().as_nonzero_u32())
            }
            Trace::LazyPatternExpr(trace) => {
                TraceId::new(TraceKind::LazyPatternExpr, trace.as_id().as_nonzero_u32())
            }
            Trace::LazyStmt(trace) => {
                TraceId::new(TraceKind::LazyStmt, trace.as_id().as_nonzero_u32())
            }
            Trace::EagerCall(trace) => {
                TraceId::new(TraceKind::EagerCall, trace.as_id().as_nonzero_u32())
            }
            Trace::EagerCallInput(trace) => {
                TraceId::new(TraceKind::EagerCallInput, trace.as_id().as_nonzero_u32())
            }
            Trace::EagerExpr(trace) => {
                TraceId::new(TraceKind::EagerExpr, trace.as_id().as_nonzero_u32())
            }
            Trace::EagerPatternExpr(trace) => {
                TraceId::new(TraceKind::EagerPatternExpr, trace.as_id().as_nonzero_u32())
            }
            Trace::EagerStmt(trace) => {
                TraceId::new(TraceKind::EagerStmt, trace.as_id().as_nonzero_u32())
            }
        }
    }
}

impl IsTrace for Trace {}

#[salsa::tracked(jar = TraceJar, return_ref)]
pub(crate) fn root_traces(db: &dyn TraceDb, crate_path: CratePath) -> Vec<Trace> {
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
    db: &dyn TraceDb,
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
    db: &dyn TraceDb,
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
