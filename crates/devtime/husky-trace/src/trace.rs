pub mod eager_call;
pub mod eager_expr;
pub mod eager_stmt;
pub mod lazy_call;
pub mod lazy_expr;
pub mod lazy_stmt;
pub mod loop_group;
pub mod submodule;
pub mod val_item;

use self::eager_call::*;
use self::eager_expr::*;
use self::eager_stmt::*;
use self::lazy_call::*;
use self::lazy_expr::*;
use self::lazy_stmt::*;
use self::loop_group::*;
use self::submodule::*;
use self::val_item::*;
use crate::{
    registry::{
        trace_path::{TracePathDisambiguator, TracePathRegistry},
        *,
    },
    *,
};
use husky_entity_kind::FugitiveKind;
use husky_entity_path::MajorItemPath;
use husky_entity_path::{FugitivePath, ItemPath};
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_sema_expr::SemaExprIdx;
use husky_trace_protocol::{
    id::{TraceId, TraceKind},
    settings::TraceSettings,
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
    LazyCall(LazyCallTrace),
    LazyExpr(LazyExprTrace),
    LazyStmt(LazyStmtTrace),
    EagerCall(EagerCallTrace),
    EagerExpr(EagerExprTrace),
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
        match self {
            Trace::Submodule(slf) => slf.view_data(db),
            Trace::ValItem(slf) => slf.view_data(db),
            Trace::LazyCall(slf) => slf.view_data(db),
            Trace::LazyExpr(slf) => slf.view_data(db),
            Trace::LazyStmt(slf) => slf.view_data(db),
            Trace::EagerCall(slf) => slf.view_data(db),
            Trace::EagerExpr(slf) => slf.view_data(db),
            Trace::EagerStmt(slf) => slf.view_data(db),
        }
    }

    pub fn subtraces<'a>(self, db: &'a dyn TraceDb) -> &'a [Trace] {
        match self {
            Trace::Submodule(slf) => slf.subtraces(db),
            Trace::ValItem(slf) => slf.subtraces(db),
            Trace::LazyCall(slf) => slf.subtraces(db),
            Trace::LazyExpr(slf) => slf.subtraces(db),
            Trace::LazyStmt(slf) => slf.subtraces(db),
            Trace::EagerCall(slf) => slf.subtraces(db),
            Trace::EagerExpr(slf) => slf.subtraces(db),
            Trace::EagerStmt(slf) => slf.subtraces(db),
        }
    }

    pub fn val_repr(self, db: &dyn TraceDb) -> ValRepr {
        todo!()
    }
}

impl From<TraceId> for Trace {
    fn from(trace_id: TraceId) -> Self {
        match trace_id.kind() {
            TraceKind::Submodule => {
                Trace::Submodule(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::ValItem => Trace::ValItem(unsafe { std::mem::transmute(trace_id.value()) }),
            TraceKind::LazyCall => {
                Trace::LazyCall(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::LazyExpr => {
                Trace::LazyExpr(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::LazyStmt => {
                Trace::LazyStmt(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::EagerCall => {
                Trace::EagerCall(unsafe { std::mem::transmute(trace_id.value()) })
            }
            TraceKind::EagerExpr => {
                Trace::EagerExpr(unsafe { std::mem::transmute(trace_id.value()) })
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
            Trace::LazyExpr(trace) => {
                TraceId::new(TraceKind::LazyExpr, trace.as_id().as_nonzero_u32())
            }
            Trace::LazyStmt(trace) => {
                TraceId::new(TraceKind::LazyStmt, trace.as_id().as_nonzero_u32())
            }
            Trace::EagerCall(trace) => {
                TraceId::new(TraceKind::EagerCall, trace.as_id().as_nonzero_u32())
            }
            Trace::EagerExpr(trace) => {
                TraceId::new(TraceKind::EagerExpr, trace.as_id().as_nonzero_u32())
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
    let Ok(item_paths) = module_item_paths(db, root_module_path) else {
        unreachable!("module path should be guaranteed to be valid")
    };
    item_paths
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
