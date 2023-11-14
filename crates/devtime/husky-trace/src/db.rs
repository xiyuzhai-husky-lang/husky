use husky_text::db::TextDb;
use husky_token_info::TokenInfoDb;

use crate::*;

pub trait TraceDb: salsa::DbWithJar<TraceJar> + TokenInfoDb + TextDb + ValReprDb {
    fn root_traces(&self, crate_path: CratePath) -> &[Trace];
}

impl<Db> TraceDb for Db
where
    Db: salsa::DbWithJar<TraceJar> + TokenInfoDb + TextDb + ValReprDb,
{
    fn root_traces(&self, crate_path: CratePath) -> &[Trace] {
        crate::trace::root_traces(self, crate_path).as_ref()
    }
}

#[salsa::jar(db = TraceDb)]
pub struct TraceJar(
    crate::trace::submodule::submodule_view_tokens,
    crate::trace::submodule::submodule_contains_val_item,
    crate::trace::submodule::submodule_trace_subtraces,
    crate::trace::val_item::ValItemTracePath,
    crate::trace::val_item::ValItemTrace,
    crate::trace::val_item::val_item_trace_view_lines,
    crate::trace::val_item::val_item_trace_subtraces,
    crate::trace::val_item::val_item_trace_val_repr,
    crate::trace::lazy_call::LazyCallTracePath,
    crate::trace::lazy_call::LazyCallTrace,
    crate::trace::lazy_expr::LazyExprTracePath,
    crate::trace::lazy_expr::LazyExprTrace,
    crate::trace::lazy_expr::lazy_expr_trace_view_lines,
    crate::trace::lazy_expr::lazy_expr_trace_val_repr,
    crate::trace::lazy_expr::lazy_expr_trace_val_repr_expansion,
    crate::trace::lazy_stmt::LazyStmtTracePath,
    crate::trace::lazy_stmt::LazyStmtTrace,
    crate::trace::lazy_stmt::lazy_stmt_trace_view_lines,
    crate::trace::lazy_stmt::lazy_stmt_trace_subtraces,
    crate::trace::lazy_stmt::lazy_stmt_trace_val_repr,
    crate::trace::lazy_stmt::lazy_stmt_trace_val_repr_expansion,
    crate::trace::eager_call::EagerCallTracePath,
    crate::trace::eager_call::EagerCallTrace,
    crate::trace::eager_expr::EagerExprTracePath,
    crate::trace::eager_expr::EagerExprTrace,
    crate::trace::eager_expr::eager_expr_trace_view_lines,
    crate::trace::eager_expr::eager_expr_trace_have_subtraces,
    crate::trace::eager_expr::eager_expr_trace_subtraces,
    crate::trace::eager_stmt::EagerStmtTracePath,
    crate::trace::eager_stmt::EagerStmtTrace,
    crate::trace::eager_stmt::eager_stmt_trace_subtraces,
    crate::trace::eager_stmt::eager_stmt_trace_view_lines,
    crate::trace::eager_loop_group::EagerLoopGroupTracePath,
    crate::trace::eager_loop_group::EagerLoopGroupTrace,
    // helpers
    crate::trace::root_traces,
);
