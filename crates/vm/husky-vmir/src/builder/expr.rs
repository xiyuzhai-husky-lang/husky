use super::*;
use crate::vmir::Binding;
use husky_entity_path::AssocItemPath;
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprIdx};
use husky_hir_opr::{binary::HirBinaryOpr, suffix::HirSuffixOpr};
use husky_linkage::template_argument::ty::LinType;

impl<'a, 'b> VmirBlockBuilder<'a, 'b> {
    pub(super) fn compile_eager_expr(
        &mut self,
        expr: HirEagerExprIdx,
        output_stack_idx: VMStackIdx,
        discard: bool,
    ) {
        match self.expr_arena[expr].data {
            HirEagerExprData::Literal(_) => todo!(),
            HirEagerExprData::PrincipalEntityPath(_) => todo!(),
            HirEagerExprData::AssocFn { assoc_item_path } => todo!(),
            HirEagerExprData::ConstSymbol { ident } => todo!(),
            HirEagerExprData::Variable(_) => todo!(),
            HirEagerExprData::Binary { lopd, opr, ropd } => todo!(),
            HirEagerExprData::Be { src, ref target } => todo!(),
            HirEagerExprData::Prefix {
                opr,
                opd_hir_expr_idx,
            } => todo!(),
            HirEagerExprData::Suffix {
                opd_hir_expr_idx,
                opr,
            } => todo!(),
            HirEagerExprData::Unveil {
                unveil_assoc_fn_path,
                ref instantiation,
                return_ty,
                opd_hir_expr_idx,
            } => todo!(),
            HirEagerExprData::Unwrap { opd_hir_expr_idx } => todo!(),
            HirEagerExprData::As { opd, ty } => todo!(),
            HirEagerExprData::TypeConstructorFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => todo!(),
            HirEagerExprData::TypeVariantConstructorCall {
                path,
                ref instantiation,
                ref item_groups,
            } => todo!(),
            HirEagerExprData::FunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => todo!(),
            HirEagerExprData::AssocFunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => todo!(),
            HirEagerExprData::PropsStructField {
                owner_hir_expr_idx,
                ident,
                field_ty,
            } => todo!(),
            HirEagerExprData::MemoizedField {
                owner_hir_expr_idx,
                ident,
                path,
            } => todo!(),
            HirEagerExprData::MethodFnCall {
                self_argument,
                self_contract,
                ident,
                path,
                ref instantiation,
                ref item_groups,
            } => todo!(),
            HirEagerExprData::NewTuple { ref items } => todo!(),
            HirEagerExprData::Index {
                owner_hir_expr_idx,
                ref items,
            } => todo!(),
            HirEagerExprData::NewList {
                ref items,
                element_ty,
            } => todo!(),
            HirEagerExprData::Block { stmts } => todo!(),
            HirEagerExprData::EmptyHtmlTag {
                function_ident,
                ref arguments,
            } => todo!(),
            HirEagerExprData::Todo => todo!(),
            HirEagerExprData::Unreachable => todo!(),
        }
        // match expr.variant {
        //     EagerExprVariant::Variable { varname, binding } => {
        //         // no discard
        //         assert!(!discard);
        //         let stack_idx = self.sheet.variable_stack.stack_idx(varname);
        //         self.push_instruction(Vmir::new(
        //             VmirData::PushVariable {
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
        //         self.push_instruction(Vmir::new(
        //             VmirData::PushLiteralValue {
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
        //     EagerExprVariant::ThisValue { binding } => self.push_instruction(Vmir::new(
        //         VmirData::PushVariable {
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
        //         VmirGenContext::Normal => {
        //             self.push_instruction(Vmir::new(
        //                 VmirData::PushVariable {
        //                     varname: ContextualIdent::ThisValue.into(),
        //                     stack_idx: VMStackIdx::this(),
        //                     binding: this_binding,
        //                     range: expr.range,
        //                     ty: this_ty,
        //                     explicit: false,
        //                 },
        //                 expr.clone(),
        //             ));
        //             self.push_instruction(Vmir::new(
        //                 if let Some(linkage) = self.db.field_linkage_resolved(
        //                     this_ty.intrinsic(),
        //                     field_ident.ident,
        //                     field_binding,
        //                 ) {
        //                     VmirData::CallRoutine {
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
        //                     //         VmirData::VirtualStructField {
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
        //         VmirGenContext::NewVirtualStruct { output_stack_idx } => self
        //             .push_instruction(Vmir::new(
        //                 VmirData::PushVariable {
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
        //     EagerExprVariant::EnumKindLiteral(route) => self.push_instruction(Vmir::new(
        //         VmirData::PushLiteralValue {
        //             value: __VirtualEnum {
        //                 kind_idx: self.db.enum_literal_to_i32(route),
        //             }
        //             .to_register(),
        //             ty: expr.intrinsic_ty(),
        //             explicit: true,
        //         },
        //         expr.clone(),
        //     )),
        //     EagerExprVariant::EntityFeature { route } => self.push_instruction(Vmir::new(
        //         VmirData::EntityFeature {
        //             feature_uid: self.db.item_uid(route),
        //             ty: expr.intrinsic_ty(),
        //         },
        //         expr.clone(),
        //     )),
        //     EagerExprVariant::EntityThickFp { route } => self.push_instruction(Vmir::new(
        //         VmirData::PushEntityFp {
        //             opt_linkage: self.db.routine_linkage(route),
        //             opt_instruction_region: self.db.item_instruction_region(route),
        //             ty: expr.intrinsic_ty(),
        //         },
        //         expr.clone(),
        //     )),
        // }
        // match expr.implicit_conversion {
        //     ImplicitConversion::None => (),
        //     ImplicitConversion::WrapInSome { number_of_somes } => {
        //         self.push_instruction(Vmir::new(
        //             VmirData::WrapInSome { number_of_somes },
        //             expr.clone(),
        //         ))
        //     }
        //     ImplicitConversion::ConvertToBool => todo!(),
        // }
    }

    fn compile_suffix(
        &mut self,
        _opr: &HirSuffixOpr,
        _opds: &[HirEagerExprIdx],
        _expr: HirEagerExprIdx,
        _discard: bool,
    ) {
        todo!()
    }

    fn compile_binary_opn(
        &mut self,
        opr: HirBinaryOpr,
        opds: &[HirEagerExprIdx],
        expr: HirEagerExprIdx,
        discard: bool,
    ) {
        todo!()
    }

    fn compile_prefix_opn(
        &mut self,
        prefix: HirBinaryOpr,
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
        todo!()
        // let index_linkage = self.db.index_linkage(opds.map(|opd| opd.intrinsic_ty()));
        // self.push_instruction(Vmir::new(
        //     VmirData::CallRoutine {
        //         return_ty: expr.intrinsic_ty(),
        //         nargs: opds.len().try_into().unwrap(),
        //         resolved_linkage: index_linkage.bind(element_binding),
        //         discard: false,
        //     },
        //     expr,
        // ))
    }

    fn method_call_instruction_variant(
        &self,
        method_item_path: AssocItemPath,
        return_ty: LinType,
        nargs: u8,
        discard: bool,
    ) -> VmirData {
        todo!()
        // let method_uid = self.db.item_uid(method_route);
        // let call_fugitive_syn_decl = self.db.item_call_fugitive_syn_decl(method_route).unwrap();
        // VmirData::CallInterpreted {
        //     routine_uid: method_uid,
        //     nargs: (call_fugitive_syn_decl.primary_parameters.len() + 1)
        //         .try_into()
        //         .unwrap(),
        //     return_ty,
        //     discard,
        // }
    }
}
