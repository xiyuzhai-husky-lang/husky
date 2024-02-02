use crate::*;

use husky_linkage_table::ResolveLinkage;
use husky_vm::{
    __root::{__ASSIGN_LINKAGE, __EQ_LINKAGE, __NEQ_LINKAGE, __VALUE_CALL_LINKAGE},
    *,
};
use map_collect::MapCollect;

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_eager_expr(
        &mut self,
        expr: HirEagerExprIdx,
        output_stack_idx: VMStackIdx,
        discard: bool,
    ) {
        todo!()
        // match expr.variant {
        //     EagerExprVariant::Variable { varname, binding } => {
        //         // no discard
        //         assert!(!discard);
        //         let stack_idx = self.sheet.variable_stack.stack_idx(varname);
        //         self.push_instruction(Instruction::new(
        //             InstructionData::PushVariable {
        //                 varname: varname.into(),
        //                 stack_idx,
        //                 binding,
        //                 range: expr.range,
        //                 ty: expr.intrinsic_ty(),
        //                 explicit: true,
        //             },
        //             expr.clone(),
        //         ))
        //     }
        //     EagerExprVariant::PrimitiveLiteral(value) => {
        //         // no discard
        //         assert!(!discard);
        //         self.push_instruction(Instruction::new(
        //             InstructionData::PushLiteralValue {
        //                 value: convert_primitive_literal_to_register(value, expr.intrinsic_ty()),
        //                 explicit: true,
        //                 ty: expr.intrinsic_ty(),
        //             },
        //             expr.clone(),
        //         ))
        //     }
        //     EagerExprVariant::Bracketed(ref expr) => {
        //         self.compile_eager_expr(expr, output_stack_idx, discard)
        //     }
        //     EagerExprVariant::Opn {
        //         ref opn_variant,
        //         ref opds,
        //     } => self.compile_opn(opn_variant, opds, expr, output_stack_idx, discard),
        //     EagerExprVariant::Lambda(_, _) => todo!(),
        //     EagerExprVariant::ThisValue { binding } => self.push_instruction(Instruction::new(
        //         InstructionData::PushVariable {
        //             varname: ContextualIdent::ThisValue.into(),
        //             stack_idx: VMStackIdx::this(),
        //             binding,
        //             range: expr.range,
        //             ty: expr.intrinsic_ty(),
        //             explicit: true,
        //         },
        //         expr.clone(),
        //     )),
        //     EagerExprVariant::ThisField {
        //         field_ident,
        //         field_idx,
        //         this_ty,
        //         this_binding,
        //         field_binding,
        //     } => match self.context.value() {
        //         InstructionGenContext::Normal => {
        //             self.push_instruction(Instruction::new(
        //                 InstructionData::PushVariable {
        //                     varname: ContextualIdent::ThisValue.into(),
        //                     stack_idx: VMStackIdx::this(),
        //                     binding: this_binding,
        //                     range: expr.range,
        //                     ty: this_ty,
        //                     explicit: false,
        //                 },
        //                 expr.clone(),
        //             ));
        //             self.push_instruction(Instruction::new(
        //                 if let Some(linkage) = self.db.field_linkage_resolved(
        //                     this_ty.intrinsic(),
        //                     field_ident.ident,
        //                     field_binding,
        //                 ) {
        //                     InstructionData::CallRoutine {
        //                         return_ty: expr.intrinsic_ty(),
        //                         nargs: 1,
        //                         resolved_linkage: linkage,
        //                         discard,
        //                     }
        //                 } else {
        //                     todo!()
        //                     // let this_ty_decl = self.db.ty_decl(this_ty).unwrap();
        //                     // let field_decl = this_ty_decl.field_decl(field_ident).unwrap();
        //                     // match field_decl.field_kind {
        //                     //     FieldKind::StructRegular
        //                     //     | FieldKind::StructDefault
        //                     //     | FieldKind::StructDerived => {
        //                     //         InstructionData::VirtualStructField {
        //                     //             field_idx: this_ty_decl
        //                     //                 .field_idx(field_ident.ident)
        //                     //                 .try_into()
        //                     //                 .unwrap(),
        //                     //             field_binding,
        //                     //             field_ty: expr.intrinsic_ty(),
        //                     //         }
        //                     //     }
        //                     //     FieldKind::StructMemo => todo!(),
        //                     //     FieldKind::RecordRegular => todo!(),
        //                     //     FieldKind::RecordProperty => todo!(),
        //                     // }
        //                 },
        //                 expr.clone(),
        //             ))
        //         }
        //         InstructionGenContext::NewVirtualStruct { output_stack_idx } => self
        //             .push_instruction(Instruction::new(
        //                 InstructionData::PushVariable {
        //                     varname: field_ident.ident.into(),
        //                     stack_idx: output_stack_idx + field_idx,
        //                     binding: field_binding,
        //                     range: expr.range,
        //                     ty: expr.intrinsic_ty(),
        //                     explicit: true,
        //                 },
        //                 expr.clone(),
        //             )),
        //     },
        //     EagerExprVariant::EnumKindLiteral(route) => self.push_instruction(Instruction::new(
        //         InstructionData::PushLiteralValue {
        //             value: __VirtualEnum {
        //                 kind_idx: self.db.enum_literal_to_i32(route),
        //             }
        //             .to_register(),
        //             ty: expr.intrinsic_ty(),
        //             explicit: true,
        //         },
        //         expr.clone(),
        //     )),
        //     EagerExprVariant::EntityFeature { route } => self.push_instruction(Instruction::new(
        //         InstructionData::EntityFeature {
        //             feature_uid: self.db.item_uid(route),
        //             ty: expr.intrinsic_ty(),
        //         },
        //         expr.clone(),
        //     )),
        //     EagerExprVariant::EntityThickFp { route } => self.push_instruction(Instruction::new(
        //         InstructionData::PushEntityFp {
        //             opt_linkage: self.db.routine_linkage(route),
        //             opt_instruction_sheet: self.db.item_instruction_sheet(route),
        //             ty: expr.intrinsic_ty(),
        //         },
        //         expr.clone(),
        //     )),
        // }
        // match expr.implicit_conversion {
        //     ImplicitConversion::None => (),
        //     ImplicitConversion::WrapInSome { number_of_somes } => {
        //         self.push_instruction(Instruction::new(
        //             InstructionData::WrapInSome { number_of_somes },
        //             expr.clone(),
        //         ))
        //     }
        //     ImplicitConversion::ConvertToBool => todo!(),
        // }
    }

    fn compile_opn(
        &mut self,
        opn_variant: &EagerOpnVariant,
        opds: &[HirEagerExprIdx],
        expr: HirEagerExprIdx,
        output_stack_idx: VMStackIdx,
        discard: bool,
    ) {
        todo!()
    }

    fn compile_suffix(
        &mut self,
        _opr: &EagerSuffixOpr,
        _opds: &[HirEagerExprIdx],
        _expr: HirEagerExprIdx,
        _discard: bool,
    ) {
        todo!()
    }

    fn compile_binary_opn(
        &mut self,
        opr: SynBinaryOpr,
        opds: &[HirEagerExprIdx],
        expr: HirEagerExprIdx,
        discard: bool,
    ) {
        todo!()
    }

    fn compile_prefix_opn(
        &mut self,
        prefix: SynPrefixOpr,
        opds: &[HirEagerExprIdx],
        expr: HirEagerExprIdx,
        discard: bool,
    ) {
        todo!()
    }

    fn compile_element_access(
        &mut self,
        expr: HirEagerExprIdx,
        opds: &[HirEagerExprIdx],
        element_binding: Binding,
    ) {
        let index_linkage = self.db.index_linkage(opds.map(|opd| opd.intrinsic_ty()));
        self.push_instruction(Instruction::new(
            InstructionData::CallRoutine {
                return_ty: expr.intrinsic_ty(),
                nargs: opds.len().try_into().unwrap(),
                resolved_linkage: index_linkage.bind(element_binding),
                discard: false,
            },
            expr,
        ))
    }

    fn method_call_instruction_variant(
        &self,
        method_route: EthTerm,
        return_ty: HirType,
        output_binding: Binding,
        nargs: u8,
        discard: bool,
    ) -> InstructionData {
        if let Some(linkage) = self.db.method_linkage(method_route) {
            match linkage {
                __LinkageGroup::Member { .. } => InstructionData::CallRoutine {
                    resolved_linkage: linkage.bind(output_binding),
                    nargs,
                    return_ty,
                    discard,
                },
                __LinkageGroup::Transfer(linkage) => InstructionData::CallRoutine {
                    return_ty,
                    nargs,

                    resolved_linkage: linkage,
                    discard,
                },
                __LinkageGroup::Model(_) => todo!(),
            }
        } else {
            todo!()
            // let method_uid = self.db.item_uid(method_route);
            // let call_fugitive_syn_decl = self.db.item_call_fugitive_syn_decl(method_route).unwrap();
            // InstructionData::CallInterpreted {
            //     routine_uid: method_uid,
            //     nargs: (call_fugitive_syn_decl.primary_parameters.len() + 1)
            //         .try_into()
            //         .unwrap(),
            //     return_ty,
            //     discard,
            // }
        }
    }
}
