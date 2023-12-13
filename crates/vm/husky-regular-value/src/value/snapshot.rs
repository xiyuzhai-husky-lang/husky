use crate::*;
use std::sync::Arc;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
///
/// we use Arc for everything on heap to reduce clone costs
#[cfg(feature = "vm_support")]
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum __RegularSnapshotValue {
    /// useful for snapshot caching on stack
    None,
    Invalid,
    Moved,
    Unit(()),
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    R8(u8),
    R16(u16),
    R32(u32),
    R64(u64),
    R128(u128),
    RSize(usize),
    F32(f32),
    F64(f64),
    StringLiteral(StringLiteralId),
    Box(Arc<dyn __RegularSnapshotDyn>),
    Leash(&'static dyn __RegularStandDyn),
    SizedRef(Arc<dyn __RegularSnapshotDyn>),
    SizedRefMut(Arc<dyn __RegularSnapshotDyn>),
    OptionBox(Option<Arc<dyn __RegularSnapshotDyn>>),
    OptionLeash(Option<&'static dyn __RegularStandDyn>),
    OptionSizedRef(Option<Arc<dyn __RegularSnapshotDyn>>),
    OptionSizedRefMut(Option<Arc<dyn __RegularSnapshotDyn>>),
    Intrinsic(Arc<dyn __RegularSnapshotDyn>),
}

impl RegularValue {
    pub unsafe fn snapshot(&self) -> __RegularSnapshotValue {
        match self {
            RegularValue::Moved => __RegularSnapshotValue::Moved,
            RegularValue::Invalid => __RegularSnapshotValue::Invalid,
            RegularValue::Unit(_) => __RegularSnapshotValue::Unit(()),
            RegularValue::Bool(val) => __RegularSnapshotValue::Bool(*val),
            RegularValue::Char(val) => __RegularSnapshotValue::Char(*val),
            RegularValue::I8(val) => __RegularSnapshotValue::I8(*val),
            RegularValue::I16(val) => __RegularSnapshotValue::I16(*val),
            RegularValue::I32(val) => __RegularSnapshotValue::I32(*val),
            RegularValue::I64(val) => __RegularSnapshotValue::I64(*val),
            RegularValue::I128(val) => __RegularSnapshotValue::I128(*val),
            RegularValue::ISize(val) => __RegularSnapshotValue::ISize(*val),
            RegularValue::U8(val) => __RegularSnapshotValue::U8(*val),
            RegularValue::U16(val) => __RegularSnapshotValue::U16(*val),
            RegularValue::U32(val) => __RegularSnapshotValue::U32(*val),
            RegularValue::U64(val) => __RegularSnapshotValue::U64(*val),
            RegularValue::U128(val) => __RegularSnapshotValue::U128(*val),
            RegularValue::USize(val) => __RegularSnapshotValue::USize(*val),
            RegularValue::R8(val) => __RegularSnapshotValue::R8(*val),
            RegularValue::R16(val) => __RegularSnapshotValue::R16(*val),
            RegularValue::R32(val) => __RegularSnapshotValue::R32(*val),
            RegularValue::R64(val) => __RegularSnapshotValue::R64(*val),
            RegularValue::R128(val) => __RegularSnapshotValue::R128(*val),
            RegularValue::RSize(val) => __RegularSnapshotValue::RSize(*val),
            RegularValue::F32(val) => __RegularSnapshotValue::F32(*val),
            RegularValue::F64(val) => __RegularSnapshotValue::F64(*val),
            RegularValue::StringLiteral(id) => __RegularSnapshotValue::StringLiteral(*id),
            RegularValue::Box(box_value) => {
                __RegularSnapshotValue::Box(box_value.clone_into_arc_snapshot())
            }
            RegularValue::Leash(_) => todo!(),
            RegularValue::SizedRef(_) => todo!(),
            RegularValue::SizedMut(_) => todo!(),
            RegularValue::OptionBox(_) => todo!(),
            RegularValue::OptionLeash(_) => todo!(),
            RegularValue::OptionSizedRef(_) => todo!(),
            RegularValue::OptionSizedMut(_) => todo!(),
            RegularValue::Intrinsic(_) => todo!(),
        }
    }
}
