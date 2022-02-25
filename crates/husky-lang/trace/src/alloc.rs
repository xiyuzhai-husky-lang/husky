mod decl_stmt;
mod feature_block;
mod feature_branch;
mod feature_expr;
mod feature_stmt;

use feature::{
    FeatureBlock, FeatureBranch, FeatureBranchKind, FeatureExpr, FeatureExprKind, FeatureStmtKind,
};
use serde::Deserialize;
use stdx::sync::ARwLock;
use text::{Text, TextQueryGroup};

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
}

impl TraceAllocator {
    pub(crate) fn next_id(&self) -> TraceId {
        TraceId(self.traces.write(|traces| {
            traces.push(None);
            traces.len() - 1
        }))
    }

    pub(crate) fn tokens(
        &self,
        id: TraceId,
        indent: Indent,
        kind: &TraceKind,
        text: &Text,
    ) -> Vec<TokenProps> {
        match kind {
            TraceKind::Main(feature_block) => vec![TokenProps {
                kind: TraceTokenKind::Keyword,
                value: Cow::Borrowed("main"),
                associated_trace: None,
            }],
            TraceKind::FeatureStmt(stmt) => match stmt.kind {
                FeatureStmtKind::Init { varname, ref value } => {
                    let mut tokens = vec![];
                    tokens.push(ident!(varname.0, None));
                    tokens.push(special!(" = ", None));
                    tokens.extend(self.feature_expr_associated_tokens(indent + 4, value, text));
                    tokens.into()
                }
                FeatureStmtKind::Assert { ref condition } => {
                    let mut tokens = vec![];
                    tokens.push(keyword!("assert "));
                    tokens.extend(self.feature_expr_associated_tokens(indent + 4, condition, text));
                    tokens.into()
                }
                FeatureStmtKind::Return { ref result } => {
                    let mut tokens = vec![];
                    tokens.extend(self.feature_expr_associated_tokens(indent + 4, result, text));
                    tokens.into()
                }
                FeatureStmtKind::Branches { .. } => panic!(),
            },
            TraceKind::FeatureExpr(expr) => self.feature_expr_tokens(expr, text),
            TraceKind::FeatureBranch(branch) => match branch.kind {
                FeatureBranchKind::If { ref condition } => {
                    let mut tokens = vec![keyword!("if ")];
                    tokens.extend(self.feature_expr_tokens(condition, text));
                    tokens
                }
                FeatureBranchKind::Elif { ref condition } => {
                    let mut tokens = vec![keyword!("elif ")];
                    tokens.extend(self.feature_expr_tokens(condition, text));
                    tokens
                }
                FeatureBranchKind::Else => vec![keyword!("else ")],
            },
            TraceKind::Input(_) => todo!(),
            TraceKind::DeclStmt { ref tokens, .. } => tokens.clone(),
        }
    }

    fn new_trace(
        &self,
        parent: Option<&Trace>,
        indent: Indent,
        kind: TraceKind,
        text: &Text,
    ) -> Arc<Trace> {
        let trace = Arc::new(Trace::new(
            parent.map(|trace| trace.id),
            indent,
            kind,
            self,
            text,
        ));
        self.traces.write(|traces| {
            assert!(traces[trace.id.0].is_none());
            traces[trace.id.0] = Some(trace.clone())
        });
        trace
    }

    fn new_trace2(
        &self,
        parent: TraceId,
        indent: Indent,
        gen_kind: impl FnOnce(TraceId) -> TraceKind,
        text: &Text,
    ) -> Arc<Trace> {
        let trace = Arc::new(Trace::new2(Some(parent), indent, gen_kind, self, text));
        self.traces.write(|traces| {
            assert!(traces[trace.id.0].is_none());
            traces[trace.id.0] = Some(trace.clone())
        });
        trace
    }
}

pub trait AllocateTrace: TextQueryGroup {
    fn trace_allocator(&self) -> &TraceAllocator;
    fn trace_allocator_arc(&self) -> Arc<TraceAllocator>;

    fn feature_block_subtraces(
        &self,
        parent: &Trace,
        feature_block: &FeatureBlock,
    ) -> Vec<Arc<Trace>> {
        let text = &self.text(parent.file).unwrap();
        self.trace_allocator()
            .feature_block_subtraces(parent, feature_block, text)
    }

    fn feature_branch_subtraces(
        &self,
        parent: &Trace,
        branch: &FeatureBranch,
    ) -> Arc<Vec<Arc<Trace>>> {
        let text = &self.text(parent.file).unwrap();
        self.trace_allocator().feature_branch_subtraces(
            parent,
            branch,
            self.trace_allocator(),
            text,
        )
    }

    fn new_trace(
        &self,
        parent: Option<&Trace>,
        file: FilePtr,
        indent: Indent,
        kind: TraceKind,
    ) -> Arc<Trace> {
        self.trace_allocator()
            .new_trace(parent, indent, kind, &self.text(file).unwrap())
    }

    fn trace(&self, id: TraceId) -> Arc<Trace> {
        self.trace_allocator()
            .traces
            .read(|traces| traces[id.0].as_ref().unwrap().clone())
    }

    fn clear(&self) {
        self.trace_allocator().traces.write(|traces| traces.clear())
    }
}
