mod call_head;
mod eager_expr;
mod expr;
mod feature_block;
mod feature_branch;
mod feature_expr;
mod feature_repr;
mod feature_stmt;
mod func_stmt;
mod internal;
mod proc_stmt;

use avec::Avec;
use compile_time_db::{AskCompileTime, HuskyCompileTime};
use defn_head::Parameter;
use eval_feature::EvalFeature;
use expr::ExprTokenConfig;
use feature::*;
use internal::*;
use runtime_db::AskRuntime;
use semantics_eager::*;
use serde::Deserialize;
use std::collections::HashMap;
use sync_utils::ARwLock;
use text::{Text, TextQueryGroup};
use upcast::Upcast;
use vm::{
    exec_debug, History, HistoryEntry, InstructionSheet, LoopFrameData, StackSnapshot, VMLoopKind,
    VariableStack,
};

use crate::*;

#[derive(Debug, Default)]
pub struct TraceFactory {
    entries: ARwLock<TraceFactoryInternal>,
    compile_time_version: usize,
}

#[derive(Debug, Default)]
pub struct TraceFactoryInternal {
    traces: Vec<Option<TraceNode>>,
    expansions: HashMap<TraceId, bool>,
    showns: HashMap<TraceId, bool>,
}

impl TraceFactory {
    pub fn traces(&self) -> Vec<TraceProps> {
        self.entries.read(|internal| {
            internal
                .traces
                .iter()
                .filter_map(|opt_trace| opt_trace.as_ref().map(|node| node.trace.props.clone()))
                .collect()
        })
    }

    pub(crate) fn next_id(&self) -> TraceId {
        TraceId(self.entries.write(|internal| {
            internal.traces.push(None);
            internal.traces.len() - 1
        }))
    }

    pub(crate) fn lines(
        &self,
        id: TraceId,
        indent: Indent,
        variant: &TraceVariant,
        text: &Text,
        has_parent: bool,
    ) -> Vec<LineProps> {
        match variant {
            TraceVariant::Main(feature_block) => vec![LineProps {
                indent,
                idx: 0,
                tokens: vec![TraceTokenProps {
                    kind: TraceTokenKind::Keyword,
                    value: "main".into(),
                    opt_associated_trace_id: None,
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
        variant: TraceVariant<'static>,
        text: &Text,
    ) -> Arc<Trace> {
        let trace = Arc::new(Trace::new(
            opt_parent_id,
            indent,
            variant,
            self,
            text,
            self.compile_time_version,
        ));
        self.entries.write(|internal| {
            assert!(internal.traces[trace.id().0].is_none());
            internal.traces[trace.id().0] = Some(TraceNode {
                expansion: false,
                shown: match trace.props.kind {
                    TraceKind::FeatureExpr | TraceKind::EagerExpr => false,
                    _ => true,
                },
                trace: trace.clone(),
            });
        });
        trace
    }

    pub fn toggle_expansion(&mut self, id: TraceId) {
        let expanded = self.expansions.entry(id).or_insert(false);
        *expanded = !*expanded;
    }

    pub fn is_expanded(&mut self, trace: &Trace) -> bool {
        *self.expansions.entry(trace.id()).or_insert(false)
    }

    pub fn expansions(&self) -> HashMap<TraceId, bool> {}

    pub fn toggle_show(&mut self, id: TraceId) {
        let shown = self.showns.entry(id).or_insert(false);
        *shown = !*shown;
    }

    pub fn showns(&self) -> &HashMap<TraceId, bool> {
        &self.showns
    }
}

pub trait ProduceTrace: AskRuntime {
    fn trace_factory(&self) -> &TraceFactory;

    fn feature_repr_subtraces(
        &self,
        parent: &Trace,
        feature_repr: &FeatureRepr,
    ) -> Arc<Vec<Arc<Trace>>> {
        let text = &self.compile_time().text(parent.file).unwrap();
        Arc::new(
            self.trace_factory()
                .feature_repr_subtraces(parent, feature_repr, text),
        )
    }

    fn feature_block_subtraces(
        &self,
        parent: &Trace,
        feature_block: &FeatureLazyBlock,
    ) -> Arc<Vec<Arc<Trace>>> {
        let text = &self.compile_time().text(parent.file).unwrap();
        Arc::new(
            self.trace_factory()
                .feature_lazy_block_subtraces(parent, feature_block, text),
        )
    }

    fn feature_branch_subtraces(
        &self,
        parent: &Trace,
        branch: &FeatureBranch,
    ) -> Arc<Vec<Arc<Trace>>> {
        let text = &self.compile_time().text(parent.file).unwrap();
        self.trace_factory()
            .feature_branch_subtraces(parent, branch, self.trace_factory(), text)
    }

    fn feature_expr_subtraces(
        &self,
        parent: &Trace,
        expr: &FeatureExpr,
        opt_input_id: Option<usize>,
    ) -> Arc<Vec<Arc<Trace>>> {
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
                        match self.runtime().eval_feature_expr(func_input, input_id) {
                            Ok(value) => func_input_values.push(value.into_stack().unwrap()),
                            Err(_) => return Arc::new(subtraces),
                        }
                    }
                    let history = exec_debug(
                        self.runtime().upcast(),
                        instruction_sheet,
                        func_input_values.into_iter(),
                        self.runtime().verbose(),
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
        parent: &Trace,
        expr: &Arc<EagerExpr>,
        history: &Arc<History>,
    ) -> Avec<Trace> {
        todo!()
        // self.trace_factory()
    }

    fn loop_subtraces(
        &self,
        parent: &Trace,
        loop_kind: VMLoopKind,
        loop_stmt: &Arc<ProcStmt>,
        stmts: &Arc<Vec<Arc<ProcStmt>>>,
        stack_snapshot: &StackSnapshot<'static>,
        body_instruction_sheet: &Arc<InstructionSheet>,
        verbose: bool,
    ) -> Arc<Vec<Arc<Trace>>> {
        self.trace_factory().loop_subtraces(
            self.runtime().upcast(),
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
        loop_frame_data: &LoopFrameData<'static>,
        parent: &Trace,
        verbose: bool,
    ) -> Avec<Trace> {
        let text = &self.compile_time().text(parent.file).unwrap();
        self.trace_factory().loop_frame_subtraces(
            self.runtime().upcast(),
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
        stack_snapshot: &StackSnapshot<'static>,
        parent: &Trace,
        verbose: bool,
    ) -> Avec<Trace> {
        let text = &self.compile_time().text(parent.file).unwrap();
        self.trace_factory().proc_branch_subtraces(
            self.runtime().upcast(),
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
        kind: TraceVariant<'static>,
    ) -> Arc<Trace> {
        self.trace_factory().new_trace(
            parent_id,
            indent,
            kind,
            &self.compile_time().text(file).unwrap(),
        )
    }

    fn trace(&self, id: TraceId) -> Arc<Trace> {
        self.trace_factory()
            .entries
            .read(|internal| internal.traces[id.0].as_ref().unwrap().trace.clone())
    }

    fn clear(&self) {
        self.trace_factory()
            .entries
            .write(|internal| internal.clear())
    }
}
