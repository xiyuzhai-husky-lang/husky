mod control;
mod impl_decl;
mod impl_impr;
mod value;

use std::sync::Arc;

use common::*;
pub(crate) use control::TraceInterpreterControlSignal;
use semantics::{DeclStmt, DeclStmtKind, Expr, ExprKind, ImprStmt, InstructionSheet, Opn, VarIdx};
pub use value::TraceStackValue;
use vm::{InitKind, Interpreter, StackValue, VMResult};

use crate::*;

pub struct TraceInterpreter {
    values: HashMap<VarIdx, TraceStackValue>,
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
        let mut values: HashMap<VarIdx, TraceStackValue> = Default::default();
        for (i, value) in input_values.into_iter().enumerate() {
            values.insert(i.into(), value.into());
        }
        Self {
            values,
            instruction_sheet,
            trace_allocator,
            text,
        }
    }

    fn exec_expr(
        &self,
        parent_id: TraceId,
        indent: Indent,
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
                        Opn::Binary { opr, this, .. } => {
                            let (lopd_value, lopd_tokens) =
                                self.exec_expr(parent_id, indent, opds[0].clone(), false);
                            let (ropd_value, ropd_tokens) =
                                self.exec_expr(parent_id, indent, opds[1].clone(), false);
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
                            tokens.push(special!(
                                opr.spaced_code(),
                                self.trace_allocator.new_expr_trace(
                                    parent_id,
                                    indent,
                                    expr.clone(),
                                    value.into(),
                                    &self.text,
                                )
                            ));
                            tokens.extend(ropd_tokens);
                            if show_value_default {
                                tokens.push(fade!(" = "));
                                tokens.push(fade!(value));
                            }
                            value.into()
                        }
                        Opn::Prefix(_) => todo!(),
                        Opn::Suffix(_) => todo!(),
                        Opn::RoutineCall(_) => todo!(),
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

    fn init_trace_stack_value(&mut self, varidx: VarIdx, value: TraceStackValue) {
        should!(self.values.insert(varidx, value).is_none())
    }
}

impl Interpreter<'static, 'static> for TraceInterpreter {
    fn init(&mut self, init_kind: InitKind) -> VMResult<()> {
        match init_kind {
            InitKind::Let | InitKind::Decl => todo!(),
            InitKind::Var => todo!(),
        }
    }

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
