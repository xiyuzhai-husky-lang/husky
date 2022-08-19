use husky_trace_protocol::Label;

use crate::*;

// impl HuskyComptime {
//     pub fn ty_route_from_static(&self, type_id: std::any::TypeId, text: &str) -> EntityRoutePtr {
//         todo!()
//         // if let Some(ty) = try_get_ty_route(type_id) {
//         //     ty
//         // } else {
//         //     let ty = self.parse_route_from_text(text);
//         //     insert_new_ty_route(type_id, ty);
//         //     ty
//         // }
//     }
// }
impl HuskyComptime {
    pub fn register_to_label_converter(
        &self,
    ) -> for<'eval> fn(&__Register<'eval>) -> __RegisterDowncastResult<Label> {
        let target_output_ty = self.target_output_ty().unwrap();
        let target_output_ty_intrinsic = target_output_ty.intrinsic();

        if target_output_ty_intrinsic == RootIdentifier::I32.into() {
            convert_i32_register_to_label
        } else {
            let target_output_ty_intrinsic_decl = self.ty_decl(target_output_ty_intrinsic).unwrap();
            use entity_kind::TyKind;
            match target_output_ty_intrinsic_decl.ty_kind {
                TyKind::Enum => convert_enum_register_to_label,
                TyKind::Record => todo!(),
                TyKind::Struct => todo!(),
                TyKind::Primitive => todo!(),
                TyKind::Vec => todo!(),
                TyKind::Slice => todo!(),
                TyKind::CyclicSlice => todo!(),
                TyKind::Array => todo!(),
                TyKind::Tuple => todo!(),
                TyKind::Mor => todo!(),
                TyKind::Fp => todo!(),
                TyKind::AssociatedAny => todo!(),
                TyKind::ThisAny => todo!(),
                TyKind::TargetOutputAny => todo!(),
                TyKind::SpatialPlaceholderAny => todo!(),
                TyKind::BoxAny => todo!(),
                TyKind::HigherKind => todo!(),
                TyKind::Ref => todo!(),
                TyKind::Option => todo!(),
            }
        }
    }
}

// todo: move this to vm
pub enum __RegisterDowncastResult<T> {
    Value(T),
    None,
    Unreturned,
}

fn convert_i32_register_to_label<'eval>(
    value: &__Register<'eval>,
) -> __RegisterDowncastResult<Label> {
    match value.data_kind() {
        __RegisterDataKind::PrimitiveValue => todo!(),
        __RegisterDataKind::Box => todo!(),
        __RegisterDataKind::EvalRef => __RegisterDowncastResult::Value(Label(value.downcast_i32())),
        __RegisterDataKind::TempRef => todo!(),
        __RegisterDataKind::TempMut => todo!(),
        __RegisterDataKind::Moved => todo!(),
        __RegisterDataKind::Undefined => __RegisterDowncastResult::None,
        __RegisterDataKind::Unreturned => __RegisterDowncastResult::Unreturned,
    }
}

pub fn convert_enum_register_to_label<'eval>(
    value: &__Register<'eval>,
) -> __RegisterDowncastResult<Label> {
    match value.data_kind() {
        __RegisterDataKind::PrimitiveValue => todo!(),
        __RegisterDataKind::Box | __RegisterDataKind::EvalRef => {
            __RegisterDowncastResult::Value(Label(
                value
                    .downcast_temp_ref::<__VirtualEnum>(&__VIRTUAL_ENUM_VTABLE)
                    .kind_idx,
            ))
        }
        __RegisterDataKind::TempRef => todo!(),
        __RegisterDataKind::TempMut => todo!(),
        __RegisterDataKind::Moved => todo!(),
        __RegisterDataKind::Undefined => __RegisterDowncastResult::None,
        __RegisterDataKind::Unreturned => __RegisterDowncastResult::Unreturned,
    }
}
