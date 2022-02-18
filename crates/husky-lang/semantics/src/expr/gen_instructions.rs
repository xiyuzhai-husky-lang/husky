use common::*;
use vm::{Instruction, InstructionKind};

use crate::*;

pub trait ExprInstructionBuilder {
    fn push_instruction(&mut self, instruction: Instruction);

    fn gen_expr_instructions(&mut self, expr: &Expr) {
        match expr.kind {
            ExprKind::Variable(_) => todo!(),
            ExprKind::Scope {
                scope: id,
                compiled,
            } => todo!(),
            ExprKind::Literal(_) => todo!(),
            ExprKind::Bracketed(_) => todo!(),
            ExprKind::Opn {
                ref opn,
                compiled,
                ref opds,
            } => match opn {
                expr::Opn::Binary { opr, this, kind } => todo!(),
                expr::Opn::Prefix(_) => todo!(),
                expr::Opn::Suffix(_) => todo!(),
                expr::Opn::FuncCall {
                    func,
                    scope_expr_range,
                } => {
                    for opd in opds {
                        self.gen_expr_instructions(opd);
                        if let Some(compiled) = compiled {
                            todo!()
                        } else {
                            todo!()
                        }
                    }
                    if let Some(compiled) = compiled {
                        self.push_instruction(Instruction {
                            kind: InstructionKind::Call {
                                compiled,
                                nargs: opds.len() as u16,
                            },
                        })
                    } else {
                        todo!()
                    }
                }
                expr::Opn::PattCall => todo!(),
                expr::Opn::MembVarAccess => todo!(),
                expr::Opn::MembFuncCall(_) => todo!(),
                expr::Opn::ElementAccess => todo!(),
            },
            ExprKind::Lambda(_, _) => todo!(),
        }
    }
}
