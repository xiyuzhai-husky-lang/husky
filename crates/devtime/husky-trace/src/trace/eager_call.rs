use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_entity_syn_tree::helpers::tokra_region::HasDeclTokraRegion;
use husky_entity_syn_tree::HasSynNodePath;
use husky_syn_defn::HasSynDefn;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new)]
pub struct EagerCallTracePath {
    pub biological_parent_path: EagerCallTraceBiologicalParentPath,
    pub callee_path: ItemPath,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EagerCallTraceBiologicalParentPath {
    EagerExpr(EagerExprTracePath),
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct EagerCallTrace {
    #[id]
    pub path: EagerCallTracePath,
    pub biological_parent: EagerCallTraceBiologicalParent,
    pub callee_path: ItemPath,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum EagerCallTraceBiologicalParent {
    EagerExpr(EagerExprTrace),
}

impl EagerCallTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<EagerCallTraceBiologicalParentPath>,
        biological_parent: impl Into<EagerCallTraceBiologicalParent>,
        callee_path: ItemPath,
        db: &dyn TraceDb,
    ) -> Self {
        Self::new_inner(
            db,
            EagerCallTracePath::new(db, biological_parent_path.into(), callee_path),
            biological_parent.into(),
            callee_path,
        )
    }

    pub fn view_lines<'a>(self, db: &'a dyn TraceDb) -> &'a TraceViewLines {
        eager_call_trace_view_lines(db, self)
    }

    pub fn have_subtraces(self, db: &dyn TraceDb) -> bool {
        eager_call_trace_have_subtraces(db, self)
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        eager_call_trace_subtraces(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_call_trace_view_lines(db: &dyn TraceDb, trace: EagerCallTrace) -> TraceViewLines {
    let callee_path = trace.callee_path(db);
    TraceViewLines::new(
        callee_path.module_path(db),
        callee_path
            .syn_node_path(db)
            .decl_tokra_region_token_idx_range(db),
        VoidAssociatedTraceRegistry,
        db,
    )
}

#[salsa::tracked(jar = TraceJar)]
fn eager_call_trace_have_subtraces(db: &dyn TraceDb, trace: EagerCallTrace) -> bool {
    trace
        .callee_path(db)
        .syn_defn(db)
        .expect("no syn error at trace time")
        .body_with_syn_expr_region(db)
        .is_some()
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_call_trace_subtraces(db: &dyn TraceDb, trace: EagerCallTrace) -> Vec<Trace> {
    EagerStmtTrace::from_syn_body_with_syn_expr_region(
        trace.path(db),
        trace,
        trace
            .callee_path(db)
            .syn_defn(db)
            .expect("no syn error at trace time")
            .body_with_syn_expr_region(db),
        db,
    )
}
