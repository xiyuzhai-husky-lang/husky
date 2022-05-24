use crate::*;

use entity_kind::TyKind;
use infer_decl::TyDecl;
use map_collect::MapCollect;
use static_defn::LinkageSource;
use vm::*;

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_eager_expr(&mut self, expr: &Arc<EagerExpr>) {
        match expr.variant {
            EagerExprVariant::Variable(varname) => {
                let stack_idx = self.sheet.variable_stack.stack_idx(varname);
                self.push_instruction(Instruction::new(
                    InstructionVariant::PushVariable {
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
                InstructionVariant::PushPrimitiveLiteral(value),
                expr.clone(),
            )),
            EagerExprVariant::Bracketed(ref expr) => self.compile_eager_expr(expr),
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.compile_opn(opn_variant, opds, expr),
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::ThisData => self.push_instruction(Instruction::new(
                InstructionVariant::PushVariable {
                    varname: ContextualIdentifier::ThisData.into(),
                    stack_idx: StackIdx::this(),
                    binding: expr.qualified_ty.qual.binding(expr.contract),
                    range: expr.range,
                    ty: expr.ty,
                },
                expr.clone(),
            )),
            EagerExprVariant::EnumKindLiteral(route) => self.push_instruction(Instruction::new(
                InstructionVariant::PushEnumKindLiteral(EnumKindValue {
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
            self.compile_eager_expr(opd);
        }
        match opn_kind {
            EagerOpnVariant::Binary { opr, this_ty } => {
                let ins_kind = InstructionVariant::OprOpn {
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
                    InstructionVariant::OprOpn {
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
                    SuffixOpr::Incr => InstructionVariant::OprOpn {
                        opn: OprOpn::Incr,
                        this_ty: *this_ty,
                        this_range: opds[0].range,
                    },
                    SuffixOpr::Decr => InstructionVariant::OprOpn {
                        opn: OprOpn::Decr,
                        this_ty: *this_ty,
                        this_range: opds[0].range,
                    },
                    SuffixOpr::AsTy(ty) => InstructionVariant::OprOpn {
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
                            InstructionVariant::FieldAccessCompiled {
                                linkage: field_access_fp,
                            }
                        } else {
                            let this_ty_decl = self.db.ty_decl(*this_ty).unwrap();
                            InstructionVariant::FieldAccessInterpreted {
                                field_idx: this_ty_decl
                                    .field_idx(ranged_ident.ident)
                                    .try_into()
                                    .unwrap(),
                                field_access_contract: expr.contract,
                            }
                        }
                    }
                    SuffixOpr::WithTy(_) => todo!(),
                };
                let instruction = Instruction::new(ins_kind, expr.clone());
                self.push_instruction(instruction)
            }
            EagerOpnVariant::RoutineCall(routine) => {
                if let Some(linkage) = self.db.routine_linkage(routine.route) {
                    self.push_instruction(Instruction::new(
                        InstructionVariant::CallCompiled { linkage },
                        expr.clone(),
                    ))
                } else {
                    self.push_instruction(Instruction::new(
                        InstructionVariant::CallInterpreted {
                            routine_uid: self.db.entity_uid(routine.route),
                            nargs: opds.len() as u8,
                            has_this: false,
                        },
                        expr.clone(),
                    ))
                }
            }
            EagerOpnVariant::PatternCall => todo!(),
            EagerOpnVariant::FieldAccess { field_liason } => {
                todo!()
            }
            EagerOpnVariant::MethodCall {
                method_ident,
                ref this_ty_decl,
                method_route,
            } => {
                let this = &opds[0];
                self.push_instruction(Instruction::new(
                    self.method_call_instruction_variant(
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
                        InstructionVariant::CallCompiled { linkage }
                    } else {
                        match ty_decl.kind {
                            TyKind::Struct => InstructionVariant::NewVirtualStruct {
                                fields: ty_decl
                                    .fields()
                                    .map(|decl| (decl.ident, decl.liason))
                                    .collect(),
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
            InstructionVariant::CallCompiled {
                linkage: self.db.element_access_linkage(
                    opds.map(|opd| opd.ty),
                    match expr.contract {
                        EagerContract::Pure => {
                            if self.db.is_copyable(expr.ty).unwrap() {
                                Binding::Copy
                            } else {
                                Binding::Ref
                            }
                        }
                        EagerContract::GlobalRef => todo!(),
                        EagerContract::Move => todo!(),
                        EagerContract::LetInit => {
                            if self.db.is_copyable(expr.ty).unwrap() {
                                Binding::Copy
                            } else {
                                Binding::Ref
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

    fn method_call_instruction_variant(
        &self,
        this_ty: EntityRoutePtr,
        this_ty_decl: &TyDecl,
        method_route: EntityRoutePtr,
        method_ident: CustomIdentifier,
        this_binding: Binding,
    ) -> InstructionVariant {
        if let Some(linkage) = self.db.method_linkage(method_route, this_binding) {
            InstructionVariant::CallCompiled { linkage }
        } else {
            let method_uid = self.db.entity_uid(method_route);
            let method_decl = self.db.method_decl(method_route).unwrap();
            InstructionVariant::CallInterpreted {
                routine_uid: method_uid,
                nargs: (method_decl.parameters.len() + 1).try_into().unwrap(),
                has_this: true,
            }
        }
    }
}
