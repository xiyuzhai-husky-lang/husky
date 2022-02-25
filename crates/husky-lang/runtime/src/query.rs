use common::epin;
use dataset::LabeledData;
use feature::{
    eval_feature_block, eval_feature_expr, eval_feature_stmt, FeatureBlock, FeatureExpr,
    FeatureExprKind, FeaturePtr, FeatureSheet, FeatureStmt,
};
use vm::EvalValue;

use trace::*;

use crate::*;

pub trait AskCompileTime {
    fn compile_time(&self, version: usize) -> &HuskyLangCompileTime;
}

pub trait EvalFeature {
    fn session(&self) -> &Arc<Mutex<Session<'static>>>;

    fn eval_feature_block(
        &self,
        block: &FeatureBlock,
        input_id: usize,
    ) -> EvalValue<'static, 'static> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_block(block, input, sheet)
    }

    fn eval_feature_stmt(
        &self,
        stmt: &FeatureStmt,
        input_id: usize,
    ) -> EvalValue<'static, 'static> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_stmt(stmt, input, sheet)
    }

    fn eval_feature_expr(
        &self,
        expr: &FeatureExpr,
        input_id: usize,
    ) -> EvalValue<'static, 'static> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_expr(expr, input, sheet)
    }
}

#[salsa::query_group(RuntimeQueryGroupStorage)]
pub trait RuntimeQueryGroup: AskCompileTime + AllocateTrace + EvalFeature {
    #[salsa::input]
    fn package_main(&self) -> FilePtr;

    #[salsa::input]
    fn version(&self) -> usize;

    fn subtraces(&self, trace_id: TraceId, opt_input_id: Option<usize>) -> Arc<Vec<Arc<Trace>>>;
    fn root_traces(&self) -> Arc<Vec<Arc<Trace>>>;

    fn trace_stalk(&self, trace_id: TraceId, input_id: usize) -> Arc<TraceStalk>;
}

pub fn root_traces(this: &dyn RuntimeQueryGroup) -> Arc<Vec<Arc<Trace>>> {
    let compile_time = this.compile_time(this.version());
    let package_main = this.package_main();
    Arc::new(vec![this.new_trace(
        None,
        package_main,
        0,
        TraceKind::Main(compile_time.main_feature_block(package_main).unwrap()),
    )])
}

pub fn subtraces(
    this: &dyn RuntimeQueryGroup,
    trace_id: TraceId,
    opt_input_id: Option<usize>,
) -> Arc<Vec<Arc<Trace>>> {
    let trace: &Trace = &this.trace(trace_id);
    match trace.kind {
        TraceKind::Main(ref block) => Arc::new(this.feature_block_subtraces(&trace, block)),
        TraceKind::FeatureStmt(_) | TraceKind::Input(_) | TraceKind::DeclStmt { .. } => {
            Arc::new(vec![])
        }
        TraceKind::FeatureExpr(ref expr) => feature_expr_subtraces(this, trace, expr, opt_input_id),
        TraceKind::FeatureBranch(ref branch) => this.feature_branch_subtraces(trace, branch),
    }
}

fn feature_expr_subtraces(
    this: &dyn RuntimeQueryGroup,
    parent: &Trace,
    expr: &FeatureExpr,
    opt_input_id: Option<usize>,
) -> Arc<Vec<Arc<Trace>>> {
    Arc::new(match expr.kind {
        FeatureExprKind::Literal(_)
        | FeatureExprKind::PrimitiveBinaryOpr { .. }
        | FeatureExprKind::Variable { .. } => vec![],
        FeatureExprKind::FuncCall {
            ref inputs,
            ref stmts,
            ref instruction_sheet,
            callee_file,
            ..
        } => {
            if let Some(input_id) = opt_input_id {
                let mut subtraces = vec![];
                let mut func_input_values = vec![];
                for func_input in inputs {
                    subtraces.push(this.new_trace(
                        Some(parent),
                        expr.file,
                        4,
                        TraceKind::Input(func_input.clone()),
                    ));
                    let func_input_value =
                        (|| this.eval_feature_expr(func_input, input_id)?.defined())();
                    match func_input_value {
                        Ok(value) => func_input_values.push(value),
                        Err(_) => return Arc::new(subtraces),
                    }
                }
                let interpreter = TraceInterpreter::new(
                    func_input_values,
                    instruction_sheet.clone(),
                    this.trace_allocator_arc(),
                    this.text(callee_file).unwrap(),
                );
                subtraces.extend(interpreter.decl_stmt_traces(parent, stmts, 4));
                subtraces
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
                this.eval_feature_block(block, input_id).into(),
            ],
        },
        TraceKind::FeatureStmt(ref stmt) => match stmt.kind {
            feature::FeatureStmtKind::Init { varname, ref value } => TraceStalk {
                extra_tokens: vec![
                    trace::fade!(" = "),
                    this.eval_feature_expr(value, input_id).into(),
                ],
            },
            feature::FeatureStmtKind::Assert { ref condition } => TraceStalk {
                extra_tokens: vec![
                    trace::fade!(" = "),
                    this.eval_feature_expr(condition, input_id).into(),
                ],
            },
            feature::FeatureStmtKind::Return { ref result } => TraceStalk {
                extra_tokens: vec![
                    trace::fade!(" = "),
                    this.eval_feature_expr(result, input_id).into(),
                ],
            },
            feature::FeatureStmtKind::Branches { kind, ref branches } => panic!(),
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
        TraceKind::DeclStmt { .. } => todo!(),
    })
}
