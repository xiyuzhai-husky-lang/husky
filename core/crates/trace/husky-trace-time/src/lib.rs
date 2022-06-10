mod impl_call_head;
mod impl_eager_expr;
mod impl_expr;
mod impl_feature_block;
mod impl_feature_branch;
mod impl_feature_expr;
mod impl_feature_repr;
mod impl_feature_stmt;
mod impl_figure;
mod impl_figure_control;
mod impl_func_stmt;
mod impl_ops;
mod impl_proc_stmt;
mod impl_trace_stalk;
mod node;

use avec::Avec;
use defn_head::Parameter;
use eval_feature::EvalFeature;
use feature_gen::*;
use file::FilePtr;
use husky_compile_time::{AskCompileTime, HuskyCompileTime};
use husky_debugger_protocol::*;
use husky_runtime::HuskyRuntime;
use impl_expr::ExprTokenConfig;
use node::*;
use semantics_eager::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use sync_utils::ARwLock;
use text::{Text, TextQueryGroup};
use trace::*;
use upcast::Upcast;
use vm::*;
use wild_utils::ref_to_mut_ref;

pub struct HuskyTraceTime {
    runtime: HuskyRuntime,
    focus: Focus,
    trace_nodes: Vec<Option<TraceNode>>,
    opt_active_trace_id: Option<TraceId>,
    trace_stalks: HashMap<TraceStalkKey, TraceStalk>,
    root_traces: Vec<TraceId>,
    subtraces_map: HashMap<SubtracesKey, Vec<TraceId>>,
    figures: HashMap<FigureKey, FigureProps>,
    figure_controls: HashMap<FigureControlKey, FigureControlProps>,
}

impl HuskyTraceTime {
    pub fn new(init_compile_time: impl FnOnce(&mut HuskyCompileTime), verbose: bool) -> Self {
        let mut trace_time = Self {
            runtime: HuskyRuntime::new(init_compile_time, verbose),
            trace_nodes: Default::default(),
            trace_stalks: Default::default(),
            opt_active_trace_id: Default::default(),
            subtraces_map: Default::default(),
            figures: Default::default(),
            figure_controls: Default::default(),
            root_traces: Default::default(),
            focus: Default::default(),
        };
        trace_time.update();
        trace_time
    }

    pub fn root_traces(&self) -> Vec<TraceId> {
        self.root_traces.clone()
    }

    pub fn lock_input(&mut self, command: &str) -> (Option<Option<usize>>, Option<String>) {
        if command.len() == 0 {
            return (Some(None), None);
        }
        match command.parse::<usize>() {
            Ok(id) => {
                self.focus = Focus {
                    opt_input_id: Some(id),
                };
                (Some(Some(id)), None)
            }
            Err(e) => (None, Some(format!("lock input failed due to error: {}", e))),
        }
    }

    pub fn all_traces(&self) -> Vec<TraceProps> {
        self.trace_nodes
            .iter()
            .filter_map(|opt_trace| opt_trace.as_ref().map(|node| node.trace.props.clone()))
            .collect()
    }

    pub fn subtrace_ids(&mut self, trace_id: TraceId) -> &[TraceId] {
        todo!()
    }

    pub fn subtraces(&mut self, trace_id: TraceId) -> Vec<TraceProps> {
        todo!()
    }

    pub fn focus(&self) -> Focus {
        todo!()
    }

    pub(crate) fn next_id(&mut self) -> TraceId {
        self.trace_nodes.push(None);
        TraceId(self.trace_nodes.len() - 1)
    }

    pub(crate) fn lines(
        &mut self,
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
        &mut self,
        opt_parent_id: Option<TraceId>,
        indent: Indent,
        variant: TraceVariant<'static>,
        text: &Text,
    ) -> Arc<Trace> {
        let trace = Arc::new({
            let id = self.next_id();
            let (file, range) = variant.file_and_range();
            let reachable = variant.reachable();
            let has_subtraces = variant.has_subtraces(reachable);
            let lines = self.lines(id, indent, &variant, text, opt_parent_id.is_some());
            Trace {
                props: TraceProps {
                    id,
                    opt_parent_id,
                    indent,
                    compile_time_version: 0, //compile time version
                    has_subtraces,
                    reachable,
                    lines,
                    kind: variant.kind(),
                },
                variant,
                file,
                range,
            }
        });
        assert!(self.trace_nodes[trace.id().0].is_none());
        self.trace_nodes[trace.id().0] = Some(TraceNode {
            expansion: false,
            shown: match trace.props.kind {
                TraceKind::FeatureExpr | TraceKind::EagerExpr => false,
                _ => true,
            },
            trace: trace.clone(),
        });
        trace
    }

    pub fn toggle_expansion(&mut self, id: TraceId) {
        todo!()
        // let expanded = self.expansions.entry(id).or_insert(false);
        // *expanded = !*expanded;
    }

