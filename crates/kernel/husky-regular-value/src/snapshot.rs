use crate::*;
use std::sync::Arc;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
///
/// we use Arc for everything on heap to reduce clone costs
#[cfg(feature = "vm_support")]
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum __RegularValueSnapshot {
    /// useful for snapshot caching on stack
    None,
    Moved,
    Unit(()),
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(u128),
    R8(u8),
    R16(u16),
    R32(u32),
    R64(u64),
    R128(u128),
    RSize(u128),
    F32(f32),
    F64(f64),
    StringLiteral(StringLiteralId),
    Box(Arc<dyn __RegularSnapshotDyn>),
    Leash(&'static dyn __RegularStaticDyn),
    SizedRef(Arc<dyn __RegularSnapshotDyn>),
    SizedRefMut(Arc<dyn __RegularSnapshotDyn>),
    OptionBox(Option<Arc<dyn __RegularSnapshotDyn>>),
    OptionLeash(Option<&'static dyn __RegularStaticDyn>),
    OptionSizedRef(Option<Arc<dyn __RegularSnapshotDyn>>),
    OptionSizedRefMut(Option<Arc<dyn __RegularSnapshotDyn>>),
    Intrinsic(Arc<dyn __RegularSnapshotDyn>),
}

impl From<&__RegularValue> for __RegularValueSnapshot {
    fn from(value: &__RegularValue) -> Self {
        match value {
            __RegularValue::Moved => __RegularValueSnapshot::Moved,
            __RegularValue::Unit(_) => __RegularValueSnapshot::Unit(()),
            __RegularValue::Bool(val) => __RegularValueSnapshot::Bool(*val),
            __RegularValue::Char(val) => __RegularValueSnapshot::Char(*val),
            __RegularValue::I8(val) => __RegularValueSnapshot::I8(*val),
            __RegularValue::I16(val) => __RegularValueSnapshot::I16(*val),
            __RegularValue::I32(val) => __RegularValueSnapshot::I32(*val),
            __RegularValue::I64(val) => __RegularValueSnapshot::I64(*val),
            __RegularValue::I128(val) => __RegularValueSnapshot::I128(*val),
            __RegularValue::ISize(val) => __RegularValueSnapshot::ISize(*val),
            __RegularValue::U8(val) => __RegularValueSnapshot::U8(*val),
            __RegularValue::U16(val) => __RegularValueSnapshot::U16(*val),
            __RegularValue::U32(val) => __RegularValueSnapshot::U32(*val),
            __RegularValue::U64(val) => __RegularValueSnapshot::U64(*val),
            __RegularValue::U128(val) => __RegularValueSnapshot::U128(*val),
            __RegularValue::USize(val) => __RegularValueSnapshot::USize(*val),
            __RegularValue::R8(val) => __RegularValueSnapshot::R8(*val),
            __RegularValue::R16(val) => __RegularValueSnapshot::R16(*val),
            __RegularValue::R32(val) => __RegularValueSnapshot::R32(*val),
            __RegularValue::R64(val) => __RegularValueSnapshot::R64(*val),
            __RegularValue::R128(val) => __RegularValueSnapshot::R128(*val),
            __RegularValue::RSize(val) => __RegularValueSnapshot::RSize(*val),
            __RegularValue::F32(val) => __RegularValueSnapshot::F32(*val),
            __RegularValue::F64(val) => __RegularValueSnapshot::F64(*val),
            __RegularValue::StringLiteral(id) => __RegularValueSnapshot::StringLiteral(*id),
            __RegularValue::Box(box_value) => {
                __RegularValueSnapshot::Box(box_value.clone_into_arc_snapshot())
            }
            __RegularValue::Leash(_) => todo!(),
            __RegularValue::SizedRef(_) => todo!(),
            __RegularValue::SizedRefMut(_) => todo!(),
            __RegularValue::OptionBox(_) => todo!(),
            __RegularValue::OptionLeash(_) => todo!(),
            __RegularValue::OptionSizedRef(_) => todo!(),
            __RegularValue::OptionSizedRefMut(_) => todo!(),
            __RegularValue::Intrinsic(_) => todo!(),
        }
    }
}
