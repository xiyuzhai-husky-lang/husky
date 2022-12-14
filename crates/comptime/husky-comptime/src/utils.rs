use husky_trace_protocol::Label;

use crate::*;

impl AnalysisHost {}

// todo: move this to vm
#[derive(Debug)]
pub enum __RegisterDowncastResult<T> {
    Value(T),
    None { number_of_somes: u8 },
    Unreturned,
}

pub(crate) fn convert_i32_register_to_label<'eval>(
    value: &__Register<'eval>,
) -> __RegisterDowncastResult<Label> {
    todo!()
    // match value.data_kind() {
    //     __RegisterDataKind::PrimitiveValue => todo!(),
    //     __RegisterDataKind::Box => todo!(),
    //     __RegisterDataKind::EvalRef => __RegisterDowncastResult::Value(Label(value.downcast_i32())),
    //     __RegisterDataKind::TempRef => todo!(),
    //     __RegisterDataKind::TempMut => todo!(),
    //     __RegisterDataKind::Moved => todo!(),
    //     __RegisterDataKind::SomeNone => todo!(),
    //     // __RegisterDowncastResult::SomeNone,
    //     __RegisterDataKind::Unreturned => __RegisterDowncastResult::Unreturned,
    // }
}

pub fn convert_enum_register_to_label<'eval>(
    value: &__Register<'eval>,
) -> __RegisterDowncastResult<Label> {
    todo!()
    // if value.vtable.typename_str_hash_u64 != __VIRTUAL_ENUM_VTABLE.typename_str_hash_u64 {
    //     match value.data_kind() {
    //         __RegisterDataKind::SomeNone | __RegisterDataKind::Unreturned => (),
    //         _ => {
    //             panic!(
    //                 "expect virtual enum, but got {:?} of type`{}` instead",
    //                 value.data_kind(),
    //                 value.vtable.typename_str
    //             )
    //         }
    //     }
    // }
    // match value.data_kind() {
    //     __RegisterDataKind::PrimitiveValue => todo!(),
    //     __RegisterDataKind::Box | __RegisterDataKind::EvalRef => {
    //         __RegisterDowncastResult::Value(Label(
    //             value
    //                 .downcast_temp_ref::<__VirtualEnum>(&__VIRTUAL_ENUM_VTABLE)
    //                 .kind_idx,
    //         ))
    //     }
    //     __RegisterDataKind::TempRef => todo!(),
    //     __RegisterDataKind::TempMut => todo!(),
    //     __RegisterDataKind::Moved => todo!(),
    //     __RegisterDataKind::SomeNone => __RegisterDowncastResult::None {
    //         number_of_somes: unsafe { value.data().as_number_of_somes },
    //     },
    //     __RegisterDataKind::Unreturned => __RegisterDowncastResult::Unreturned,
    // }
}
