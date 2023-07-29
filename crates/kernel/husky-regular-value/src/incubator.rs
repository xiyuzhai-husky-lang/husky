use crate::*;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
///
/// stand is used to recover RefMut from snapshot
#[cfg(feature = "vm_support")]
#[derive(Debug)]
#[repr(u8)]
pub enum __RegularValueIncubator {
    Consumed,
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
    Intrinsic(Box<dyn __RegularIncubatorDyn>),
    Box(Box<dyn __RegularIncubatorDyn>),
    Leash(&'static dyn __RegularStaticDyn),
    SizedRef(Arc<dyn __RegularIncubatorDyn>),
    SizedRefMut(Box<dyn __RegularIncubatorDyn>),
    OptionBox(Option<Box<dyn __RegularIncubatorDyn>>),
    OptionLeash(Option<&'static dyn __RegularStaticDyn>),
    OptionSizedRef(Option<Arc<dyn __RegularIncubatorDyn>>),
    OptionSizedRefMut(Option<Box<dyn __RegularIncubatorDyn>>),
}

impl From<&mut __RegularValueIncubator> for __RegularValue {
    fn from(stand: &mut __RegularValueIncubator) -> Self {
        match stand {
            __RegularValueIncubator::Consumed => unreachable!(),
            __RegularValueIncubator::Moved => __RegularValue::Moved,
            __RegularValueIncubator::Unit(_) => __RegularValue::Unit(()),
            __RegularValueIncubator::Bool(b) => __RegularValue::Bool(*b),
            __RegularValueIncubator::Char(c) => __RegularValue::Char(*c),
            __RegularValueIncubator::I8(n) => __RegularValue::I8(*n),
            __RegularValueIncubator::I16(n) => __RegularValue::I16(*n),
            __RegularValueIncubator::I32(n) => __RegularValue::I32(*n),
            __RegularValueIncubator::I64(n) => __RegularValue::I64(*n),
            __RegularValueIncubator::I128(n) => __RegularValue::I128(*n),
            __RegularValueIncubator::ISize(n) => __RegularValue::ISize(*n),
            __RegularValueIncubator::U8(n) => __RegularValue::U8(*n),
            __RegularValueIncubator::U16(n) => __RegularValue::U16(*n),
            __RegularValueIncubator::U32(n) => __RegularValue::U32(*n),
            __RegularValueIncubator::U64(n) => __RegularValue::U64(*n),
            __RegularValueIncubator::U128(n) => __RegularValue::U128(*n),
            __RegularValueIncubator::USize(n) => __RegularValue::USize(*n),
            // Assuming that R8, R16, etc. are other variant types. Replace as necessary.
            __RegularValueIncubator::R8(n) => __RegularValue::R8(*n),
            __RegularValueIncubator::R16(n) => __RegularValue::R16(*n),
            __RegularValueIncubator::R32(n) => __RegularValue::R32(*n),
            __RegularValueIncubator::R64(n) => __RegularValue::R64(*n),
            __RegularValueIncubator::R128(n) => __RegularValue::R128(*n),
            __RegularValueIncubator::RSize(n) => __RegularValue::RSize(*n),
            __RegularValueIncubator::F32(n) => __RegularValue::F32(*n),
            __RegularValueIncubator::F64(n) => __RegularValue::F64(*n),
            __RegularValueIncubator::StringLiteral(id) => __RegularValue::StringLiteral(*id),
            __RegularValueIncubator::Intrinsic(box_value) => {
                __RegularValue::Intrinsic(box_value.incubate_box())
            }
            __RegularValueIncubator::Box(box_value) => {
                __RegularValue::Box(box_value.incubate_box())
            }
            __RegularValueIncubator::Leash(leash) => __RegularValue::Leash(*leash),
            __RegularValueIncubator::SizedRef(arc_value) => {
                __RegularValue::SizedRef(arc_value.incubate_sized_ref())
            }
            __RegularValueIncubator::SizedRefMut(boxed_value) => {
                __RegularValue::SizedRef(boxed_value.incubate_sized_mut())
            }
            __RegularValueIncubator::OptionBox(opt_box_value) => __RegularValue::OptionBox(
                opt_box_value
                    .as_mut()
                    .map(|box_value| box_value.incubate_box()),
            ),
            __RegularValueIncubator::OptionLeash(leash) => __RegularValue::OptionLeash(*leash),
            __RegularValueIncubator::OptionSizedRef(opt_arc_value) => {
                __RegularValue::OptionSizedRef(
                    opt_arc_value
                        .as_ref()
                        .map(|arc_value| arc_value.incubate_sized_ref()),
                )
            }
            __RegularValueIncubator::OptionSizedRefMut(opt_box_value) => {
                __RegularValue::OptionSizedRefMut(
                    opt_box_value
                        .as_mut()
                        .map(|box_value| box_value.incubate_sized_mut()),
                )
            }
        }
    }
}