    pub fn is_expanded(&mut self, trace: &Trace) -> bool {
        todo!()
        // * self.expansions.entry(trace.id()).or_insert(false)
    }

    pub fn expansions(&self) -> HashMap<TraceId, bool> {
        todo!()
    }

    pub fn toggle_show(&mut self, id: TraceId) {
        todo!()
        // let shown = self.showns.entry(id).or_insert(false);
        // *shown = !*shown;
    }

    pub fn showns(&self) -> &HashMap<TraceId, bool> {
        todo!()
        // &self.showns
    }

    pub fn trace(&self, trace_id: TraceId) -> &Trace {
        &self.trace_nodes[trace_id.0].as_ref().unwrap().trace
    }

    pub fn init_state(&mut self) -> DebuggerServerMessageVariant {
        let root_traces = self.root_traces.clone();
        let expansions = self.expansions().clone();
        let showns = self.showns().clone();
        let focus = self.focus();
        let mut figures = HashMap::default();
        let mut figure_controls = HashMap::default();
        let opt_active_trace_id = self.opt_active_trace_id;
        if let Some(active_trace_id) = opt_active_trace_id {
            let active_trace = self.trace(active_trace_id);
            figures.insert(
                FigureKey::new(&active_trace.props),
                self.figure(active_trace_id, &focus),
            );
            figure_controls.insert(
                FigureControlKey::new(&active_trace.props),
                unsafe { ref_to_mut_ref(self) }.figure_control(&active_trace, &focus),
            );
        }
        let traces = self.all_traces();
        DebuggerServerMessageVariant::Init {
            init_data: InitData {
                trace_init_data: TraceInitState {
                    active_trace_id: opt_active_trace_id,
                    traces,
                    subtraces_map: self.subtraces_map.clone(),
                    root_traces,
                    expansions,
                    showns,
                },
                focus,
                figures,
                figure_controls,
            },
        }
    }
}

// pub trait ProduceTrace: AskRuntime {
//     fn trace_factory(&self) -> &TraceFactory;

//     fn feature_repr_subtraces(
//         &self,
//         parent: &Trace,
//         feature_repr: &FeatureRepr,
//     ) -> Arc<Vec<Arc<Trace>>> {
//         let text = &self.compile_time().text(parent.file).unwrap();
//         Arc::new(
//             self.trace_factory()
//                 .feature_repr_subtraces(parent, feature_repr, text),
//         )
//     }

//     fn feature_block_subtraces(
//         &self,
//         parent: &Trace,
//         feature_block: &FeatureLazyBlock,
//     ) -> Arc<Vec<Arc<Trace>>> {
//         let text = &self.compile_time().text(parent.file).unwrap();
//         Arc::new(
//             self.trace_factory()
//                 .feature_lazy_block_subtraces(parent, feature_block, text),
//         )
//     }

//     fn feature_branch_subtraces(
//         &self,
//         parent: &Trace,
//         branch: &FeatureBranch,
//     ) -> Arc<Vec<Arc<Trace>>> {
//         let text = &self.compile_time().text(parent.file).unwrap();
//         self.trace_factory()
//             .feature_branch_subtraces(parent, branch, self.trace_factory(), text)
//     }

