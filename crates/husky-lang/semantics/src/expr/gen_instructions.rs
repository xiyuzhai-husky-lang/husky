use common::*;
use expr::Opn;
use vm::{BinaryOpr, Contract, Instruction, InstructionKind, PrimitiveOpn};

use crate::*;

pub trait ExprInstructionBuilder {
    fn push_instruction(&mut self, instruction: Instruction);

    fn gen_expr_instructions(&mut self, expr: Arc<Expr>) {
        match expr.kind {
            StrictExprKind::Variable(ident) => todo!(),
            StrictExprKind::Scope {
                scope: id,
                compiled,
            } => todo!(),
            StrictExprKind::Literal(value) => self.push_instruction(Instruction::new(
                InstructionKind::PushPrimitiveLiteral(value),
                expr,
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
                                    self.gen_expr_instructions(opd.clone());
                                }
                                PrimitiveOpn::PureBinary(pure_binary_opr)
                            }
                            BinaryOpr::Assign(opt_binary_opr) => {
                                self.gen_expr_instructions(opds[0].clone());
                                self.gen_expr_instructions(opds[1].clone());
                                PrimitiveOpn::Assign(opt_binary_opr)
                            }
                        }),
                        expr,
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
                            expr,
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
