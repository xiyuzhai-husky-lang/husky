mod call_head;
mod expr;
mod impr_stmt;
mod lazy_block;
mod lazy_branch;
mod lazy_expr;
mod lazy_stmt;
mod strict_decl_stmt;
mod strict_expr;

use expr::ExprTokenConfig;
use feature::{LazyBlock, LazyBranch, LazyBranchKind, LazyExpr, LazyExprKind, LazyStmtKind};
use semantics::{ImprStmt, LoopKind};
use serde::Deserialize;
use stdx::sync::ARwLock;
use text::{Text, TextQueryGroup};
use vm::{History, InstructionSheet, LoopFrameSnapshot, StackSnapshot};

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

#[derive(Debug, Default)]
pub struct TraceFactory {
    traces: ARwLock<Vec<Option<Arc<Trace>>>>,
}

impl Serialize for TraceFactory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.traces
            .read(|traces| serializer.collect_seq(traces.iter()))
    }
}

impl TraceFactory {
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
            TraceKind::Main(lazy_block) => vec![TokenProps {
                kind: TraceTokenKind::Keyword,
                value: Cow::Borrowed("main"),
                associated_trace: None,
            }],
            TraceKind::LazyStmt(stmt) => self.lazy_stmt_tokens(stmt, text),
            TraceKind::LazyExpr(expr) => self.lazy_expr_tokens(expr, text, ExprTokenConfig::expr()),
            TraceKind::LazyBranch(branch) => self.lazy_branch_tokens(branch, text),
            TraceKind::Input(_) => todo!(),
            TraceKind::StrictDeclStmt { .. } => todo!(),
            TraceKind::ImprStmt {
                ref stmt,
                ref history,
            } => self.impr_stmt_tokens(stmt, text, history),
            TraceKind::StrictExpr {
                ref expr,
                ref history,
            } => self.strict_expr_tokens(expr, text, history, ExprTokenConfig::expr()),
            TraceKind::CallHead { ref tokens, .. } => tokens.clone(),
            TraceKind::LoopFrame {
                loop_frame_snapshot: ref vm_loop_frame,
                ..
            } => self.loop_frame_tokens(vm_loop_frame),
        }
    }

    fn new_trace(
        &self,
        parent_id: Option<TraceId>,
        indent: Indent,
        kind: TraceKind,
        text: &Text,
    ) -> Arc<Trace> {
        let trace = Arc::new(Trace::new(parent_id, indent, kind, self, text));
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

pub trait CreateTrace: TextQueryGroup {
    fn trace_factory(&self) -> &TraceFactory;
    fn trace_factory_arc(&self) -> Arc<TraceFactory>;

    fn lazy_block_subtraces(&self, parent: &Trace, lazy_block: &LazyBlock) -> Arc<Vec<Arc<Trace>>> {
        let text = &self.text(parent.file).unwrap();
        Arc::new(
            self.trace_factory()
                .lazy_block_subtraces(parent, lazy_block, text),
        )
    }

    fn lazy_branch_subtraces(&self, parent: &Trace, branch: &LazyBranch) -> Arc<Vec<Arc<Trace>>> {
        let text = &self.text(parent.file).unwrap();
        self.trace_factory()
            .lazy_branch_subtraces(parent, branch, self.trace_factory(), text)
    }

    fn loop_subtraces(
        &self,
        parent: &Trace,
        loop_kind: &LoopKind,
        loop_stmt: &Arc<ImprStmt>,
        stmts: &Arc<Vec<Arc<ImprStmt>>>,
        stack_snapshot: &StackSnapshot,
        body_instruction_sheet: &Arc<InstructionSheet>,
    ) -> Arc<Vec<Arc<Trace>>> {
        let text = &self.text(parent.file).unwrap();
        self.trace_factory().loop_subtraces(
            parent,
            loop_kind,
            loop_stmt,
            stmts,
            text,
            stack_snapshot,
            body_instruction_sheet,
        )
    }

    fn loop_frame_subtraces(
        &self,
        parent: &Trace,
        loop_frame_snapshot: &LoopFrameSnapshot,
        instruction_sheet: &InstructionSheet,
        stmts: &[Arc<ImprStmt>],
    ) -> Arc<Vec<Arc<Trace>>> {
        let text = &self.text(parent.file).unwrap();
        self.trace_factory().loop_frame_subtraces(
            parent,
            loop_frame_snapshot,
            instruction_sheet,
            stmts,
            text,
        )
    }

    fn new_trace(
        &self,
        parent_id: Option<TraceId>,
        file: FilePtr,
        indent: Indent,
        kind: TraceKind,
    ) -> Arc<Trace> {
        self.trace_factory()
            .new_trace(parent_id, indent, kind, &self.text(file).unwrap())
    }

    fn trace(&self, id: TraceId) -> Arc<Trace> {
        self.trace_factory()
            .traces
            .read(|traces| traces[id.0].as_ref().unwrap().clone())
    }

    fn clear(&self) {
        self.trace_factory().traces.write(|traces| traces.clear())
    }
}
