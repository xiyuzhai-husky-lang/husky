use husky_entity_syn_tree::EntitySynTreeDb;
use husky_text::db::TextDb;
use husky_token_info::TokenInfoDb;

use crate::*;

pub trait TraceDb: salsa::DbWithJar<TraceJar> + TokenInfoDb + TextDb {
    fn root_traces(&self, crate_path: CratePath) -> &[Trace];
}

impl<Db> TraceDb for Db
where
    Db: salsa::DbWithJar<TraceJar> + TokenInfoDb + TextDb,
{
    fn root_traces(&self, crate_path: CratePath) -> &[Trace] {
        crate::helpers::root_traces(self, crate_path).as_ref()
    }
}

#[salsa::jar(db = TraceDb)]
pub struct TraceJar(
    submodule_view_tokens,
    submodule_contains_val_item,
    submodule_trace_subtraces,
    ValItemTracePath,
    ValItemTrace,
    val_item_view_tokens,
    LazyCallTracePath,
    LazyCallTrace,
    LazyExprTracePath,
    LazyExprTrace,
    LazyStmtTracePath,
    LazyStmtTrace,
    lazy_stmt_associated_expr_traces,
    EagerCallTracePath,
    EagerCallTrace,
    EagerExprTracePath,
    EagerExprTrace,
    EagerStmtTracePath,
    EagerStmtTrace,
    eager_stmt_associated_expr_traces,
    LoopGroupTracePath,
    LoopGroupTrace,
    // helpers
    crate::helpers::root_traces,
);
