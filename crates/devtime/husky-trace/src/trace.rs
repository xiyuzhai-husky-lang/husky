mod eager_call;
mod eager_expr;
mod eager_stmt;
mod lazy_call;
mod lazy_expr;
mod lazy_stmt;
mod loop_group;
mod submodule;
mod val_item;

pub use self::eager_call::*;
pub use self::eager_expr::*;
pub use self::eager_stmt::*;
pub use self::lazy_call::*;
pub use self::lazy_expr::*;
pub use self::lazy_stmt::*;
pub use self::loop_group::*;
pub use self::submodule::*;
pub use self::val_item::*;

use crate::*;
use husky_entity_kind::FugitiveKind;
use husky_entity_path::MajorItemPath;
use husky_entity_path::{FugitivePath, ItemPath};
use husky_sema_expr::SemaExprIdx;
use husky_trace_protocol::{settings::TraceSettings, view::TraceViewData};
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

pub enum AssociatedExprTraces<'a> {
    Eager(&'a [(SemaExprIdx, EagerExprTrace)]),
    Lazy(&'a [(SemaExprIdx, LazyExprTrace)]),
}

pub enum Subtraces<'a> {
    Submodule(&'a [SubmoduleSubtrace]),
    LazyCall(&'a [LazyCallSubtrace]),
}

#[test]
fn associated_expr_traces_size() {
    assert_eq!(
        std::mem::size_of::<Option<AssociatedExprTraces>>(),
        std::mem::size_of::<AssociatedExprTraces>()
    )
}

impl Trace {
    pub(crate) fn from_item_path(item_path: ItemPath, db: &dyn TraceDb) -> Option<Self> {
        match item_path {
            ItemPath::Submodule(submodule_path) => {
                SubmoduleTrace::from_submodule_path(submodule_path, db).map(Into::into)
            }
            ItemPath::MajorItem(major_item_path) => Self::from_major_item_path(major_item_path, db),
            _ => None,
        }
    }

    pub fn from_major_item_path(major_item_path: MajorItemPath, db: &dyn TraceDb) -> Option<Self> {
        match major_item_path {
            MajorItemPath::Fugitive(fugitive_path) => Self::from_fugitive_path(fugitive_path, db),
            _ => None,
        }
    }

    pub fn from_fugitive_path(fugitive_path: FugitivePath, db: &dyn TraceDb) -> Option<Self> {
        match fugitive_path.fugitive_kind(db) {
            FugitiveKind::Val => Some(ValItemTrace::from_val_item_path(fugitive_path, db).into()),
            FugitiveKind::FunctionFn | FugitiveKind::FunctionGn | FugitiveKind::AliasType => None,
        }
    }

    pub fn associated_expr_traces(self, db: &dyn TraceDb) -> Option<AssociatedExprTraces> {
        match self {
            Trace::LazyStmt(trace) => {
                Some(AssociatedExprTraces::Lazy(trace.associated_expr_traces(db)))
            }
            Trace::EagerStmt(trace) => Some(AssociatedExprTraces::Eager(
                trace.associated_expr_traces(db),
            )),
            _ => None,
        }
    }

    pub fn view_data<'a>(self, db: &'a dyn TraceDb) -> &'a TraceViewData {
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

    pub fn subtraces<'a>(
        self,
        settings: &TraceSettings,
        db: &'a dyn TraceDb,
    ) -> Option<Subtraces<'a>> {
        match self {
            Trace::Submodule(slf) => slf.subtraces(db).map(Subtraces::Submodule),
            Trace::ValItem(_) => todo!(),
            Trace::LazyCall(slf) => slf.subtraces(db).map(Subtraces::LazyCall),
            Trace::LazyExpr(slf) => todo!(),
            Trace::LazyStmt(slf) => todo!(),
            Trace::EagerCall(slf) => todo!(),
            Trace::EagerExpr(slf) => todo!(),
            Trace::EagerStmt(slf) => todo!(),
        }
    }
}
