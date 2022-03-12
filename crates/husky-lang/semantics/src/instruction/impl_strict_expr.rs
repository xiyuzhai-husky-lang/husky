use crate::*;

use common::p;
use vm::{BinaryOpr, Contract, Instruction, InstructionKind, PrimitiveOpn};

impl InstructionSheetBuilder {
    pub(super) fn compile_expr(&mut self, expr: &Arc<Expr>) {
        match expr.kind {
            StrictExprKind::Variable(ident) => {
                let stack_idx = self.sheet.variable_stack.stack_idx(ident);
                p!(stack_idx, ident);
                self.push_instruction(Instruction::new(
                    InstructionKind::PushVariable {
                        stack_idx,
                        contract: expr.contract,
                    },
                    expr.clone(),
                ))
            }
            StrictExprKind::Scope {
                scope: id,
                compiled,
            } => todo!(),
            StrictExprKind::Literal(value) => self.push_instruction(Instruction::new(
                InstructionKind::PushPrimitiveLiteral(value),
                expr.clone(),
            )),
            StrictExprKind::Bracketed(_) => todo!(),
            StrictExprKind::Opn {
                opn,
                compiled,
                ref opds,
            } => match opn {
                Opn::Binary { opr, this, kind } => {
                    let instruction = Instruction::new(
                        InstructionKind::PrimitiveOpn(match opr {
                            BinaryOpr::Pure(pure_binary_opr) => {
                                for opd in opds {
                                    self.compile_expr(opd);
                                }
                                PrimitiveOpn::PureBinary(pure_binary_opr)
                            }
                            BinaryOpr::Assign(opt_binary_opr) => {
                                self.compile_expr(&opds[0]);
                                self.compile_expr(&opds[1]);
                                PrimitiveOpn::Assign(opt_binary_opr)
                            }
                        }),
                        expr.clone(),
                    );
                    self.push_instruction(instruction)
                }
                Opn::Prefix(_) => todo!(),
                Opn::Suffix(_) => todo!(),
                Opn::RoutineCall(routine) => {
                    if let Some(compiled) = compiled {
                        self.push_instruction(Instruction::new(
                            InstructionKind::Call {
                                compiled,
                                nargs: opds.len() as u8,
                            },
                            expr.clone(),
                        ))
                    } else {
                        todo!()
                    }
                }
                Opn::PattCall => todo!(),
                Opn::MembVarAccess => todo!(),
                Opn::MembFuncCall(_) => todo!(),
                Opn::ElementAccess => todo!(),
            },
            StrictExprKind::Lambda(_, _) => todo!(),
        }
    }
}
