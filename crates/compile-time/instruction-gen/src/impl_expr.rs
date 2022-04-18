use crate::*;

use entity_kind::TyKind;
use infer_decl::TypeDecl;
use syntax_types::SuffixOpr;
use vm::{BinaryOpr, EntityUid, Instruction, InstructionKind, PrimitiveOpn, StackIdx};

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
                        SuffixOpr::MembAccess(ranged_ident) => {
                            if let Some(field_access_fp) =
                                self.field_access_fp(*this_ty, ranged_ident.ident)
                            {
                                InstructionKind::FieldAccessCompiled {
                                    linkage: field_access_fp,
                                }
                            } else {
                                let this_ty_decl = self.db.type_decl(*this_ty).unwrap();
                                InstructionKind::FieldAccessInterpreted {
                                    field_idx: this_ty_decl
                                        .field_idx(ranged_ident.ident)
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
                        InstructionKind::RoutineCallCompiled { linkage: fp },
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
            EagerOpnKind::FieldAccess { field_contract } => {
                todo!()
            }
            EagerOpnKind::MethodCall {
                method_ident,
                ref this_ty_decl,
                method_route,
            } => self.push_instruction(Instruction::new(
                self.method_call_instruction_kind(
                    opds[0].ty,
                    this_ty_decl,
                    self.db.entity_uid(*method_route),
                    method_ident.ident,
                ),
                expr.clone(),
            )),
            EagerOpnKind::ElementAccess => todo!(),
            EagerOpnKind::TypeCall {
                ranged_ty,
                ref ty_decl,
            } => {
                msg_once!("TypeCall compiled");
                match ty_decl.kind {
                    TyKind::Struct => {
                        if let Some(compiled_routine) = self
                            .db
                            .linkage_table()
                            .struct_constructor(self.db.entity_uid(ranged_ty.route))
                        {
                            todo!()
                        } else {
                            self.push_instruction(Instruction::new(
                                InstructionKind::NewVirtualStruct {
                                    fields: ty_decl.fields().map(|decl| decl.contract).collect(),
                                },
                                expr.clone(),
                            ));
                        }
                    }
                    TyKind::Enum => todo!(),
                    TyKind::Record => todo!(),
                    TyKind::Vec => self.push_instruction(Instruction::new(
                        InstructionKind::RoutineCallCompiled {
                            linkage: self.db.linkage_table().vec_constructor(
                                self.db
                                    .entity_uid(ranged_ty.route.generic_arguments[0].as_scope()),
                            ),
                        },
                        expr.clone(),
                    )),
                    _ => todo!(),
                }
            }
        }
    }

    fn method_call_instruction_kind(
        &self,
        this_ty: EntityRoutePtr,
        this_ty_decl: &TypeDecl,
        method_uid: EntityUid,
        method_ident: CustomIdentifier,
    ) -> InstructionKind {
        if let Some(routine_fp) = self.db.linkage_table().routine(method_uid) {
            todo!()
        } else {
            match this_ty_decl.kind {
                TyKind::Struct => todo!(),
                TyKind::Enum => todo!(),
                TyKind::Record => todo!(),
                TyKind::Vec => {
                    let linkage = self.db.virtual_vec_method_linkages()[method_ident].1;
                    InstructionKind::RoutineCallCompiled { linkage }
                }
                _ => todo!(),
            }
        }
    }
}
