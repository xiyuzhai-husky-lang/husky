use crate::*;

use common::p;
use vm::{BinaryOpr, Contract, Instruction, InstructionKind, PrimitiveOpn};

impl InstructionSheetBuilder {
    pub(super) fn compile_expr(&mut self, expr: &Arc<Expr>) {
        match expr.kind {
            ExprKind::Variable(ident) => {
                let stack_idx = self.sheet.variable_stack.stack_idx(ident);
                self.push_instruction(Instruction::new(
                    InstructionKind::PushVariable {
                        stack_idx,
                        contract: expr.contract,
                    },
                    expr.clone(),
                ))
            }
            ExprKind::Scope {
                scope: id,
                compiled,
            } => todo!(),
            ExprKind::Literal(value) => self.push_instruction(Instruction::new(
                InstructionKind::PushPrimitiveLiteral(value),
                expr.clone(),
            )),
            ExprKind::Bracketed(_) => todo!(),
            ExprKind::Opn {
                opn_kind: opn,
                compiled,
                ref opds,
            } => match opn {
                OpnKind::Binary { opr, this } => {
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
                OpnKind::Prefix(_) => todo!(),
                OpnKind::Suffix(_) => todo!(),
                OpnKind::RoutineCall(routine) => {
                    if let Some(compiled) = compiled {
                        self.push_instruction(Instruction::new(
                            InstructionKind::CallCompiled {
                                compiled,
                                nargs: opds.len() as u8,
                            },
                            expr.clone(),
                        ))
                    } else {
                        todo!()
                    }
                }
                OpnKind::PatternCall => todo!(),
                OpnKind::MembVarAccess => todo!(),
                OpnKind::MembFuncCall(_) => todo!(),
                OpnKind::ElementAccess => todo!(),
            },
            ExprKind::Lambda(_, _) => todo!(),
        }
    }
}
