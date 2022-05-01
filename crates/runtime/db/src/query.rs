use datasets::LabeledData;
use defn_head::InputPlaceholder;
use feature::*;
use semantics_eager::ProcStmtVariant;
use semantics_entity::EntityDefnVariant;
use upcast::Upcast;
use visual_runtime::VisualQueryGroup;
use vm::{exec_debug, EvalResult, HistoryEntry, InterpreterQueryGroup};

use trace::*;

use crate::*;

pub trait EvalFeature {
    fn feature_query_group(&self) -> &dyn FeatureQueryGroup;
    fn session(&self) -> &Arc<Mutex<Session<'static>>>;

    fn eval_feature_block(&self, block: &FeatureBlock, input_id: usize) -> EvalResult<'static> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_block(self.feature_query_group(), block, input, sheet)
    }

    fn eval_feature_stmt(&self, stmt: &FeatureStmt, input_id: usize) -> EvalResult<'static> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_stmt(self.feature_query_group(), stmt, input, sheet)
    }

    fn eval_feature_expr(&self, expr: &FeatureExpr, input_id: usize) -> EvalResult<'static> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_expr(self.feature_query_group(), expr, input, sheet)
    }
}

#[salsa::query_group(RuntimeQueryGroupStorage)]
pub trait RuntimeQueryGroup:
    AskCompileTime + CreateTrace<'static> + EvalFeature + VisualQueryGroup
{
    #[salsa::input]
    fn pack_main(&self) -> FilePtr;

    #[salsa::input]
    fn version(&self) -> usize;

    fn subtraces(
        &self,
        trace_id: TraceId,
        effective_opt_input_id: Option<usize>,
    ) -> Arc<Vec<Arc<Trace<'static>>>>;
    fn root_traces(&self) -> Arc<Vec<TraceId>>;

    fn trace_stalk(&self, trace_id: TraceId, input_id: usize) -> Arc<TraceStalk<'static>>;
}

pub fn root_traces(this: &dyn RuntimeQueryGroup) -> Arc<Vec<TraceId>> {
    let compile_time = this.compile_time(this.version());
    let pack_main = this.pack_main();
    Arc::new(vec![this
        .new_trace(
            None,
            pack_main,
            0,
            TraceVariant::Main(compile_time.main_feature_block(pack_main).unwrap()),
        )
        .id()])
}

pub fn subtraces(
    db: &dyn RuntimeQueryGroup,
    trace_id: TraceId,
    effective_opt_input_id: Option<usize>,
) -> Arc<Vec<Arc<Trace<'static>>>> {
    let trace: &Trace = &db.trace(trace_id);
    match trace.variant {
        TraceVariant::Main(ref block) => db.feature_block_subtraces(&trace, block),
        TraceVariant::FeatureStmt(_)
        | TraceVariant::FeatureCallInput { .. }
        | TraceVariant::FuncStmt { .. }
        | TraceVariant::CallHead { .. } => Arc::new(vec![]),
        TraceVariant::ProcStmt {
            ref stmt,
            ref history,
        } => match stmt.variant {
            ProcStmtVariant::Init { .. }
            | ProcStmtVariant::Assert { .. }
            | ProcStmtVariant::Execute { .. }
            | ProcStmtVariant::Return { .. } => Arc::new(vec![]),
            ProcStmtVariant::BranchGroup { .. } => panic!(),
            ProcStmtVariant::Loop {
                loop_variant: ref loop_kind,
                ref stmts,
            } => match history.entry(stmt) {
                HistoryEntry::NonVoidExpr { .. } | HistoryEntry::Exec { .. } => Arc::new(vec![]),
                HistoryEntry::Loop {
                    control: result,
                    ref stack_snapshot,
                    ref body,
                    ..
                } => db.loop_subtraces(
                    db.compile_time(trace.compile_time_version),
                    trace,
                    loop_kind,
                    stmt,
                    stmts,
                    stack_snapshot,
                    body,
                ),
            },
        },
        TraceVariant::FeatureExpr(ref expr) => {
            feature_expr_subtraces(db, trace, expr, effective_opt_input_id)
        }
        TraceVariant::FeatureBranch(ref branch) => db.feature_branch_subtraces(trace, branch),
        TraceVariant::EagerExpr { .. } => todo!(),
        TraceVariant::LoopFrame {
            loop_frame_data: ref vm_loop_frame,
            ref body_stmts,
            ref body_instruction_sheet,
            ..
        } => db.loop_frame_subtraces(
            db.compile_time(trace.compile_time_version),
            trace,
            vm_loop_frame,
            body_instruction_sheet,
            body_stmts,
        ),
    }
}

