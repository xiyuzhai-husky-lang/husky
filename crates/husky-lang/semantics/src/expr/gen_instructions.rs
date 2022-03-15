use common::*;
use expr::OpnKind;
use vm::{BinaryOpr, Contract, Instruction, InstructionKind, PrimitiveOpn};

use crate::*;

pub trait ExprInstructionBuilder {
    fn push_instruction(&mut self, instruction: Instruction);

    fn gen_expr_instructions(&mut self, expr: Arc<Expr>) {
        match expr.kind {
            ExprKind::Variable(ident) => todo!(),
            ExprKind::Scope {
                scope: id,
                compiled,
            } => todo!(),
            ExprKind::Literal(value) => self.push_instruction(Instruction::new(
                InstructionKind::PushPrimitiveLiteral(value),
                expr,
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
                OpnKind::Prefix(_) => todo!(),
                OpnKind::Suffix(_) => todo!(),
                OpnKind::RoutineCall(routine) => {
                    if let Some(compiled) = compiled {
                        self.push_instruction(Instruction::new(
                            InstructionKind::CallCompiled {
                                compiled,
                                nargs: opds.len() as u8,
                            },
                            expr,
                        ))
                    } else {
                        todo!()
                    }
                }
                OpnKind::PattCall => todo!(),
                OpnKind::MembVarAccess => todo!(),
                OpnKind::MembFuncCall(_) => todo!(),
                OpnKind::ElementAccess => todo!(),
            },
            ExprKind::Lambda(_, _) => todo!(),
        }
    }
}
