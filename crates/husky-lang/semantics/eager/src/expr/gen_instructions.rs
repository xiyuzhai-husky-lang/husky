use super::EagerOpnKind;
use common::*;
use vm::{BinaryOpr, Contract, Instruction, InstructionKind, PrimitiveOpn};

use crate::*;

pub trait ExprInstructionBuilder {
    fn push_instruction(&mut self, instruction: Instruction);

    fn gen_expr_instructions(&mut self, expr: Arc<EagerExpr>) {
        match expr.kind {
            EagerExprKind::Variable(ident) => todo!(),
            EagerExprKind::Scope {
                scope: id,
                compiled,
            } => todo!(),
            EagerExprKind::Literal(value) => self.push_instruction(Instruction::new(
                InstructionKind::PushPrimitiveLiteral(value),
                expr,
            )),
            EagerExprKind::Bracketed(_) => todo!(),
            EagerExprKind::Opn {
                opn_kind: opn,
                compiled,
                ref opds,
            } => match opn {
                EagerOpnKind::Binary { opr, this } => {
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
                EagerOpnKind::Prefix(_) => todo!(),
                EagerOpnKind::Suffix(_) => todo!(),
                EagerOpnKind::RoutineCall(routine) => {
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
                EagerOpnKind::PatternCall => todo!(),
                EagerOpnKind::MembVarAccess => todo!(),
                EagerOpnKind::MembFuncCall(_) => todo!(),
                EagerOpnKind::ElementAccess => todo!(),
                EagerOpnKind::TypeCall(_) => todo!(),
            },
            EagerExprKind::Lambda(_, _) => todo!(),
        }
    }
}
