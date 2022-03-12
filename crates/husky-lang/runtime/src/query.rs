use common::epin;
use dataset::LabeledData;
use feature::{
    eval_lazy_block, eval_lazy_expr, eval_lazy_stmt, FeaturePtr, FeatureSheet, LazyBlock, LazyExpr,
    LazyExprKind, LazyStmt,
};
use semantics::{EntityQueryGroup, ImprStmtKind};
use vm::{exec_debug, EvalResult, HistoryEntry};

use trace::*;

use crate::*;

pub trait AskCompileTime {
    fn compile_time(&self, version: usize) -> &HuskyLangCompileTime;
}

pub trait EvalFeature {
    fn session(&self) -> &Arc<Mutex<Session<'static>>>;

    fn eval_lazy_block(&self, block: &LazyBlock, input_id: usize) -> EvalResult<'static> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_lazy_block(block, input, sheet)
    }

    fn eval_lazy_stmt(&self, stmt: &LazyStmt, input_id: usize) -> EvalResult<'static> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_lazy_stmt(stmt, input, sheet)
    }

    fn eval_lazy_expr(&self, expr: &LazyExpr, input_id: usize) -> EvalResult<'static> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_lazy_expr(expr, input, sheet)
    }
}

#[salsa::query_group(RuntimeQueryGroupStorage)]
pub trait RuntimeQueryGroup: AskCompileTime + CreateTrace + EvalFeature {
    #[salsa::input]
    fn package_main(&self) -> FilePtr;

    #[salsa::input]
    fn version(&self) -> usize;

    fn subtraces(&self, trace_id: TraceId, opt_input_id: Option<usize>) -> Arc<Vec<Arc<Trace>>>;
    fn root_traces(&self) -> Arc<Vec<TraceId>>;

    fn trace_stalk(&self, trace_id: TraceId, input_id: usize) -> Arc<TraceStalk>;
}

pub fn root_traces(this: &dyn RuntimeQueryGroup) -> Arc<Vec<TraceId>> {
    let compile_time = this.compile_time(this.version());
    let package_main = this.package_main();
    Arc::new(vec![this
        .new_trace(
            None,
            package_main,
            0,
            TraceKind::Main(compile_time.main_block(package_main).unwrap()),
        )
        .id()])
}

pub fn subtraces(
    this: &dyn RuntimeQueryGroup,
    trace_id: TraceId,
    opt_input_id: Option<usize>,
) -> Arc<Vec<Arc<Trace>>> {
    let trace: &Trace = &this.trace(trace_id);
    match trace.kind {
        TraceKind::Main(ref block) => this.lazy_block_subtraces(&trace, block),
        TraceKind::LazyStmt(_)
        | TraceKind::Input(_)
        | TraceKind::StrictDeclStmt { .. }
        | TraceKind::CallHead { .. } => Arc::new(vec![]),
        TraceKind::ImprStmt {
            ref stmt,
            ref history,
        } => match stmt.kind {
            ImprStmtKind::Init { .. }
            | ImprStmtKind::Assert { .. }
            | ImprStmtKind::Execute { .. }
            | ImprStmtKind::Return { .. } => Arc::new(vec![]),
            ImprStmtKind::BranchGroup { .. } => panic!(),
            ImprStmtKind::Loop {
                ref loop_kind,
                ref stmts,
            } => match history.entry(stmt) {
                HistoryEntry::PureExpr { .. }
                | HistoryEntry::Exec
                | HistoryEntry::Assign { .. } => Arc::new(vec![]),
                HistoryEntry::Loop {
                    result,
                    ref stack_snapshot,
                    ref body,
                } => this.loop_subtraces(trace, loop_kind, stmt, stmts, stack_snapshot, body),
            },
        },
        TraceKind::LazyExpr(ref expr) => lazy_expr_subtraces(this, trace, expr, opt_input_id),
        TraceKind::LazyBranch(ref branch) => this.lazy_branch_subtraces(trace, branch),
        TraceKind::StrictExpr { .. } => todo!(),
        TraceKind::LoopFrame {
            loop_frame_snapshot: ref vm_loop_frame,
            ref body_stmts,
            ref body_instruction_sheet,
            ..
        } => this.loop_frame_subtraces(trace, vm_loop_frame, body_instruction_sheet, body_stmts),
    }
}

