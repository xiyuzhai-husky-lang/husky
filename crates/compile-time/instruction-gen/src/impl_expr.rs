use crate::*;

use entity_kind::TyKind;
use infer_decl::TyDecl;
use map_collect::MapCollect;
use static_defn::LinkageSource;
use vm::*;

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_expr(&mut self, expr: &Arc<EagerExpr>) {
        match expr.variant {
            EagerExprVariant::Variable(varname) => {
                let stack_idx = self.sheet.variable_stack.stack_idx(varname);
                self.push_instruction(Instruction::new(
                    InstructionKind::PushVariable {
                        varname: varname.into(),
                        stack_idx,
                        binding: expr.qualified_ty.qual.binding(expr.contract),
                        range: expr.range,
                        ty: expr.ty,
                    },
                    expr.clone(),
                ))
            }
            EagerExprVariant::EntityRoute { route } => todo!(),
            EagerExprVariant::PrimitiveLiteral(value) => self.push_instruction(Instruction::new(
                InstructionKind::PushPrimitiveLiteral(value),
                expr.clone(),
            )),
            EagerExprVariant::Bracketed(ref expr) => self.compile_expr(expr),
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.compile_opn(opn_variant, opds, expr),
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::This => self.push_instruction(Instruction::new(
                InstructionKind::PushVariable {
                    varname: ContextualIdentifier::ThisData.into(),
                    stack_idx: StackIdx::this(),
                    binding: expr.qualified_ty.qual.binding(expr.contract),
                    range: expr.range,
                    ty: expr.ty,
                },
                expr.clone(),
            )),
            EagerExprVariant::EnumKindLiteral(route) => self.push_instruction(Instruction::new(
                InstructionKind::PushEnumKindLiteral(EnumKindValue {
                    kind_idx: self.db.enum_literal_as_u8(route),
                    route,
                }),
                expr.clone(),
            )),
        }
    }

    fn compile_opn(
        &mut self,
        opn_kind: &EagerOpnVariant,
        opds: &[Arc<EagerExpr>],
        expr: &Arc<EagerExpr>,
    ) {
        for opd in opds {
            self.compile_expr(opd);
        }
        match opn_kind {
            EagerOpnVariant::Binary { opr, this_ty } => {
                let ins_kind = InstructionKind::OprOpn {
                    opn: match opr {
                        BinaryOpr::Pure(pure_binary_opr) => OprOpn::PureBinary(*pure_binary_opr),
                        BinaryOpr::Assign(opt_binary_opr) => OprOpn::BinaryAssign(*opt_binary_opr),
                    },
                    this_ty: *this_ty,
                    this_range: opds[0].range,
                };
                let instruction = Instruction::new(ins_kind, expr.clone());
                self.push_instruction(instruction)
            }
            EagerOpnVariant::Prefix { opr, this_ty } => {
                let instruction = Instruction::new(
                    InstructionKind::OprOpn {
                        opn: OprOpn::Prefix(*opr),
                        this_ty: *this_ty,
                        this_range: opds[0].range,
                    },
                    expr.clone(),
                );
                self.push_instruction(instruction)
            }
            EagerOpnVariant::Suffix { opr, this_ty } => {
                let ins_kind = match opr {
                    SuffixOpr::Incr => InstructionKind::OprOpn {
                        opn: OprOpn::Incr,
                        this_ty: *this_ty,
                        this_range: opds[0].range,
                    },
                    SuffixOpr::Decr => InstructionKind::OprOpn {
                        opn: OprOpn::Decr,
                        this_ty: *this_ty,
                        this_range: opds[0].range,
                    },
                    SuffixOpr::AsTy(ty) => InstructionKind::OprOpn {
                        opn: match ty.route {
                            EntityRoutePtr::Root(ty_ident) => match ty_ident {
                                RootIdentifier::Void => todo!(),
                                RootIdentifier::I32 => OprOpn::Cast(CastOpn::AsI32),
                                RootIdentifier::F32 => OprOpn::Cast(CastOpn::AsF32),
                                RootIdentifier::B32 => OprOpn::Cast(CastOpn::AsB32),
                                RootIdentifier::B64 => todo!(),
                                RootIdentifier::Bool => todo!(),
                                _ => todo!(),
                            },
                            EntityRoutePtr::Custom(_) => todo!(),
                            EntityRoutePtr::ThisType => todo!(),
                        },
                        this_ty: *this_ty,
                        this_range: opds[0].range,
                    },
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::FieldAccess(ranged_ident) => {
                        if let Some(field_access_fp) =
                            self.db.field_access_fp(*this_ty, ranged_ident.ident)
                        {
                            InstructionKind::FieldAccessCompiled {
                                linkage: field_access_fp,
                            }
                        } else {
                            let this_ty_decl = self.db.ty_decl(*this_ty).unwrap();
                            InstructionKind::FieldAccessInterpreted {
                                field_idx: this_ty_decl
                                    .field_idx(ranged_ident.ident)
                                    .try_into()
                                    .unwrap(),
                                contract: expr.contract,
                            }
                        }
                    }
                    SuffixOpr::WithTy(_) => todo!(),
                };
                let instruction = Instruction::new(ins_kind, expr.clone());
                self.push_instruction(instruction)
            }
            EagerOpnVariant::RoutineCall(routine) => {
                if let Some(fp) = self.db.routine_linkage(routine.route) {
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
            EagerOpnVariant::PatternCall => todo!(),
            EagerOpnVariant::FieldAccess { field_contract } => {
                todo!()
            }
            EagerOpnVariant::MethodCall {
                method_ident,
                ref this_ty_decl,
                method_route,
            } => {
                let this = &opds[0];
                self.push_instruction(Instruction::new(
                    self.method_call_instruction_kind(
                        this.ty,
                        this_ty_decl,
                        *method_route,
                        method_ident.ident,
                        this.qualified_ty.qual.binding(this.contract),
                    ),
                    expr.clone(),
                ))
            }
            EagerOpnVariant::ElementAccess => self.compile_element_access(expr.clone(), opds),
            EagerOpnVariant::TypeCall {
                ranged_ty,
                ref ty_decl,
            } => {
                emsg_once!("TypeCall compiled");
                let instruction_kind =
                    if let Some(linkage) = self.db.type_call_linkage(ranged_ty.route) {
                        InstructionKind::RoutineCallCompiled { linkage }
                    } else {
                        match ty_decl.kind {
                            TyKind::Struct => InstructionKind::NewVirtualStruct {
                                fields: ty_decl.fields().map(|decl| decl.contract).collect(),
                            },
                            TyKind::Enum => todo!(),
                            TyKind::Record => todo!(),
                            TyKind::Vec => panic!(),
                            // self.push_instruction(Instruction::new(
                            //     InstructionKind::RoutineCallCompiled {
                            //         linkage: todo!(),
                            //         // self.db.linkage_table().vec_constructor(self.db.entity_uid(
                            //         //     ranged_ty.route.generic_arguments[0].as_entity_route(),
                            //         // )),
                            //     },
                            //     expr.clone(),
                            // )),
                            _ => todo!(),
                        }
                    };
                self.push_instruction(Instruction::new(instruction_kind, expr.clone()))
            }
        }
    }

    fn compile_element_access(&mut self, expr: Arc<EagerExpr>, opds: &[Arc<EagerExpr>]) {
        self.push_instruction(Instruction::new(
            InstructionKind::RoutineCallCompiled {
                linkage: self.db.element_access_linkage(
                    opds.map(|opd| opd.ty),
                    match expr.contract {
                        EagerContract::Pure => {
                            if self.db.is_copyable(expr.ty).unwrap() {
                                Binding::Copy
                            } else {
                                todo!()
                            }
                        }
                        EagerContract::GlobalRef => todo!(),
                        EagerContract::Move => todo!(),
                        EagerContract::LetInit => {
                            if self.db.is_copyable(expr.ty).unwrap() {
                                Binding::Copy
                            } else {
                                todo!()
                            }
                        }
                        EagerContract::VarInit => todo!(),
                        EagerContract::Return => {
                            if self.db.is_copyable(expr.ty).unwrap() {
                                Binding::Copy
                            } else {
                                Binding::Move
                            }
                        }
                        EagerContract::RefMut => Binding::RefMut,
                        EagerContract::MoveMut => todo!(),
                        EagerContract::Exec => todo!(),
                        EagerContract::UseMemberForLetInit => todo!(),
                        EagerContract::UseMemberForVarInit => todo!(),
                    },
                ),
            },
            expr,
        ))
    }

    fn method_call_instruction_kind(
        &self,
        this_ty: EntityRoutePtr,
        this_ty_decl: &TyDecl,
        method_route: EntityRoutePtr,
        method_ident: CustomIdentifier,
        binding: Binding,
    ) -> InstructionKind {
        if let Some(linkage) = self.db.method_linkage(method_route, binding) {
            InstructionKind::RoutineCallCompiled { linkage }
        } else {
            match this_ty_decl.kind {
                TyKind::Struct => todo!(),
                TyKind::Enum => todo!(),
                TyKind::Record => todo!(),
                TyKind::Vec => {
                    todo!()
                    // let linkage = self.db.virtual_vec_method_linkages()[method_ident].1;
                    // InstructionKind::RoutineCallCompiled { linkage }
                }
                _ => {
                    p!(this_ty_decl.kind, method_ident);
                    todo!()
                }
            }
        }
    }
}
