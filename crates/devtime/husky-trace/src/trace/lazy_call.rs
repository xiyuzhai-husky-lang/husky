use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_entity_path::AssociatedItemPath;
use husky_entity_syn_tree::helpers::tokra_region::HasDeclTokraRegion;
use husky_entity_syn_tree::HasSynNodePath;
use husky_syn_defn::HasSynDefn;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LazyCallTracePathData {
    biological_parent_path: TracePath,
    callee_path: ItemPath,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct LazyCallTraceData {
    path: TracePath,
    biological_parent: Trace,
    callee_path: ItemPath,
}

impl Trace {
    pub(crate) fn new_lazy_call(
        biological_parent_path: TracePath,
        biological_parent: Trace,
        callee_path: ItemPath,
        db: &::salsa::Db,
    ) -> Self {
        let path = TracePath::new(
            LazyCallTracePathData {
                biological_parent_path: biological_parent_path.into(),
                callee_path,
            },
            db,
        );
        Trace::new(
            path,
            LazyCallTraceData {
                path,
                biological_parent: biological_parent.into(),
                callee_path,
            }
            .into(),
            db,
        )
    }
}

impl LazyCallTraceData {
    fn view_lines(&self, db: &::salsa::Db) -> TraceViewLines {
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

    fn have_subtraces(&self, db: &::salsa::Db) -> bool {
        self.callee_path
            .syn_defn(db)
            .expect("no syn error at trace time")
            .body_with_syn_expr_region(db)
            .is_some()
    }

    fn subtraces(&self, trace: Trace, db: &::salsa::Db) -> Vec<Trace> {
        Trace::new_lazy_stmts_from_syn_body_with_syn_expr_region(
            self.path,
            trace,
            self.callee_path
                .syn_defn(db)
                .expect("no syn error at trace time")
                .body_with_syn_expr_region(db),
            db,
        )
    }

    pub fn val_repr(self, db: &::salsa::Db) -> ValRepr {
        self.biological_parent.val_repr(db).expect("should be some")
    }
}
