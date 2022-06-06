mod call_head;
mod eager_expr;
mod expr;
mod feature_block;
mod feature_branch;
mod feature_expr;
mod feature_repr;
mod feature_stmt;
mod func_stmt;
mod proc_stmt;

use avec::Avec;
use compile_time_db::{AskCompileTime, HuskyCompileTime};
use defn_head::Parameter;
use eval_feature::EvalFeature;
use expr::ExprTokenConfig;
use feature::*;
use semantics_eager::*;
use serde::Deserialize;
use sync_utils::ARwLock;
use text::{Text, TextQueryGroup};
use vm::{
    exec_debug, History, HistoryEntry, InstructionSheet, LoopFrameData, StackSnapshot, VMLoopKind,
    VariableStack,
};

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
            TraceVariant::FuncStmt {
                ref stmt,
                ref history,
                ..
            } => self.func_stmt_lines(stmt, text, history),
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

pub trait ProduceTrace<'eval>: AskCompileTime + EvalFeature<'eval> {
    fn trace_factory(&self) -> &TraceFactory<'eval>;

    fn feature_repr_subtraces(
        &self,
        parent: &Trace<'eval>,
        feature_repr: &FeatureRepr,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        let text = &self.compile_time().text(parent.file).unwrap();
        Arc::new(
            self.trace_factory()
                .feature_repr_subtraces(parent, feature_repr, text),
        )
    }

    fn feature_block_subtraces(
        &self,
        parent: &Trace<'eval>,
        feature_block: &FeatureLazyBlock,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        let text = &self.compile_time().text(parent.file).unwrap();
        Arc::new(
            self.trace_factory()
                .feature_lazy_block_subtraces(parent, feature_block, text),
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

    fn feature_expr_subtraces(
        &self,
        parent: &Trace,
        expr: &FeatureExpr,
        opt_input_id: Option<usize>,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        Arc::new(match expr.variant {
            FeatureExprVariant::PrimitiveLiteral(_)
            | FeatureExprVariant::PrimitiveBinaryOpr { .. }
            | FeatureExprVariant::Variable { .. } => vec![],
            FeatureExprVariant::RoutineCall {
                ref opt_instruction_sheet,
                ref routine_defn,
                ref opds,
                has_this,
                ..
            } => {
                let instruction_sheet: &InstructionSheet = opt_instruction_sheet.as_ref().unwrap();
                if let Some(input_id) = opt_input_id {
                    let mut subtraces = vec![];
                    let mut func_input_values = vec![];
                    subtraces.push(self.trace_factory().new_call_head(
                        routine_defn.clone(),
                        &self.compile_time().text(routine_defn.file).unwrap(),
                    ));
                    let parameters: &[Parameter] = match routine_defn.variant {
                        EntityDefnVariant::Func { ref parameters, .. } => parameters,
                        EntityDefnVariant::Proc {
                            parameters: ref parameters,
                            ..
                        } => parameters,
                        _ => panic!(),
                    };
                    for (i, func_input) in opds.iter().enumerate() {
                        subtraces.push(self.new_trace(
                            Some(parent.id()),
                            expr.expr.file,
                            4,
                            TraceVariant::FeatureCallInput {
                                input: func_input.clone(),
                                ident: parameters[i].ranged_ident.ident,
                            },
                        ));
                        match self.eval_feature_expr(func_input, input_id) {
                            Ok(value) => func_input_values.push(value.into_stack().unwrap()),
                            Err(_) => return Arc::new(subtraces),
                        }
                    }
                    let history = exec_debug(
                        self.compile_time(),
                        instruction_sheet,
                        func_input_values.into_iter(),
                        self.verbose(),
                    );
                    match routine_defn.variant {
                        EntityDefnVariant::Func { ref stmts, .. } => {
                            subtraces.extend(self.trace_factory().func_stmts_traces(
                                parent.id(),
                                4,
                                stmts,
                                &self.compile_time().text(routine_defn.file).unwrap(),
                                &history,
                            ));
                        }
                        EntityDefnVariant::Proc { ref stmts, .. } => {
                            subtraces.extend(self.trace_factory().proc_stmts_traces(
                                parent.id(),
                                4,
                                stmts,
                                &self.compile_time().text(routine_defn.file).unwrap(),
                                &history,
                            ));
                        }
                        _ => panic!(),
                    }
                    subtraces
                } else {
                    vec![]
                }
            }
            FeatureExprVariant::EntityFeature { .. } => todo!(),
            FeatureExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureExprVariant::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => todo!(),
            FeatureExprVariant::ThisValue { ref repr } => todo!(),
            FeatureExprVariant::PatternCall {} => todo!(),
            FeatureExprVariant::RecordDerivedFieldAccess { .. } => todo!(),
            FeatureExprVariant::StructOriginalFieldAccess { .. } => panic!(),
            FeatureExprVariant::EnumKindLiteral { .. } => panic!(),
            FeatureExprVariant::EvalInput => panic!(),
            FeatureExprVariant::ElementAccess { ref opds, .. } => panic!(),
            FeatureExprVariant::StructDerivedLazyFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => todo!(),
        })
    }

    fn eager_expr_subtraces(
        &self,
        parent: &Trace<'eval>,
        expr: &Arc<EagerExpr>,
        history: &Arc<History<'eval>>,
    ) -> Avec<Trace<'eval>> {
        todo!()
        // self.trace_factory()
    }

    fn loop_subtraces(
        &self,
        parent: &Trace,
        loop_kind: VMLoopKind,
        loop_stmt: &Arc<ProcStmt>,
        stmts: &Arc<Vec<Arc<ProcStmt>>>,
        stack_snapshot: &StackSnapshot<'eval>,
        body_instruction_sheet: &Arc<InstructionSheet>,
        verbose: bool,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        self.trace_factory().loop_subtraces(
            self.compile_time(),
            parent,
            loop_kind,
            loop_stmt,
            stmts,
            stack_snapshot,
            body_instruction_sheet,
            verbose,
        )
    }

    fn loop_frame_subtraces(
        &self,
        loop_stmt: &Arc<ProcStmt>,
        stmts: &[Arc<ProcStmt>],
        instruction_sheet: &InstructionSheet,
        loop_frame_data: &LoopFrameData<'eval>,
        parent: &Trace,
        verbose: bool,
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
            verbose,
        )
    }

    fn proc_branch_subtraces(
        &self,
        stmts: &[Arc<ProcStmt>],
        instruction_sheet: &InstructionSheet,
        stack_snapshot: &StackSnapshot<'eval>,
        parent: &Trace,
        verbose: bool,
    ) -> Avec<Trace<'eval>> {
        let text = &self.compile_time().text(parent.file).unwrap();
        self.trace_factory().proc_branch_subtraces(
            self.compile_time(),
            text,
            stmts,
            instruction_sheet,
            stack_snapshot,
            parent,
            verbose,
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
