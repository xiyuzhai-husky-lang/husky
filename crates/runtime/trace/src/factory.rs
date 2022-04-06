mod call_head;
mod eager_decl_stmt;
mod eager_expr;
mod eager_impr_stmt;
mod expr;
mod feature_block;
mod feature_branch;
mod feature_expr;
mod feature_stmt;

use compile_time_db::HuskyLangCompileTime;
use expr::ExprTokenConfig;
use feature::*;
use semantics_eager::*;
use serde::Deserialize;
use sync_utils::ARwLock;
use text::{Text, TextQueryGroup};
use vm::{InstructionSheet, LoopFrameSnapshot, StackSnapshot};

use crate::*;

#[derive(Debug, Clone, Deserialize, Copy, PartialEq, Eq, Hash)]
pub struct TraceId(pub usize);

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
pub struct TraceFactory<'eval> {
    traces: ARwLock<Vec<Option<Arc<Trace<'eval>>>>>,
}

impl<'eval> Serialize for TraceFactory<'eval> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.traces.read(|traces| {
            serializer.collect_seq(traces.iter().map(|opt_trace| -> Option<&Trace<'eval>> {
                opt_trace.as_ref().map(|trace| -> &Trace<'eval> { trace })
            }))
        })
    }
}

impl<'eval> TraceFactory<'eval> {
    pub(crate) fn next_id(&self) -> TraceId {
        TraceId(self.traces.write(|traces| {
            traces.push(None);
            traces.len() - 1
        }))
    }

    pub(crate) fn lines(
        &self,
        id: TraceId,
        indent: Indent,
        kind: &TraceKind<'eval>,
        text: &Text,
    ) -> Vec<LineProps<'eval>> {
        match kind {
            TraceKind::Main(feature_block) => vec![LineProps {
                indent,
                idx: 0,
                tokens: vec![TokenProps {
                    kind: TraceTokenKind::Keyword,
                    value: Cow::Borrowed("main"),
                    associated_trace: None,
                }],
            }],
            TraceKind::FeatureStmt(stmt) => self.feature_stmt_lines(stmt, text),
            TraceKind::FeatureExpr(expr) => {
                self.feature_expr_lines(expr, text, ExprTokenConfig::expr())
            }
            TraceKind::FeatureBranch(branch) => self.feature_branch_lines(indent, branch, text),
            TraceKind::Input(_) => todo!(),
            TraceKind::StrictDeclStmt { .. } => todo!(),
            TraceKind::ImprStmt {
                ref stmt,
                ref history,
            } => self.impr_stmt_lines(stmt, text, history),
            TraceKind::EagerExpr {
                ref expr,
                ref history,
            } => self.eager_expr_lines(expr, text, history, ExprTokenConfig::expr()),
            TraceKind::CallHead { ref tokens, .. } => vec![LineProps {
                indent: 0,
                idx: 0,
                tokens: tokens.clone(),
            }],
            TraceKind::LoopFrame {
                loop_frame_snapshot: ref vm_loop_frame,
                ..
            } => self.loop_frame_lines(indent, vm_loop_frame),
        }
    }

    fn new_trace(
        &self,
        parent_id: Option<TraceId>,
        indent: Indent,
        kind: TraceKind<'eval>,
        text: &Text,
    ) -> Arc<Trace<'eval>> {
        let trace = Arc::new(Trace::new(parent_id, indent, kind, self, text));
        self.traces.write(|traces| {
            assert!(traces[trace.id.0].is_none());
            traces[trace.id.0] = Some(trace.clone())
        });
        trace
    }

    // fn new_trace2(
    //     &self,
    //     parent: TraceId,
    //     indent: Indent,
    //     gen_kind: impl FnOnce(TraceId) -> TraceKind,
    //     text: &Text,
    // ) -> Arc<Trace> {
    //     let trace = Arc::new(Trace::new2(Some(parent), indent, gen_kind, self, text));
    //     self.traces.write(|traces| {
    //         assert!(traces[trace.id.0].is_none());
    //         traces[trace.id.0] = Some(trace.clone())
    //     });
    //     trace
    // }
}

pub trait CreateTrace<'eval>: TextQueryGroup {
    fn trace_factory(&self) -> &TraceFactory<'eval>;
    fn trace_factory_arc(&self) -> Arc<TraceFactory<'eval>>;

    fn feature_block_subtraces(
        &self,
        parent: &Trace<'eval>,
        feature_block: &FeatureBlock,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        let text = &self.text(parent.file).unwrap();
        Arc::new(
            self.trace_factory()
                .feature_block_subtraces(parent, feature_block, text),
        )
    }

    fn feature_branch_subtraces(
        &self,
        parent: &Trace<'eval>,
        branch: &FeatureBranch,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        let text = &self.text(parent.file).unwrap();
        self.trace_factory()
            .feature_branch_subtraces(parent, branch, self.trace_factory(), text)
    }

    fn loop_subtraces(
        &self,
        compile_time: &HuskyLangCompileTime,
        parent: &Trace,
        loop_kind: &LoopKind,
        loop_stmt: &Arc<ProcStmt>,
        stmts: &Arc<Vec<Arc<ProcStmt>>>,
        stack_snapshot: &StackSnapshot<'eval>,
        body_instruction_sheet: &Arc<InstructionSheet>,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        let text = &self.text(parent.file).unwrap();
        self.trace_factory().loop_subtraces(
            compile_time,
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
        compile_time: &HuskyLangCompileTime,
        parent: &Trace,
        loop_frame_snapshot: &LoopFrameSnapshot<'eval>,
        instruction_sheet: &InstructionSheet,
        stmts: &[Arc<ProcStmt>],
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        let text = &self.text(parent.file).unwrap();
        self.trace_factory().loop_frame_subtraces(
            compile_time,
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
        kind: TraceKind<'eval>,
    ) -> Arc<Trace<'eval>> {
        self.trace_factory()
            .new_trace(parent_id, indent, kind, &self.text(file).unwrap())
    }

    fn trace(&self, id: TraceId) -> Arc<Trace<'eval>> {
        self.trace_factory()
            .traces
            .read(|traces| traces[id.0].as_ref().unwrap().clone())
    }

    fn clear(&self) {
        self.trace_factory().traces.write(|traces| traces.clear())
    }
}
