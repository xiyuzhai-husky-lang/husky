mod feature_block;
mod feature_branch;
mod feature_expr;
mod feature_stmt;

use feature::FeatureExpr;
use serde::Deserialize;
use stdx::sync::ARwLock;

use crate::*;

#[derive(Debug, Clone, Deserialize, Copy, PartialEq, Eq, Hash)]
pub struct TraceId(pub(crate) usize);

impl Serialize for TraceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0 as i64)
    }
}

#[test]
fn test_trace_id_deserialize() {
    let id0 = TraceId(0);
    let id1: TraceId = serde_json::from_str("0").unwrap();
    assert_eq!(id0, id1);
}

#[derive(Default)]
pub struct TraceAllocator {
    traces: ARwLock<Vec<Option<Arc<Trace>>>>,
    expr_traces: ARwLock<HashMap<(TraceId, usize), Arc<Trace>>>,
}

impl TraceAllocator {
    pub(crate) fn next_id(&self) -> TraceId {
        TraceId(self.traces.write(|traces| {
            traces.push(None);
            traces.len() - 1
        }))
    }

    pub(crate) fn tokens(&self, id: TraceId, kind: &TraceKind) -> Vec<TokenProps> {
        match kind {
            TraceKind::Main {
                main_file,
                feature_block,
            } => vec![TokenProps {
                kind: TraceTokenKind::Keyword,
                value: Cow::Borrowed("main"),
                associated_trace: None,
            }],
            TraceKind::FeatureStmt(stmt) => self.feature_stmt_tokens(id, stmt),
            TraceKind::FeatureExpr(expr) => self.feature_expr_tokens(expr, false),
            TraceKind::FeatureBranch(branch) => self.feature_branch_tokens(branch).into(),
        }
    }

    pub fn new_trace(&self, parent: Option<&Trace>, indent: Indent, kind: TraceKind) -> Arc<Trace> {
        let trace = Arc::new(Trace::new(parent.map(|trace| trace.id), indent, kind, self));
        self.traces.write(|traces| {
            assert!(traces[trace.id.0].is_none());
            traces[trace.id.0] = Some(trace.clone())
        });
        trace
    }

    fn expr_trace(&self, stmt_trace_id: TraceId, expr: &Arc<FeatureExpr>) -> Arc<Trace> {
        let p: *const FeatureExpr = &**expr;
        self.expr_traces.write(|expr_traces| {
            expr_traces
                .entry((stmt_trace_id, p as usize))
                .or_insert(self.new_trace(None, 0, TraceKind::FeatureExpr(expr.clone())))
                .clone()
        })
    }

    fn trace(&self, id: TraceId) -> Arc<Trace> {
        self.traces
            .read(|traces| traces[id.0].as_ref().unwrap().clone())
    }

    pub fn clear(&self) {
        self.traces.write(|traces| traces.clear())
    }
}

pub trait AllocateTrace {
    fn trace_allocator(&self) -> &TraceAllocator;

    fn new_trace(&self, parent: Option<&Trace>, indent: Indent, kind: TraceKind) -> Arc<Trace> {
        self.trace_allocator().new_trace(parent, indent, kind)
    }

    fn expr_trace(&self, stmt_trace_id: TraceId, expr: &Arc<FeatureExpr>) -> Arc<Trace> {
        self.trace_allocator().expr_trace(stmt_trace_id, expr)
    }

    fn trace(&self, id: TraceId) -> Arc<Trace> {
        self.trace_allocator().trace(id)
    }

    fn clear(&self) {
        self.trace_allocator().clear()
    }
}
