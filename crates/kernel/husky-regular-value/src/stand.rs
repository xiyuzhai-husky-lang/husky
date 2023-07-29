use crate::*;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
///
/// stand is used to recover RefMut from snapshot
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum __RegularValueStand {
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
    Intrinsic(Box<dyn __RegularDyn>),
    Box(Box<dyn __RegularDyn>),
    Leash(&'static dyn __RegularDyn),
    SizedRef(Arc<dyn __RegularDyn>),
    SizedRefMut(Box<dyn __RegularDyn>),
    OptionBox(Option<Box<dyn __RegularDyn>>),
    OptionLeash(Option<&'static dyn __RegularDyn>),
    OptionSizedRef(Option<Arc<dyn __RegularDyn>>),
    OptionSizedRefMut(Option<Box<dyn __RegularDyn>>),
}

impl From<&mut __RegularValueStand> for __RegularValue {
    fn from(stand: &mut __RegularValueStand) -> Self {
        match stand {
            __RegularValueStand::Consumed => unreachable!(),
            __RegularValueStand::Moved => __RegularValue::Moved,
            __RegularValueStand::Unit(_) => __RegularValue::Unit(()),
            __RegularValueStand::Bool(b) => __RegularValue::Bool(*b),
            __RegularValueStand::Char(c) => __RegularValue::Char(*c),
            __RegularValueStand::I8(n) => __RegularValue::I8(*n),
            __RegularValueStand::I16(n) => __RegularValue::I16(*n),
            __RegularValueStand::I32(n) => __RegularValue::I32(*n),
            __RegularValueStand::I64(n) => __RegularValue::I64(*n),
            __RegularValueStand::I128(n) => __RegularValue::I128(*n),
            __RegularValueStand::ISize(n) => __RegularValue::ISize(*n),
            __RegularValueStand::U8(n) => __RegularValue::U8(*n),
            __RegularValueStand::U16(n) => __RegularValue::U16(*n),
            __RegularValueStand::U32(n) => __RegularValue::U32(*n),
            __RegularValueStand::U64(n) => __RegularValue::U64(*n),
            __RegularValueStand::U128(n) => __RegularValue::U128(*n),
            __RegularValueStand::USize(n) => __RegularValue::USize(*n),
            // Assuming that R8, R16, etc. are other variant types. Replace as necessary.
            __RegularValueStand::R8(n) => __RegularValue::R8(*n),
            __RegularValueStand::R16(n) => __RegularValue::R16(*n),
            __RegularValueStand::R32(n) => __RegularValue::R32(*n),
            __RegularValueStand::R64(n) => __RegularValue::R64(*n),
            __RegularValueStand::R128(n) => __RegularValue::R128(*n),
            __RegularValueStand::RSize(n) => __RegularValue::RSize(*n),
            __RegularValueStand::F32(n) => __RegularValue::F32(*n),
            __RegularValueStand::F64(n) => __RegularValue::F64(*n),
            __RegularValueStand::StringLiteral(id) => __RegularValue::StringLiteral(*id),
            __RegularValueStand::Intrinsic(_) => {
                let __RegularValueStand::Intrinsic(box_value) =
                    std::mem::replace(stand, __RegularValueStand::Consumed)
                else {
                    unreachable!()
                };
                __RegularValue::Intrinsic(box_value)
            }
            __RegularValueStand::Box(_) => {
                let __RegularValueStand::Box(box_value) =
                    std::mem::replace(stand, __RegularValueStand::Consumed)
                else {
                    unreachable!()
                };
                __RegularValue::Box(box_value)
            }
            __RegularValueStand::Leash(_) => todo!(),
            __RegularValueStand::SizedRef(arc_value) => {
                __RegularValue::SizedRef(&**arc_value as *const dyn __RegularDyn)
            }
            __RegularValueStand::SizedRefMut(boxed_value) => {
                __RegularValue::SizedRefMut(&mut **boxed_value as *mut dyn __RegularDyn)
            }
            __RegularValueStand::OptionBox(opt_box_value) => {
                let __RegularValueStand::OptionBox(opt_box_value) =
                    std::mem::replace(stand, __RegularValueStand::Consumed)
                else {
                    unreachable!()
                };
                __RegularValue::OptionBox(opt_box_value)
            }
            __RegularValueStand::OptionLeash(leash) => __RegularValue::OptionLeash(*leash),
            __RegularValueStand::OptionSizedRef(_) => todo!(),
            __RegularValueStand::OptionSizedRefMut(_) => todo!(),
        }
    }
}