fn lazy_expr_subtraces(
    this: &dyn RuntimeQueryGroup,
    parent: &Trace,
    expr: &LazyExpr,
    opt_input_id: Option<usize>,
) -> Arc<Vec<Arc<Trace>>> {
    Arc::new(match expr.kind {
        LazyExprKind::Literal(_)
        | LazyExprKind::PrimitiveBinaryOpr { .. }
        | LazyExprKind::Variable { .. } => vec![],
        LazyExprKind::FuncCall {
            ranged_scope,
            ref inputs,
            ref stmts,
            ref instruction_sheet,
            callee_file,
            ..
        } => {
            if let Some(input_id) = opt_input_id {
                let mut subtraces = vec![];
                let mut func_input_values = vec![];
                let entity = this
                    .compile_time(this.version())
                    .entity(ranged_scope.scope)
                    .unwrap();
                subtraces.push(
                    this.trace_factory()
                        .new_call_head(entity.clone(), &this.text(callee_file).unwrap()),
                );
                for func_input in inputs {
                    subtraces.push(this.new_trace(
                        Some(parent.id()),
                        expr.file,
                        4,
                        TraceKind::Input(func_input.clone()),
                    ));
                    match this.eval_lazy_expr(func_input, input_id) {
                        Ok(value) => func_input_values.push(value),
                        Err(_) => return Arc::new(subtraces),
                    }
                }
                todo!()
                // let interpreter = TraceInterpreter::new(
                //     func_input_values,
                //     instruction_sheet.clone(),
                //     this.trace_allocator_arc(),
                //     this.text(callee_file).unwrap(),
                // );
                // subtraces.extend(interpreter.decl_stmt_traces(parent, stmts, 4));
                // subtraces
            } else {
                vec![]
            }
        }
        LazyExprKind::ProcCall {
            ranged_scope,
            ref inputs,
            ref stmts,
            ref instruction_sheet,
            callee_file,
            ..
        } => {
            if let Some(input_id) = opt_input_id {
                let mut subtraces = vec![];
                let mut func_input_values = vec![];
                let entity = this
                    .compile_time(this.version())
                    .entity(ranged_scope.scope)
                    .unwrap();
                subtraces.push(
                    this.trace_factory()
                        .new_call_head(entity.clone(), &this.text(callee_file).unwrap()),
                );
                for func_input in inputs {
                    subtraces.push(this.new_trace(
                        Some(parent.id()),
                        expr.file,
                        4,
                        TraceKind::Input(func_input.clone()),
                    ));
                    match this.eval_lazy_expr(func_input, input_id) {
                        Ok(value) => match value.into_stack() {
                            Ok(value) => func_input_values.push(value),
                            Err(_) => {
                                todo!();
                                return Arc::new(subtraces);
                            }
                        },
                        Err(_) => {
                            todo!();
                            return Arc::new(subtraces);
                        }
                    }
                }
                let history = exec_debug(func_input_values, instruction_sheet);
                subtraces.extend(this.trace_factory().impr_stmts_traces(
                    parent.id(),
                    4,
                    stmts,
                    &this.text(callee_file).unwrap(),
                    &history,
                ));
                subtraces
                // let interpreter = TraceInterpreter::new(
                //     func_input_values,
                //     instruction_sheet.clone(),
                //     this.trace_allocator_arc(),
                //     this.text(callee_file).unwrap(),
                // );
                // subtraces.extend(interpreter.impr_stmt_traces(parent, stmts, 4));
                // subtraces
            } else {
                vec![]
            }
        }
    })
}

pub fn trace_stalk(
    this: &dyn RuntimeQueryGroup,
    trace_id: TraceId,
    input_id: usize,
) -> Arc<TraceStalk> {
    let trace: &Trace = &this.trace(trace_id);
    Arc::new(match trace.kind {
        TraceKind::Main(ref block) => TraceStalk {
            extra_tokens: vec![
                trace::fade!(" = "),
                this.eval_lazy_block(block, input_id).into(),
            ],
        },
        TraceKind::LazyStmt(ref stmt) => match stmt.kind {
            feature::LazyStmtKind::Init { varname, ref value } => TraceStalk {
                extra_tokens: vec![
                    trace::fade!(" = "),
                    this.eval_lazy_expr(value, input_id).into(),
                ],
            },
            feature::LazyStmtKind::Assert { ref condition } => TraceStalk {
                extra_tokens: vec![
                    trace::fade!(" = "),
                    this.eval_lazy_expr(condition, input_id).into(),
                ],
            },
            feature::LazyStmtKind::Return { ref result } => TraceStalk {
                extra_tokens: vec![
                    trace::fade!(" = "),
                    this.eval_lazy_expr(result, input_id).into(),
                ],
            },
            feature::LazyStmtKind::Branches { kind, ref branches } => panic!(),
        },
        TraceKind::LazyBranch(_) => TraceStalk {
            extra_tokens: vec![],
        },
        TraceKind::LazyExpr(ref expr) => TraceStalk {
            extra_tokens: vec![
                trace::fade!(" = "),
                this.eval_lazy_expr(expr, input_id).into(),
            ],
        },
        TraceKind::Input(_) => todo!(),
        TraceKind::StrictDeclStmt { .. }
        | TraceKind::ImprStmt { .. }
        | TraceKind::StrictExpr { .. }
        | TraceKind::CallHead { .. } => panic!(),
        TraceKind::LoopFrame {
            loop_frame_snapshot: ref vm_loop_frame,
            ..
        } => match vm_loop_frame.control {
            vm::ControlSnapshot::None => TraceStalk::default(),
            vm::ControlSnapshot::Return(_) => todo!(),
            vm::ControlSnapshot::Break => todo!(),
            vm::ControlSnapshot::Err(_) => todo!(),
        },
    })
}
