use check_utils::should_eq;
use eval_feature::EvalFeature;
use vm::{ControlSnapshot, HistoryEntry};

use crate::*;

#[salsa::query_group(TraceQueryGroupStorage)]
pub trait TraceQueryGroup: ProduceTrace {
    #[salsa::input]
    fn pack_main(&self) -> FilePtr;

    #[salsa::input]
    fn version(&self) -> usize;

    fn root_traces(&self) -> Vec<TraceId>;
    fn subtraces(&self, trace_id: TraceId, effective_opt_sample_id: Option<usize>) -> Vec<TraceId>;
    fn trace_stalk(&self, trace_id: TraceId, sample_id: SampleId) -> Arc<TraceStalkData>;
}

pub fn root_traces(this: &dyn TraceQueryGroup) -> Vec<TraceId> {
    let compile_time = this.compile_time();
    let pack_main = this.pack_main();
    Arc::new()
}

pub fn subtraces(
    db: &dyn TraceQueryGroup,
    trace_id: TraceId,
    effective_opt_sample_id: Option<usize>,
) -> Vec<TraceId> {
    let trace: &Trace = &db.trace(trace_id);
    match trace.variant {
        TraceVariant::Main(ref repr) => db.feature_repr_subtraces(&trace, repr),
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
            ProcStmtVariant::ConditionFlow { .. } => panic!(),
            ProcStmtVariant::Loop { ref stmts, .. } => {
                match history
                    .get(stmt)
                    .expect("if there is no entry, there is no subtraces")
                {
                    HistoryEntry::PureExpr { .. } | HistoryEntry::Exec { .. } => Arc::new(vec![]),
                    HistoryEntry::Loop {
                        control,
                        ref stack_snapshot,
                        body_instruction_sheet: ref body,
                        loop_kind,
                        ..
                    } => db.loop_subtraces(
                        trace,
                        *loop_kind,
                        stmt,
                        stmts,
                        stack_snapshot,
                        body,
                        db.runtime().verbose(),
                    ),
                    HistoryEntry::ControlFlow {
                        opt_branch_entered: enter,
                        ..
                    } => todo!(),
                    HistoryEntry::Break => todo!(),
                    HistoryEntry::PatternMatching { .. } => todo!(),
                }
            }
            ProcStmtVariant::Break => Arc::new(vec![]),
            ProcStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        },
        TraceVariant::FeatureExpr(ref expr) => {
            db.feature_expr_subtraces(trace, expr, effective_opt_sample_id)
        }
        TraceVariant::FeatureBranch(ref branch) => db.feature_branch_subtraces(trace, branch),
        TraceVariant::EagerExpr {
            ref expr,
            ref history,
        } => db.eager_expr_subtraces(trace, expr, history),
        TraceVariant::LoopFrame {
            ref loop_frame_data,
            ref loop_stmt,
            ref body_stmts,
            ref body_instruction_sheet,
        } => db.loop_frame_subtraces(
            loop_stmt,
            body_stmts,
            body_instruction_sheet,
            loop_frame_data,
            trace,
            db.runtime().verbose(),
        ),
        TraceVariant::ProcBranch {
            ref stmt,
            branch_idx,
            ref history,
            ref opt_vm_branch,
            ref branch,
            ..
        } => match history.get(stmt).unwrap() {
            HistoryEntry::ControlFlow {
                stack_snapshot,
                opt_branch_entered: branch_entered,
                ..
            } => {
                should_eq!(Some(branch_idx), *branch_entered);
                db.proc_branch_subtraces(
                    &branch.stmts,
                    &opt_vm_branch.as_ref().unwrap().body,
                    stack_snapshot,
                    trace,
                    db.runtime().verbose(),
                )
            }
            _ => panic!(),
        },
    }
}

pub fn trace_stalk(
    this: &dyn TraceQueryGroup,
    trace_id: TraceId,
    sample_id: SampleId,
) -> Arc<TraceStalkData> {
    let trace: &Trace = &this.trace(trace_id);
    Arc::new(match trace.variant {
        TraceVariant::Main(ref repr) => TraceStalkData {
            extra_tokens: vec![
                fade!(" = "),
                this.runtime().eval_feature_repr(repr, sample_id).into(),
            ],
        },
        TraceVariant::FeatureStmt(ref stmt) => match stmt.variant {
            FeatureStmtVariant::Init { varname, ref value } => TraceStalkData {
                extra_tokens: vec![
                    fade!(" = "),
                    this.runtime().eval_feature_expr(value, sample_id).into(),
                ],
            },
            FeatureStmtVariant::Assert { ref condition } => TraceStalkData {
                extra_tokens: vec![
                    fade!(" = "),
                    this.runtime()
                        .eval_feature_expr(condition, sample_id)
                        .into(),
                ],
            },
            FeatureStmtVariant::Return { ref result } => TraceStalkData {
                extra_tokens: vec![
                    fade!(" = "),
                    this.runtime().eval_feature_expr(result, sample_id).into(),
                ],
            },
            FeatureStmtVariant::ConditionFlow { ref branches } => panic!(),
        },
        TraceVariant::FeatureBranch(_) => TraceStalkData {
            extra_tokens: vec![],
        },
        TraceVariant::FeatureExpr(ref expr) => TraceStalkData {
            extra_tokens: vec![
                fade!(" = "),
                this.runtime().eval_feature_expr(expr, sample_id).into(),
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
            ControlSnapshot::None => TraceStalkData::default(),
            ControlSnapshot::Return(_) => todo!(),
            ControlSnapshot::Break => todo!(),
            ControlSnapshot::Err(_) => todo!(),
        },
        TraceVariant::ProcBranch { .. } => panic!(),
    })
}