fn feature_expr_subtraces(
    db: &dyn RuntimeQueryGroup,
    parent: &Trace,
    expr: &FeatureExpr,
    opt_input_id: Option<usize>,
) -> Arc<Vec<Arc<Trace<'static>>>> {
    Arc::new(match expr.kind {
        FeatureExprKind::PrimitiveLiteral(_)
        | FeatureExprKind::PrimitiveBinaryOpr { .. }
        | FeatureExprKind::Variable { .. } => vec![],
        FeatureExprKind::RoutineCall {
            ref instruction_sheet,
            ref routine_defn,
            ref opds,
            ..
        } => {
            if let Some(input_id) = opt_input_id {
                let mut subtraces = vec![];
                let mut func_input_values = vec![];
                subtraces.push(
                    db.trace_factory()
                        .new_call_head(routine_defn.clone(), &db.text(routine_defn.file).unwrap()),
                );
                let input_placeholders: &[InputPlaceholder] = match routine_defn.variant {
                    EntityDefnVariant::Func {
                        ref input_placeholders,
                        ..
                    } => input_placeholders,
                    EntityDefnVariant::Proc {
                        ref input_placeholders,
                        ..
                    } => input_placeholders,
                    _ => panic!(),
                };
                for (i, func_input) in opds.iter().enumerate() {
                    subtraces.push(db.new_trace(
                        Some(parent.id()),
                        expr.expr.file,
                        4,
                        TraceVariant::FeatureCallInput {
                            input: func_input.clone(),
                            ident: input_placeholders[i].ident.ident,
                        },
                    ));
                    match db.eval_feature_expr(func_input, input_id) {
                        Ok(value) => func_input_values.push(value.into_stack().unwrap()),
                        Err(_) => return Arc::new(subtraces),
                    }
                }
                let history = exec_debug(
                    db.compile_time(parent.compile_time_version),
                    func_input_values,
                    instruction_sheet,
                );
                match routine_defn.variant {
                    EntityDefnVariant::Func { ref stmts, .. } => {
                        subtraces.extend(db.trace_factory().func_stmts_traces(
                            parent.id(),
                            4,
                            stmts,
                            &db.text(routine_defn.file).unwrap(),
                            &history,
                        ));
                    }
                    EntityDefnVariant::Proc { ref stmts, .. } => {
                        subtraces.extend(db.trace_factory().proc_stmts_traces(
                            parent.id(),
                            4,
                            stmts,
                            &db.text(routine_defn.file).unwrap(),
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

        FeatureExprKind::EntityFeature { .. } => todo!(),
        FeatureExprKind::NewRecord { ty, ref opds, .. } => todo!(),
        FeatureExprKind::RecordOriginalFieldAccess {
            ref this,
            field_ident,
            ..
        } => todo!(),
        FeatureExprKind::This { ref repr } => todo!(),
        FeatureExprKind::PatternCall {} => todo!(),
        FeatureExprKind::RecordDerivedFieldAccess { .. } => todo!(),
        FeatureExprKind::StructOriginalFieldAccess { .. } => panic!(),
        FeatureExprKind::EnumLiteral { .. } => panic!(),
        FeatureExprKind::GlobalInput => panic!(),
        FeatureExprKind::ElementAccess { ref opds, .. } => panic!(),
    })
}

pub fn trace_stalk(
    this: &dyn RuntimeQueryGroup,
    trace_id: TraceId,
    input_id: usize,
) -> Arc<TraceStalk<'static>> {
    let trace: &Trace = &this.trace(trace_id);
    Arc::new(match trace.variant {
        TraceVariant::Main(ref block) => TraceStalk {
            extra_tokens: vec![
                trace::fade!(" = "),
                this.eval_feature_block(block, input_id).into(),
            ],
        },
        TraceVariant::FeatureStmt(ref stmt) => match stmt.kind {
            FeatureStmtKind::Init { varname, ref value } => TraceStalk {
                extra_tokens: vec![
                    trace::fade!(" = "),
                    this.eval_feature_expr(value, input_id).into(),
                ],
            },
            FeatureStmtKind::Assert { ref condition } => TraceStalk {
                extra_tokens: vec![
                    trace::fade!(" = "),
                    this.eval_feature_expr(condition, input_id).into(),
                ],
            },
            FeatureStmtKind::Return { ref result } => TraceStalk {
                extra_tokens: vec![
                    trace::fade!(" = "),
                    this.eval_feature_expr(result, input_id).into(),
                ],
            },
            FeatureStmtKind::BranchGroup { kind, ref branches } => panic!(),
        },
        TraceVariant::FeatureBranch(_) => TraceStalk {
            extra_tokens: vec![],
        },
        TraceVariant::FeatureExpr(ref expr) => TraceStalk {
            extra_tokens: vec![
                trace::fade!(" = "),
                this.eval_feature_expr(expr, input_id).into(),
            ],
        },
        TraceVariant::FeatureCallInput { .. } => todo!(),
        TraceVariant::FuncStmt { .. }
        | TraceVariant::ProcStmt { .. }
        | TraceVariant::EagerExpr { .. }
        | TraceVariant::CallHead { .. } => panic!(),
        TraceVariant::LoopFrame {
            loop_frame_data: ref vm_loop_frame,
            ..
        } => match vm_loop_frame.control {
            vm::ControlSnapshot::None => TraceStalk::default(),
            vm::ControlSnapshot::Return(_) => todo!(),
            vm::ControlSnapshot::Break => todo!(),
            vm::ControlSnapshot::Err(_) => todo!(),
        },
    })
}
