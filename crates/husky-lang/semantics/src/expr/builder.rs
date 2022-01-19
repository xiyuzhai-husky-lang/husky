use common::*;
use virtual_stack::{Instruction, InstructionKind};

use crate::*;

pub trait ExprInstructionBuilder {
    fn push_instruction(&mut self, instruction: Instruction);

    fn build_expr_instructions(&mut self, expr: &Expr) {
        epin!();
        match &expr.kind {
            ExprKind::Variable(_) => todo!(),
            ExprKind::Scope { id, compiled } => todo!(),
            ExprKind::Literal(_) => todo!(),
            ExprKind::Bracketed(_) => todo!(),
            ExprKind::Opn {
                opn,
                compiled,
                opds,
            } => match opn {
                expr::Opn::Binary { opr, this, kind } => todo!(),
                expr::Opn::Prefix(_) => todo!(),
                expr::Opn::Suffix(_) => todo!(),
                expr::Opn::FuncCall { func } => {
                    for opd in opds {
                        self.build_expr_instructions(opd);
                        if let Some(compiled) = compiled {
                            todo!()
                        } else {
                            todo!()
                        }
                    }
                    self.push_instruction(Instruction {
                        kind: InstructionKind::Call {
                            compiled: self.compiled(func),
                            nargs: opds.len() as u16,
                        },
                    })
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
