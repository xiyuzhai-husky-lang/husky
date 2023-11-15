use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_entity_path::AssociatedItemPath;
use husky_entity_syn_tree::helpers::tokra_region::HasDeclTokraRegion;
use husky_entity_syn_tree::HasSynNodePath;
use husky_syn_defn::HasSynDefn;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct LazyCallTracePath {
    pub biological_parent_path: LazyCallTraceBiologicalParentPath,
    pub callee_path: ItemPath,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum LazyCallTraceBiologicalParentPath {
    LazyExpr(LazyExprTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyCallTracePathData {
    FunctionFn { path: FugitivePath },
    AssociatedFunctionFn { path: AssociatedItemPath },
    MethodFn { path: AssociatedItemPath },
    FunctionGn { path: FugitivePath },
    AssociatedFunctionGn { path: AssociatedItemPath },
    MethodGn { path: AssociatedItemPath },
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyCallTrace {
    #[id]
    pub path: LazyCallTracePath,
    pub biological_parent: LazyCallTraceBiologicalParent,
    pub callee_path: ItemPath,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum LazyCallTraceBiologicalParent {
    LazyExpr(LazyExprTrace),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyCallTraceData {
    FunctionFn { path: FugitivePath },
    AssociatedFunctionFn { path: AssociatedItemPath },
    MethodFn { path: AssociatedItemPath },
    FunctionGn { path: FugitivePath },
    AssociatedFunctionGn { path: AssociatedItemPath },
    MethodGn { path: AssociatedItemPath },
}

impl LazyCallTraceData {
    fn path_data(&self) -> LazyCallTracePathData {
        match *self {
            LazyCallTraceData::FunctionFn { path } => LazyCallTracePathData::FunctionFn { path },
            LazyCallTraceData::AssociatedFunctionFn { path } => {
                LazyCallTracePathData::AssociatedFunctionFn { path }
            }
            LazyCallTraceData::MethodFn { path } => LazyCallTracePathData::MethodFn { path },
            LazyCallTraceData::FunctionGn { path } => todo!(),
            LazyCallTraceData::AssociatedFunctionGn { path } => todo!(),
            LazyCallTraceData::MethodGn { path } => todo!(),
        }
    }
}

impl LazyCallTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<LazyCallTraceBiologicalParentPath>,
        biological_parent: impl Into<LazyCallTraceBiologicalParent>,
        callee_path: ItemPath,
        db: &dyn TraceDb,
    ) -> Self {
        Self::new_inner(
            db,
            LazyCallTracePath::new(db, biological_parent_path.into(), callee_path),
            biological_parent.into(),
            callee_path,
        )
    }

    pub fn view_lines<'a>(self, db: &'a dyn TraceDb) -> &'a TraceViewLines {
        lazy_call_trace_view_lines(db, self)
    }

    pub fn have_subtraces(self, db: &dyn TraceDb) -> bool {
        lazy_call_trace_have_subtraces(db, self)
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        lazy_call_trace_subtraces(db, self)
    }

    pub fn val_repr(self, db: &dyn TraceDb) -> ValRepr {
        match self.biological_parent(db) {
            LazyCallTraceBiologicalParent::LazyExpr(parent_trace) => {
                parent_trace.val_repr(db).expect("should be some")
            }
        }
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_call_trace_view_lines(db: &dyn TraceDb, trace: LazyCallTrace) -> TraceViewLines {
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
fn lazy_call_trace_have_subtraces(db: &dyn TraceDb, trace: LazyCallTrace) -> bool {
    trace
        .callee_path(db)
        .syn_defn(db)
        .expect("no syn error at trace time")
        .body_with_syn_expr_region(db)
        .is_some()
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_call_trace_subtraces(db: &dyn TraceDb, trace: LazyCallTrace) -> Vec<Trace> {
    LazyStmtTrace::from_syn_body_with_syn_expr_region(
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
