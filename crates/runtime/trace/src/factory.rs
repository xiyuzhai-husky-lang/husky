mod call_head;
mod eager_expr;
mod expr;
mod feature_block;
mod feature_branch;
mod feature_expr;
mod feature_stmt;
mod func_stmt;
mod proc_stmt;

use avec::Avec;
use compile_time_db::{AskCompileTime, HuskyLangCompileTime};
use expr::ExprTokenConfig;
use feature::*;
use semantics_eager::*;
use serde::Deserialize;
use sync_utils::ARwLock;
use text::{Text, TextQueryGroup};
use vm::{History, InstructionSheet, LoopFrameData, StackSnapshot, VMLoopKind, VariableStack};

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
    compile_time_version: usize,
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
        variant: &TraceVariant<'eval>,
        text: &Text,
        has_parent: bool,
    ) -> Vec<LineProps<'eval>> {
        match variant {
            TraceVariant::Main(feature_block) => vec![LineProps {
                indent,
                idx: 0,
                tokens: vec![TokenProps {
                    kind: TraceTokenKind::Keyword,
                    value: Cow::Borrowed("main"),
                    associated_trace: None,
                }],
            }],
            TraceVariant::FeatureStmt(stmt) => self.feature_stmt_lines(stmt, text),
            TraceVariant::FeatureExpr(expr) => {
                self.feature_expr_lines(expr, text, ExprTokenConfig::expr(false))
            }
            TraceVariant::FeatureBranch(branch) => self.feature_branch_lines(indent, branch, text),
            TraceVariant::FeatureCallInput { ident, input } => {
                let mut lines = self.feature_expr_lines(input, text, ExprTokenConfig::expr(true));
                lines[0].tokens.insert(0, special!(" = "));
                lines[0].tokens.insert(0, ident!(ident.0));
                lines
            }
            TraceVariant::FuncStmt { .. } => todo!(),
            TraceVariant::ProcStmt {
                ref stmt,
                ref history,
            } => self.proc_stmt_lines(stmt, text, history),
            TraceVariant::EagerExpr {
                ref expr,
                ref history,
            } => self.eager_expr_lines(
                text,
                expr,
                history,
                indent,
                ExprTokenConfig::expr(has_parent),
            ),
            TraceVariant::CallHead { ref tokens, .. } => vec![LineProps {
                indent: 0,
                idx: 0,
                tokens: tokens.clone(),
            }],
            TraceVariant::LoopFrame {
                loop_frame_data: ref vm_loop_frame,
                ..
            } => self.loop_frame_lines(indent, vm_loop_frame),
            TraceVariant::ProcBranch {
                stmt,
                branch,
                branch_idx,
                history,
                ..
            } => self.proc_branch_lines(text, indent, branch, history),
        }
    }

    fn new_trace(
        &self,
        opt_parent_id: Option<TraceId>,
        indent: Indent,
        kind: TraceVariant<'eval>,
        text: &Text,
    ) -> Arc<Trace<'eval>> {
        let trace = Arc::new(Trace::new(
            opt_parent_id,
            indent,
            kind,
            self,
            text,
            self.compile_time_version,
        ));
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

pub trait CreateTrace<'eval>: AskCompileTime {
    fn trace_factory(&self) -> &TraceFactory<'eval>;

    fn feature_block_subtraces(
        &self,
        parent: &Trace<'eval>,
        feature_block: &FeatureBlock,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        let text = &self.compile_time().text(parent.file).unwrap();
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
        let text = &self.compile_time().text(parent.file).unwrap();
        self.trace_factory()
            .feature_branch_subtraces(parent, branch, self.trace_factory(), text)
    }

    fn eager_expr_subtraces(
        &self,
        parent: &Trace<'eval>,
        expr: &Arc<EagerExpr>,
        history: &Arc<History<'eval>>,
    ) -> Avec<Trace<'eval>> {
        match expr.variant {
            EagerExprVariant::Variable(_) => todo!(),
            EagerExprVariant::This => todo!(),
            EagerExprVariant::EntityRoute { route } => todo!(),
            EagerExprVariant::PrimitiveLiteral(_) => todo!(),
            EagerExprVariant::Bracketed(_) => todo!(),
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => todo!(),
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::EnumKindLiteral(_) => todo!(),
        }
    }

    fn loop_subtraces(
        &self,
        compile_time: &HuskyLangCompileTime,
        parent: &Trace,
        loop_kind: VMLoopKind,
        loop_stmt: &Arc<ProcStmt>,
        stmts: &Arc<Vec<Arc<ProcStmt>>>,
        stack_snapshot: &StackSnapshot<'eval>,
        body_instruction_sheet: &Arc<InstructionSheet>,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        let text = &self.compile_time().text(parent.file).unwrap();
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
        loop_stmt: &Arc<ProcStmt>,
        stmts: &[Arc<ProcStmt>],
        instruction_sheet: &InstructionSheet,
        loop_frame_data: &LoopFrameData<'eval>,
        parent: &Trace,
    ) -> Avec<Trace<'eval>> {
        let text = &self.compile_time().text(parent.file).unwrap();
        self.trace_factory().loop_frame_subtraces(
            self.compile_time(),
            text,
            loop_stmt,
            stmts,
            instruction_sheet,
            loop_frame_data,
            parent,
        )
    }

    fn proc_branch_subtraces(
        &self,
        stmts: &[Arc<ProcStmt>],
        instruction_sheet: &InstructionSheet,
        stack_snapshot: &StackSnapshot<'eval>,
        parent: &Trace,
    ) -> Avec<Trace<'eval>> {
        let text = &self.compile_time().text(parent.file).unwrap();
        self.trace_factory().proc_branch_subtraces(
            self.compile_time(),
            text,
            stmts,
            instruction_sheet,
            stack_snapshot,
            parent,
        )
    }

    fn new_trace(
        &self,
        parent_id: Option<TraceId>,
        file: FilePtr,
        indent: Indent,
        kind: TraceVariant<'eval>,
    ) -> Arc<Trace<'eval>> {
        self.trace_factory().new_trace(
            parent_id,
            indent,
            kind,
            &self.compile_time().text(file).unwrap(),
        )
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
