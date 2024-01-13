pub mod synchrotron;

// todo: move these to husky-value-protocol
use self::synchrotron::ValuePresentationSynchrotron;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValuePresentation {
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
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
    Enum,
    Struct,
    AdHoc(String),
}

pub type EnumU8ValuePresenter =
    fn(u8, &mut ValuePresenterCache, &mut ValuePresentationSynchrotron) -> ValuePresentation;

#[derive(Default)]
pub struct ValuePresenterCache {}
