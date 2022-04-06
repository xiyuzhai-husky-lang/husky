use datasets::LabeledData;
use feature::*;
use semantics_eager::ProcStmtKind;
use visual_runtime::VisualQueryGroup;
use vm::{exec_debug, EvalResult, HistoryEntry};

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
        opt_input_id: Option<usize>,
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
            TraceKind::Main(compile_time.main_feature_block(pack_main).unwrap()),
        )
        .id()])
}

pub fn subtraces(
    db: &dyn RuntimeQueryGroup,
    trace_id: TraceId,
    opt_input_id: Option<usize>,
) -> Arc<Vec<Arc<Trace<'static>>>> {
    let trace: &Trace = &db.trace(trace_id);
    match trace.kind {
        TraceKind::Main(ref block) => db.feature_block_subtraces(&trace, block),
        TraceKind::FeatureStmt(_)
        | TraceKind::Input(_)
        | TraceKind::StrictDeclStmt { .. }
        | TraceKind::CallHead { .. } => Arc::new(vec![]),
        TraceKind::ImprStmt {
            ref stmt,
            ref history,
        } => match stmt.kind {
            ProcStmtKind::Init { .. }
            | ProcStmtKind::Assert { .. }
            | ProcStmtKind::Execute { .. }
            | ProcStmtKind::Return { .. } => Arc::new(vec![]),
            ProcStmtKind::BranchGroup { .. } => panic!(),
            ProcStmtKind::Loop {
                ref loop_kind,
                ref stmts,
            } => match history.entry(stmt) {
                HistoryEntry::NonVoidExpr { .. }
                | HistoryEntry::Exec
                | HistoryEntry::Assign { .. } => Arc::new(vec![]),
                HistoryEntry::Loop {
                    result,
                    ref stack_snapshot,
                    ref body,
                } => db.loop_subtraces(
                    db.compile_time(trace.compile_time_version()),
                    trace,
                    loop_kind,
                    stmt,
                    stmts,
                    stack_snapshot,
                    body,
                ),
            },
        },
        TraceKind::FeatureExpr(ref expr) => feature_expr_subtraces(db, trace, expr, opt_input_id),
        TraceKind::FeatureBranch(ref branch) => db.feature_branch_subtraces(trace, branch),
        TraceKind::EagerExpr { .. } => todo!(),
        TraceKind::LoopFrame {
            loop_frame_snapshot: ref vm_loop_frame,
            ref body_stmts,
            ref body_instruction_sheet,
            ..
        } => db.loop_frame_subtraces(
            db.compile_time(trace.compile_time_version()),
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
        FeatureExprKind::FuncCall {
            func_ranged_scope: ranged_scope,
            ref inputs,
            ref stmts,
            ref instruction_sheet,
            callee_file,
            ..
        } => {
            if let Some(input_id) = opt_input_id {
                let mut subtraces = vec![];
                let mut func_input_values = vec![];
                let entity_defn = db
                    .compile_time(db.version())
                    .opt_entity_defn(ranged_scope.route)
                    .unwrap()
                    .unwrap();
                subtraces.push(
                    db.trace_factory()
                        .new_call_head(entity_defn.clone(), &db.text(callee_file).unwrap()),
                );
                for func_input in inputs {
                    subtraces.push(db.new_trace(
                        Some(parent.id()),
                        expr.file,
                        4,
                        TraceKind::Input(func_input.clone()),
                    ));
                    match db.eval_feature_expr(func_input, input_id) {
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
        FeatureExprKind::ProcCall {
            proc_ranged_scope: ranged_scope,
            ref inputs,
            ref stmts,
            ref instruction_sheet,
            callee_file,
            ..
        } => {
            if let Some(input_id) = opt_input_id {
                let mut subtraces = vec![];
                let mut func_input_values = vec![];
                let entity_defn = db
                    .compile_time(db.version())
                    .opt_entity_defn(ranged_scope.route)
                    .unwrap()
                    .unwrap();
                subtraces.push(
                    db.trace_factory()
                        .new_call_head(entity_defn.clone(), &db.text(callee_file).unwrap()),
                );
                for func_input in inputs {
                    subtraces.push(db.new_trace(
                        Some(parent.id()),
                        expr.file,
                        4,
                        TraceKind::Input(func_input.clone()),
                    ));
                    match db.eval_feature_expr(func_input, input_id) {
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
                let history = exec_debug(
                    db.compile_time(parent.compile_time_version()),
                    func_input_values,
                    instruction_sheet,
                );
                subtraces.extend(db.trace_factory().impr_stmts_traces(
                    parent.id(),
                    4,
                    stmts,
                    &db.text(callee_file).unwrap(),
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
        FeatureExprKind::StructMembVarAccess { .. } => todo!(),
        FeatureExprKind::EnumLiteral { .. } => todo!(),
        FeatureExprKind::MembFuncCall {
            memb_ident,
            ref opds,
            ref instruction_sheet,
            ref stmts,
            ..
        } => todo!(),
        FeatureExprKind::MembProcCall {
            memb_ident,
            ref opds,
            ref instruction_sheet,
            ref stmts,
            ..
        } => todo!(),
        FeatureExprKind::MembPattCall {
            memb_ident,
            ref opds,
            ref instruction_sheet,
            ref stmts,
        } => todo!(),
        FeatureExprKind::FeatureBlock { .. } => todo!(),
        FeatureExprKind::ClassCall { ty, ref opds, .. } => todo!(),
        FeatureExprKind::RecordMembAccess {
            ref this,
            memb_ident,
            ..
        } => todo!(),
        FeatureExprKind::This { ref repr } => todo!(),
        FeatureExprKind::GlobalInput => todo!(),
    })
}

pub fn trace_stalk(
    this: &dyn RuntimeQueryGroup,
    trace_id: TraceId,
    input_id: usize,
) -> Arc<TraceStalk<'static>> {
    let trace: &Trace = &this.trace(trace_id);
    Arc::new(match trace.kind {
        TraceKind::Main(ref block) => TraceStalk {
            extra_tokens: vec![
                trace::fade!(" = "),
                this.eval_feature_block(block, input_id).into(),
            ],
        },
        TraceKind::FeatureStmt(ref stmt) => match stmt.kind {
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
        TraceKind::FeatureBranch(_) => TraceStalk {
            extra_tokens: vec![],
        },
        TraceKind::FeatureExpr(ref expr) => TraceStalk {
            extra_tokens: vec![
                trace::fade!(" = "),
                this.eval_feature_expr(expr, input_id).into(),
            ],
        },
        TraceKind::Input(_) => todo!(),
        TraceKind::StrictDeclStmt { .. }
        | TraceKind::ImprStmt { .. }
        | TraceKind::EagerExpr { .. }
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
