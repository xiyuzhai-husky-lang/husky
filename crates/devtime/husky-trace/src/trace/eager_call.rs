use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_entity_syn_tree::HasSynNodePath;
use husky_syn_defn::item_syn_defn;

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
    pub(super) fn view_lines(&self, db: &::salsa::Db) -> TraceViewLines {
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

    pub(super) fn have_subtraces(&self, db: &::salsa::Db) -> bool {
        item_syn_defn(db, self.callee_path).is_some()
    }

    pub(super) fn subtraces(&self, trace: Trace, db: &::salsa::Db) -> Vec<Trace> {
        let biological_parent_path = self.path;
        let biological_parent = trace;
        Trace::new_eager_stmts_from_syn_body_with_syn_expr_region(
            biological_parent_path,
            biological_parent,
            item_syn_defn(db, self.callee_path),
            db,
        )
    }
}
