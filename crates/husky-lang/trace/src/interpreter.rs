mod control;
mod value;

use std::sync::Arc;

use common::*;
pub(crate) use control::TraceInterpreterControlSignal;
use semantics::{DeclStmt, DeclStmtKind, Expr, ExprKind, InstructionSheet, Opn};
pub(crate) use value::TraceStackValue;
use vm::{Interpreter, StackValue, VMResult};

use crate::*;

pub struct TraceInterpreter {
    values: HashMap<usize, StackValue<'static, 'static>>,
    instruction_sheet: Arc<InstructionSheet>,
    trace_allocator: Arc<TraceAllocator>,
    text: Arc<Text>,
}

impl TraceInterpreter {
    pub fn new(
        input_values: Vec<StackValue<'static, 'static>>,
        instruction_sheet: Arc<InstructionSheet>,
        trace_allocator: Arc<TraceAllocator>,
        text: Arc<Text>,
    ) -> Self {
        let mut values: HashMap<usize, StackValue<'static, 'static>> = Default::default();
        for (i, value) in input_values.into_iter().enumerate() {
            values.insert(i, value);
        }
        Self {
            values,
            instruction_sheet,
            trace_allocator,
            text,
        }
    }

    pub fn decl_stmt_traces(
        &self,
        parent: &Trace,
        stmts: &[Arc<DeclStmt>],
        indent: Indent,
    ) -> Vec<Arc<Trace>> {
        let mut traces = vec![];
        for stmt in stmts {
            let trace = self.trace_allocator.new_decl_stmt_trace(
                parent.id,
                indent,
                stmt.clone(),
                |trace_id| self.exec_decl_stmt(trace_id, stmt),
                &self.text,
            );
            let stop = match trace.kind {
                TraceKind::DeclStmt {
                    ref control_signal, ..
                } => match control_signal {
                    TraceInterpreterControlSignal::Return(_)
                    | TraceInterpreterControlSignal::Err(_) => true,
                    TraceInterpreterControlSignal::None => false,
                },
                _ => panic!(),
            };
            traces.push(trace);
            if stop {
                break;
            }
        }
        traces
    }

    fn exec_decl_stmt(
        &self,
        trace_id: TraceId,
        stmt: &DeclStmt,
    ) -> (TraceInterpreterControlSignal, Vec<TokenProps>) {
        match stmt.kind {
            DeclStmtKind::Init { varname, ref value } => todo!(),
            DeclStmtKind::Assert { ref condition } => todo!(),
            DeclStmtKind::Return { ref result } => {
                let (result, tokens) = self.exec_expr(trace_id, result.clone(), true);
                let control_signal = match result {
                    Ok(value) => TraceInterpreterControlSignal::Return(value),
                    Err(error) => TraceInterpreterControlSignal::Err(error),
                };
                (control_signal, tokens)
            }
            DeclStmtKind::Branches { kind, ref branches } => todo!(),
        }
    }

    fn exec_expr(
        &self,
        parent_id: TraceId,
        expr: Arc<Expr>,
        show_value_default: bool,
    ) -> (VMResult<TraceStackValue>, Vec<TokenProps>) {
        match expr.kind {
            ExprKind::Variable(_) => todo!(),
            ExprKind::Scope { scope, compiled } => todo!(),
            ExprKind::Literal(value) => (Ok(value.into()), vec![literal!(value)]),
            ExprKind::Bracketed(_) => todo!(),
            ExprKind::Opn {
                opn,
                compiled,
                ref opds,
            } => {
                let mut tokens = vec![];
                let value = (|| {
                    Ok(match opn {
                        Opn::Binary { opr, this, kind } => {
                            let (lopd_value, lopd_tokens) =
                                self.exec_expr(parent_id, opds[0].clone(), false);
                            let (ropd_value, ropd_tokens) =
                                self.exec_expr(parent_id, opds[1].clone(), false);
                            let value = if this.is_builtin() {
                                (|| {
                                    opr.act_on_primitives(
                                        lopd_value?.as_primitive()?,
                                        ropd_value?.as_primitive()?,
                                    )
                                })()
                            } else {
                                todo!()
                            }?;
                            tokens.extend(lopd_tokens);
                            tokens.push(special!(opr.spaced_code()));
                            tokens.extend(ropd_tokens);
                            if show_value_default {
                                tokens.push(fade!(" = "));
                                tokens.push(fade!(value));
                            }
                            value.into()
                        }
                        Opn::Prefix(_) => todo!(),
                        Opn::Suffix(_) => todo!(),
                        Opn::FuncCall {
                            func,
                            scope_expr_range,
                        } => todo!(),
                        Opn::PattCall => todo!(),
                        Opn::MembVarAccess => todo!(),
                        Opn::MembFuncCall(_) => todo!(),
                        Opn::ElementAccess => todo!(),
                    })
                })();
                (value, tokens)
            }
            ExprKind::Lambda(_, _) => todo!(),
        }
    }
}

impl Interpreter<'static, 'static> for TraceInterpreter {
    fn var(&self, rel_idx: usize) -> vm::VMResult<&StackValue<'static, 'static>> {
        todo!()
    }

    fn var_mut(&mut self, rel_idx: usize) -> vm::VMResult<&mut StackValue<'static, 'static>> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn push(&mut self, value: StackValue<'static, 'static>) {
        todo!()
    }

    fn pop(&mut self) -> vm::VMResult<StackValue<'static, 'static>> {
        todo!()
    }

    fn drain(&mut self, new_len: usize) -> Vec<StackValue<'static, 'static>> {
        todo!()
    }
}