//     fn feature_expr_subtraces(
//         &self,
//         parent: &Trace,
//         expr: &FeatureExpr,
//         opt_input_id: Option<usize>,
//     ) -> Arc<Vec<Arc<Trace>>> {
//         Arc::new(match expr.variant {
//             FeatureExprVariant::PrimitiveLiteral(_)
//             | FeatureExprVariant::PrimitiveBinaryOpr { .. }
//             | FeatureExprVariant::Variable { .. } => vec![],
//             FeatureExprVariant::RoutineCall {
//                 ref opt_instruction_sheet,
//                 ref routine_defn,
//                 ref opds,
//                 has_this,
//                 ..
//             } => {
//                 let instruction_sheet: &InstructionSheet = opt_instruction_sheet.as_ref().unwrap();
//                 if let Some(input_id) = opt_input_id {
//                     let mut subtraces = vec![];
//                     let mut func_input_values = vec![];
//                     subtraces.push(self.trace_factory().new_call_head(
//                         routine_defn.clone(),
//                         &self.compile_time().text(routine_defn.file).unwrap(),
//                     ));
//                     let parameters: &[Parameter] = match routine_defn.variant {
//                         EntityDefnVariant::Func { ref parameters, .. } => parameters,
//                         EntityDefnVariant::Proc {
//                             parameters: ref parameters,
//                             ..
//                         } => parameters,
//                         _ => panic!(),
//                     };
//                     for (i, func_input) in opds.iter().enumerate() {
//                         subtraces.push(self.new_trace(
//                             Some(parent.id()),
//                             expr.expr.file,
//                             4,
//                             TraceVariant::FeatureCallInput {
//                                 input: func_input.clone(),
//                                 ident: parameters[i].ranged_ident.ident,
//                             },
//                         ));
//                         match self.runtime.eval_feature_expr(func_input, input_id) {
//                             Ok(value) => func_input_values.push(value.into_stack().unwrap()),
//                             Err(_) => return Arc::new(subtraces),
//                         }
//                     }
//                     let history = exec_debug(
//                         self.runtime.upcast(),
//                         instruction_sheet,
//                         func_input_values.into_iter(),
//                         self.runtime.verbose(),
//                     );
//                     match routine_defn.variant {
//                         EntityDefnVariant::Func { ref stmts, .. } => {
//                             subtraces.extend(self.trace_factory().func_stmts_traces(
//                                 parent.id(),
//                                 4,
//                                 stmts,
//                                 &self.compile_time().text(routine_defn.file).unwrap(),
//                                 &history,
//                             ));
//                         }
//                         EntityDefnVariant::Proc { ref stmts, .. } => {
//                             subtraces.extend(self.trace_factory().proc_stmts_traces(
//                                 parent.id(),
//                                 4,
//                                 stmts,
//                                 &self.compile_time().text(routine_defn.file).unwrap(),
//                                 &history,
//                             ));
//                         }
//                         _ => panic!(),
//                     }
//                     subtraces
//                 } else {
//                     vec![]
//                 }
//             }
//             FeatureExprVariant::EntityFeature { .. } => todo!(),
//             FeatureExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
//             FeatureExprVariant::RecordOriginalFieldAccess {
//                 ref this,
//                 field_ident,
//                 ..
//             } => todo!(),
//             FeatureExprVariant::ThisValue { ref repr } => todo!(),
//             FeatureExprVariant::PatternCall {} => todo!(),
//             FeatureExprVariant::RecordDerivedFieldAccess { .. } => todo!(),
//             FeatureExprVariant::StructOriginalFieldAccess { .. } => panic!(),
//             FeatureExprVariant::EnumKindLiteral { .. } => panic!(),
//             FeatureExprVariant::EvalInput => panic!(),
//             FeatureExprVariant::ElementAccess { ref opds, .. } => panic!(),
//             FeatureExprVariant::StructDerivedLazyFieldAccess {
//                 ref this,
//                 field_ident,
//                 ref repr,
//             } => todo!(),
//         })
//     }

//     fn eager_expr_subtraces(
//         &self,
//         parent: &Trace,
//         expr: &Arc<EagerExpr>,
//         history: &Arc<History>,
//     ) -> Avec<Trace> {
//         todo!()
//         // self.trace_factory()
//     }

//     fn loop_subtraces(
//         &self,
//         parent: &Trace,
//         loop_kind: VMLoopKind,
//         loop_stmt: &Arc<ProcStmt>,
//         stmts: &Arc<Vec<Arc<ProcStmt>>>,
//         stack_snapshot: &StackSnapshot<'static>,
//         body_instruction_sheet: &Arc<InstructionSheet>,
//         verbose: bool,
//     ) -> Arc<Vec<Arc<Trace>>> {
//         self.trace_factory().loop_subtraces(
//             self.runtime.upcast(),
//             parent,
//             loop_kind,
//             loop_stmt,
//             stmts,
//             stack_snapshot,
//             body_instruction_sheet,
//             verbose,
//         )
//     }

//     fn loop_frame_subtraces(
//         &self,
//         loop_stmt: &Arc<ProcStmt>,
//         stmts: &[Arc<ProcStmt>],
//         instruction_sheet: &InstructionSheet,
//         loop_frame_data: &LoopFrameData<'static>,
//         parent: &Trace,
//         verbose: bool,
//     ) -> Avec<Trace> {
//         let text = &self.compile_time().text(parent.file).unwrap();
//         self.trace_factory().loop_frame_subtraces(
//             self.runtime.upcast(),
//             text,
//             loop_stmt,
//             stmts,
//             instruction_sheet,
//             loop_frame_data,
//             parent,
//             verbose,
//         )
//     }

//     fn proc_branch_subtraces(
//         &self,
//         stmts: &[Arc<ProcStmt>],
//         instruction_sheet: &InstructionSheet,
//         stack_snapshot: &StackSnapshot<'static>,
//         parent: &Trace,
//         verbose: bool,
//     ) -> Avec<Trace> {
//         let text = &self.compile_time().text(parent.file).unwrap();
//         self.trace_factory().proc_branch_subtraces(
//             self.runtime.upcast(),
//             text,
//             stmts,
//             instruction_sheet,
//             stack_snapshot,
//             parent,
//             verbose,
//         )
//     }

//     fn new_trace(
//         &self,
//         parent_id: Option<TraceId>,
//         file: FilePtr,
//         indent: Indent,
//         kind: TraceVariant<'static>,
//     ) -> Arc<Trace> {
//         self.trace_factory().new_trace(
//             parent_id,
//             indent,
//             kind,
//             &self.compile_time().text(file).unwrap(),
//         )
//     }
// }
