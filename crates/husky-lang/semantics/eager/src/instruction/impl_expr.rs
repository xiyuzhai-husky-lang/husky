use crate::*;

use common::{msg_once, p};
use vm::{BinaryOpr, Compiled, InputContract, Instruction, InstructionKind, PrimitiveOpn};

impl InstructionSheetBuilder {
    pub(super) fn compile_expr(&mut self, expr: &Arc<EagerExpr>) {
        match expr.kind {
            EagerExprKind::Variable(ident) => {
                let stack_idx = self.sheet.variable_stack.stack_idx(ident);
                self.push_instruction(Instruction::new(
                    InstructionKind::PushVariable {
                        stack_idx,
                        contract: expr.contract,
                    },
                    expr.clone(),
                ))
            }
            EagerExprKind::Scope { scope, compiled } => todo!(),
            EagerExprKind::Literal(value) => self.push_instruction(Instruction::new(
                InstructionKind::PushPrimitiveLiteral(value),
                expr.clone(),
            )),
            EagerExprKind::Bracketed(_) => todo!(),
            EagerExprKind::Opn {
                ref opn_kind,
                compiled,
                ref opds,
            } => self.compile_opn(opn_kind, compiled, opds, expr),
            EagerExprKind::Lambda(_, _) => todo!(),
        }
    }

    fn compile_opn(
        &mut self,
        opn_kind: &EagerOpnKind,
        compiled: Option<Compiled>,
        opds: &[Arc<EagerExpr>],
        expr: &Arc<EagerExpr>,
    ) {
        for opd in opds {
            self.compile_expr(opd);
        }
        match opn_kind {
            EagerOpnKind::Binary { opr, this } => {
                let instruction = Instruction::new(
                    InstructionKind::PrimitiveOpn(match opr {
                        BinaryOpr::Pure(pure_binary_opr) => {
                            PrimitiveOpn::PureBinary(*pure_binary_opr)
                        }
                        BinaryOpr::Assign(opt_binary_opr) => PrimitiveOpn::Assign(*opt_binary_opr),
                    }),
                    expr.clone(),
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
                        expr.clone(),
                    ))
                } else {
                    todo!()
                }
            }
            EagerOpnKind::PatternCall => todo!(),
            EagerOpnKind::MembVarAccess { memb_var_contract } => {
                todo!()
            }
            EagerOpnKind::MembFuncCall(_) => todo!(),
            EagerOpnKind::ElementAccess => todo!(),
            EagerOpnKind::TypeCall {
                ranged_ty,
                ref ty_signature,
            } => {
                msg_once!("TypeCall compiled");
                self.push_instruction(Instruction::new(
                    InstructionKind::TyCallInterpreted {
                        ty_signature: ty_signature.vm_ty_signature(),
                    },
                    expr.clone(),
                ))
            }
        }
    }
}
