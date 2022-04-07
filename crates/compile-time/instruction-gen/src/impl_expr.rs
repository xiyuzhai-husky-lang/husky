use crate::*;

use decl::{TyDecl, TyDeclKind};
use syntax_types::SuffixOpr;
use vm::{BinaryOpr, Instruction, InstructionKind, PrimitiveOpn, StackIdx};

impl<'a> InstructionSheetBuilder<'a> {
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
            EagerExprKind::Scope { scope } => todo!(),
            EagerExprKind::PrimitiveLiteral(value) => self.push_instruction(Instruction::new(
                InstructionKind::PushPrimitiveLiteral(value),
                expr.clone(),
            )),
            EagerExprKind::Bracketed(_) => todo!(),
            EagerExprKind::Opn {
                ref opn_kind,
                ref opds,
            } => self.compile_opn(opn_kind, opds, expr),
            EagerExprKind::Lambda(_, _) => todo!(),
            EagerExprKind::This => self.push_instruction(Instruction::new(
                InstructionKind::PushVariable {
                    stack_idx: StackIdx::this(),
                    contract: expr.contract,
                },
                expr.clone(),
            )),
        }
    }

    fn compile_opn(
        &mut self,
        opn_kind: &EagerOpnKind,
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
            EagerOpnKind::Prefix { opr, .. } => {
                todo!()
            }
            EagerOpnKind::Suffix { opr, this: this_ty } => {
                let instruction = Instruction::new(
                    match opr {
                        SuffixOpr::Incr => todo!(),
                        SuffixOpr::Decr => todo!(),
                        SuffixOpr::MayReturn => todo!(),
                        SuffixOpr::MembAccess(memb_ident) => {
                            if let Some(memb_access_fp) = self.memb_access_fp(*this_ty, *memb_ident)
                            {
                                InstructionKind::MembAccessCompiled { memb_access_fp }
                            } else {
                                let this_ty_decl = self.db.ty_decl(*this_ty).unwrap();
                                InstructionKind::MembAccessInterpreted {
                                    memb_idx: this_ty_decl
                                        .memb_idx(*memb_ident)
                                        .try_into()
                                        .unwrap(),
                                    contract: expr.contract,
                                }
                            }
                        }
                        SuffixOpr::WithType(_) => todo!(),
                    },
                    expr.clone(),
                );
                self.push_instruction(instruction)
            }
            EagerOpnKind::RoutineCall(routine) => {
                if let Some(fp) = self.routine_fp(routine.route) {
                    self.push_instruction(Instruction::new(
                        InstructionKind::RoutineCallCompiled { fp },
                        expr.clone(),
                    ))
                } else {
                    self.push_instruction(Instruction::new(
                        InstructionKind::RoutineCallInterpreted {
                            routine: self.db.entity_uid(routine.route),
                            nargs: opds.len() as u8,
                        },
                        expr.clone(),
                    ))
                }
            }
            EagerOpnKind::PatternCall => todo!(),
            EagerOpnKind::MembVarAccess { memb_var_contract } => {
                todo!()
            }
            EagerOpnKind::MembRoutineCall {
                memb_ident,
                ref this_ty_decl,
            } => self.push_instruction(Instruction::new(
                self.memb_routine_call_instruction_kind(opds[0].ty, this_ty_decl, *memb_ident),
                expr.clone(),
            )),
            EagerOpnKind::ElementAccess => todo!(),
            EagerOpnKind::TypeCall {
                ranged_ty,
                ref ty_decl,
            } => {
                msg_once!("TypeCall compiled");
                match ty_decl.kind {
                    TyDeclKind::Struct {
                        ref memb_vars,
                        ref memb_routines,
                    } => {
                        if let Some(compiled_routine) = self
                            .db
                            .fp_table()
                            .struct_constructor(self.db.entity_uid(ranged_ty.route))
                        {
                            todo!()
                        } else {
                            self.push_instruction(Instruction::new(
                                InstructionKind::NewVirtualStruct {
                                    memb_vars: memb_vars
                                        .iter()
                                        .map(|(_, decl)| decl.contract)
                                        .collect(),
                                },
                                expr.clone(),
                            ));
                        }
                    }
                    TyDeclKind::Enum { ref variants } => todo!(),
                    TyDeclKind::Record {
                        ref memb_vars,
                        ref memb_features,
                    } => todo!(),
                    TyDeclKind::Vec { element_ty } => self.push_instruction(Instruction::new(
                        InstructionKind::RoutineCallCompiled {
                            fp: self
                                .db
                                .fp_table()
                                .vec_constructor(self.db.entity_uid(element_ty)),
                        },
                        expr.clone(),
                    )),
                }
            }
        }
    }

    fn memb_routine_call_instruction_kind(
        &self,
        this_ty: EntityRoutePtr,
        this_ty_decl: &TyDecl,
        memb_ident: CustomIdentifier,
    ) -> InstructionKind {
        if let Some(routine_fp) = self.memb_routine_fp(this_ty, memb_ident) {
            todo!()
        } else {
            match this_ty_decl.kind {
                TyDeclKind::Struct {
                    ref memb_vars,
                    ref memb_routines,
                } => todo!(),
                TyDeclKind::Enum { ref variants } => todo!(),
                TyDeclKind::Record {
                    ref memb_vars,
                    ref memb_features,
                } => todo!(),
                TyDeclKind::Vec { element_ty } => {
                    let fp = self.db.virtual_vec_memb_routine_fps()[memb_ident];
                    InstructionKind::RoutineCallCompiled { fp }
                }
            }
        }
    }
}
