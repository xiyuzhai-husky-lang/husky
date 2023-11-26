use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_entity_syn_tree::helpers::tokra_region::HasDeclTokraRegion;
use husky_entity_syn_tree::HasSynNodePath;
use husky_syn_defn::HasSynDefn;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EagerCallTracePathData {
    biological_parent_path: TracePath,
    callee_path: ItemPath,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EagerCallTraceData {
    path: TracePath,
    biological_parent: Trace,
    callee_path: ItemPath,
}

impl Trace {
    pub(crate) fn new_eager_call(
        biological_parent_path: TracePath,
        biological_parent: Trace,
        callee_path: ItemPath,
        db: &::salsa::Db,
    ) -> Self {
        let path = TracePath::new(
            EagerCallTracePathData {
                biological_parent_path: biological_parent_path.into(),
                callee_path,
            },
            db,
        );
        Trace::new(
            path,
            EagerCallTraceData {
                path,
                biological_parent: biological_parent.into(),
                callee_path,
            }
            .into(),
            db,
        )
    }
}

impl EagerCallTraceData {
    fn eager_call_trace_view_lines(&self, db: &::salsa::Db) -> TraceViewLines {
        let callee_path = self.callee_path;
        TraceViewLines::new(
            callee_path.module_path(db),
            callee_path
                .syn_node_path(db)
                .decl_tokra_region_token_idx_range(db),
            VoidAssociatedTraceRegistry,
            db,
        )
    }

    fn eager_call_trace_have_subtraces(&self, db: &::salsa::Db) -> bool {
        self.callee_path
            .syn_defn(db)
            .expect("no syn error at trace time")
            .body_with_syn_expr_region(db)
            .is_some()
    }

    fn eager_call_trace_subtraces(&self, trace: Trace, db: &::salsa::Db) -> Vec<Trace> {
        let biological_parent_path = self.path;
        let biological_parent = trace;
        Trace::new_eager_stmts_from_syn_body_with_syn_expr_region(
            biological_parent_path,
            biological_parent,
            self.callee_path
                .syn_defn(db)
                .expect("no syn error at trace time")
                .body_with_syn_expr_region(db),
            db,
        )
    }
}
