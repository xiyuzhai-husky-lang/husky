use crate::*;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
///
/// incubator is used as a medium to recover from snapshot
///
/// incubator must be consumed
#[cfg(feature = "vm_support")]
#[derive(Debug)]
#[repr(u8)]
pub enum __RegularIncubatorValue {
    Moved,
    Invalid,
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
    Intrinsic(Box<dyn __RegularIncubatorDyn>),
    Box(Box<dyn __RegularIncubatorDyn>),
    Leash(&'static dyn __RegularStandDyn),
    SizedRef(Box<dyn __RegularIncubatorDyn>, Box<dyn __RegularStandDyn>),
    SizedRefMut(Box<dyn __RegularIncubatorDyn>, Box<dyn __RegularStandDyn>),
    OptionBox(Option<Box<dyn __RegularIncubatorDyn>>),
    OptionLeash(Option<&'static dyn __RegularStandDyn>),
    OptionSizedRef(
        Option<Box<dyn __RegularIncubatorDyn>>,
        Option<Box<dyn __RegularStandDyn>>,
    ),
    OptionSizedRefMut(
        Option<Box<dyn __RegularIncubatorDyn>>,
        Option<Box<dyn __RegularStandDyn>>,
    ),
}

impl __RegularSnapshotValue {
    pub unsafe fn incubator(&self) -> __RegularIncubatorValue {
        match self {
            __RegularSnapshotValue::None => unreachable!(),
            __RegularSnapshotValue::Invalid => __RegularIncubatorValue::Invalid,
            __RegularSnapshotValue::Moved => __RegularIncubatorValue::Moved,
            __RegularSnapshotValue::Unit(val) => __RegularIncubatorValue::Unit(*val),
            __RegularSnapshotValue::Bool(val) => __RegularIncubatorValue::Bool(*val),
            __RegularSnapshotValue::Char(val) => __RegularIncubatorValue::Char(*val),
            __RegularSnapshotValue::I8(val) => __RegularIncubatorValue::I8(*val),
            __RegularSnapshotValue::I16(val) => __RegularIncubatorValue::I16(*val),
            __RegularSnapshotValue::I32(val) => __RegularIncubatorValue::I32(*val),
            __RegularSnapshotValue::I64(val) => __RegularIncubatorValue::I64(*val),
            __RegularSnapshotValue::I128(val) => __RegularIncubatorValue::I128(*val),
            __RegularSnapshotValue::ISize(val) => __RegularIncubatorValue::ISize(*val),
            __RegularSnapshotValue::U8(val) => __RegularIncubatorValue::U8(*val),
            __RegularSnapshotValue::U16(val) => __RegularIncubatorValue::U16(*val),
            __RegularSnapshotValue::U32(val) => __RegularIncubatorValue::U32(*val),
            __RegularSnapshotValue::U64(val) => __RegularIncubatorValue::U64(*val),
            __RegularSnapshotValue::U128(val) => __RegularIncubatorValue::U128(*val),
            __RegularSnapshotValue::USize(val) => __RegularIncubatorValue::USize(*val),
            __RegularSnapshotValue::R8(val) => __RegularIncubatorValue::R8(*val),
            __RegularSnapshotValue::R16(val) => __RegularIncubatorValue::R16(*val),
            __RegularSnapshotValue::R32(val) => __RegularIncubatorValue::R32(*val),
            __RegularSnapshotValue::R64(val) => __RegularIncubatorValue::R64(*val),
            __RegularSnapshotValue::R128(val) => __RegularIncubatorValue::R128(*val),
            __RegularSnapshotValue::RSize(val) => __RegularIncubatorValue::RSize(*val),
            __RegularSnapshotValue::F32(val) => __RegularIncubatorValue::F32(*val),
            __RegularSnapshotValue::F64(val) => __RegularIncubatorValue::F64(*val),
            __RegularSnapshotValue::StringLiteral(id) => {
                __RegularIncubatorValue::StringLiteral(*id)
            }
            __RegularSnapshotValue::Box(arc_value) => {
                __RegularIncubatorValue::Box(arc_value.clone_into_incubator_box_dyn())
            }
            __RegularSnapshotValue::Leash(_) => todo!(),
            __RegularSnapshotValue::SizedRef(arc_value) => {
                let mut incubator_box_dyn = arc_value.clone_into_incubator_box_dyn();
                let incubated_box_dyn = incubator_box_dyn.incubate_box_dyn();
                __RegularIncubatorValue::SizedRef(incubator_box_dyn, incubated_box_dyn)
            }
            __RegularSnapshotValue::SizedRefMut(arc_value) => {
                let mut incubator_box_dyn = arc_value.clone_into_incubator_box_dyn();
                let incubated_box_dyn = incubator_box_dyn.incubate_box_dyn();
                __RegularIncubatorValue::SizedRefMut(incubator_box_dyn, incubated_box_dyn)
            }
            __RegularSnapshotValue::OptionBox(_) => todo!(),
            __RegularSnapshotValue::OptionLeash(_) => todo!(),
            __RegularSnapshotValue::OptionSizedRef(_) => todo!(),
            __RegularSnapshotValue::OptionSizedRefMut(_) => todo!(),
            __RegularSnapshotValue::Intrinsic(_) => todo!(),
        }
    }
}

impl __RegularIncubatorValue {
    /// must use once and only once
    pub unsafe fn incubate(&mut self) -> __RegularValue {
        match self {
            __RegularIncubatorValue::Invalid => __RegularValue::Invalid,
            __RegularIncubatorValue::Moved => __RegularValue::Moved,
            __RegularIncubatorValue::Unit(_) => __RegularValue::Unit(()),
            __RegularIncubatorValue::Bool(b) => __RegularValue::Bool(*b),
            __RegularIncubatorValue::Char(c) => __RegularValue::Char(*c),
            __RegularIncubatorValue::I8(n) => __RegularValue::I8(*n),
            __RegularIncubatorValue::I16(n) => __RegularValue::I16(*n),
            __RegularIncubatorValue::I32(n) => __RegularValue::I32(*n),
            __RegularIncubatorValue::I64(n) => __RegularValue::I64(*n),
            __RegularIncubatorValue::I128(n) => __RegularValue::I128(*n),
            __RegularIncubatorValue::ISize(n) => __RegularValue::ISize(*n),
            __RegularIncubatorValue::U8(n) => __RegularValue::U8(*n),
            __RegularIncubatorValue::U16(n) => __RegularValue::U16(*n),
            __RegularIncubatorValue::U32(n) => __RegularValue::U32(*n),
            __RegularIncubatorValue::U64(n) => __RegularValue::U64(*n),
            __RegularIncubatorValue::U128(n) => __RegularValue::U128(*n),
            __RegularIncubatorValue::USize(n) => __RegularValue::USize(*n),
            // Assuming that R8, R16, etc. are other variant types. Replace as necessary.
            __RegularIncubatorValue::R8(n) => __RegularValue::R8(*n),
            __RegularIncubatorValue::R16(n) => __RegularValue::R16(*n),
            __RegularIncubatorValue::R32(n) => __RegularValue::R32(*n),
            __RegularIncubatorValue::R64(n) => __RegularValue::R64(*n),
            __RegularIncubatorValue::R128(n) => __RegularValue::R128(*n),
            __RegularIncubatorValue::RSize(n) => __RegularValue::RSize(*n),
            __RegularIncubatorValue::F32(n) => __RegularValue::F32(*n),
            __RegularIncubatorValue::F64(n) => __RegularValue::F64(*n),
            __RegularIncubatorValue::StringLiteral(id) => __RegularValue::StringLiteral(*id),
            __RegularIncubatorValue::Intrinsic(box_value) => {
                __RegularValue::Intrinsic(box_value.incubate_box_dyn())
            }
            __RegularIncubatorValue::Box(box_value) => {
                __RegularValue::Box(box_value.incubate_box_dyn())
            }
            __RegularIncubatorValue::Leash(leash) => __RegularValue::Leash(*leash),
            __RegularIncubatorValue::SizedRef(_, box_value) => {
                __RegularValue::SizedRef(&**box_value as *const _)
            }
            __RegularIncubatorValue::SizedRefMut(_, box_value) => {
                __RegularValue::SizedRef(&mut **box_value as *mut _)
            }
            __RegularIncubatorValue::OptionBox(opt_box_value) => __RegularValue::OptionBox(
                opt_box_value
                    .as_mut()
                    .map(|box_value| box_value.incubate_box_dyn()),
            ),
            __RegularIncubatorValue::OptionLeash(leash) => __RegularValue::OptionLeash(*leash),
            __RegularIncubatorValue::OptionSizedRef(_, opt_box_value) => {
                __RegularValue::OptionSizedRef(
                    opt_box_value
                        .as_ref()
                        .map(|box_value| &**box_value as *const _),
                )
            }
            __RegularIncubatorValue::OptionSizedRefMut(_, opt_box_value) => {
                __RegularValue::OptionSizedMut(
                    opt_box_value
                        .as_mut()
                        .map(|box_value| &mut **box_value as *mut _),
                )
            }
        }
    }
}
