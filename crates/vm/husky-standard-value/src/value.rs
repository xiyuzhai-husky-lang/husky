use crate::{r#static::StaticDyn, *};
use husky_task_prelude::{all_ritchies, value::IsValue};

pub(crate) const REGULAR_VALUE_SIZE_OVER_I64: usize = 3;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
#[husky_task_prelude::value]
#[derive(Debug)]
#[repr(u8)]
pub enum Value {
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
    /// `Box<T>`
    Box(Box<dyn StaticDyn>),
    // ad hoc
    /// `~T`
    Leash(&'static dyn StaticDyn),
    /// `&T` for T Sized
    Ref(*const dyn StaticDyn),
    /// `&mut T` for T Sized
    Mut(*mut dyn StaticDyn),
    OptionBox(Option<Box<dyn StaticDyn>>),
    OptionLeash(Option<&'static dyn StaticDyn>),
    OptionSizedRef(Option<*const dyn StaticDyn>),
    OptionSizedMut(Option<*mut dyn StaticDyn>),
    /// T where T is not in above cases
    Intrinsic(Box<dyn StaticDyn>),
    EnumU8(u8),
}

unsafe impl Send for Value {}

impl Value {
    pub fn share(&self) -> Value {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StringLiteralId(NonZeroU32);

#[cfg(feature = "constant")]
impl From<StringLiteralData> for StringLiteralId {
    fn from(lit: StringLiteralData) -> Self {
        unsafe { std::mem::transmute(lit) }
    }
}

#[test]
fn regular_value_size_works() {
    assert_eq!(
        std::mem::size_of::<Value>(),
        std::mem::size_of::<[u64; REGULAR_VALUE_SIZE_OVER_I64]>()
    )
}

impl IsValue for Value {
    fn from_ref<'a, T>(t: &'a T) -> Self {
        todo!()
    }

    fn into_ref<'a, T>(t: &'a mut T) -> Self {
        todo!()
    }

    fn from_leash<T>(t: &'static T) -> Self {
        todo!()
    }

    fn into_leash<T>(t: &'static mut T) -> Self {
        todo!()
    }

    fn from_mut<'a, T>(self) -> &'a mut T {
        todo!()
    }

    fn into_mut<'a, T>(self) -> &'a mut T {
        todo!()
    }

    fn from_option_ref<'a, T>(t: Option<&'a T>) -> Self {
        todo!()
    }

    fn into_option_ref<'a, T>(t: Option<&'a mut T>) -> Self {
        todo!()
    }

    fn from_enum_u8(index_raw: u8) -> Self {
        Value::EnumU8(index_raw)
    }
}

impl std::ops::Add<Value> for Value {
    type Output = Self;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a + b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a + b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a + b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a + b),
            (Value::I128(a), Value::I128(b)) => Value::I128(a + b),
            (Value::ISize(a), Value::ISize(b)) => Value::ISize(a + b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a + b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a + b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a + b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a + b),
            (Value::U128(a), Value::U128(b)) => Value::U128(a + b),
            (Value::USize(a), Value::USize(b)) => Value::USize(a + b),
            (Value::R8(a), Value::R8(b)) => Value::R8(a + b),
            (Value::R16(a), Value::R16(b)) => Value::R16(a + b),
            (Value::R32(a), Value::R32(b)) => Value::R32(a + b),
            (Value::R64(a), Value::R64(b)) => Value::R64(a + b),
            (Value::R128(a), Value::R128(b)) => Value::R128(a + b),
            (Value::RSize(a), Value::RSize(b)) => Value::RSize(a + b),
            (Value::F32(a), Value::F32(b)) => Value::F32(a + b),
            (Value::F64(a), Value::F64(b)) => Value::F64(a + b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::AddAssign<Value> for Value {
    fn add_assign(&mut self, rhs: Value) {
        todo!()
    }
}
impl std::ops::BitOr for Value {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::R8(a), Value::R8(b)) => Value::R8(a | b),
            (Value::R16(a), Value::R16(b)) => Value::R16(a | b),
            (Value::R32(a), Value::R32(b)) => Value::R32(a | b),
            (Value::R64(a), Value::R64(b)) => Value::R64(a | b),
            (Value::R128(a), Value::R128(b)) => Value::R128(a | b),
            (Value::RSize(a), Value::RSize(b)) => Value::RSize(a | b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::BitOrAssign for Value {
    fn bitor_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::BitXor for Value {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::BitXorAssign for Value {
    fn bitxor_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a / b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a / b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a / b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a / b),
            (Value::I128(a), Value::I128(b)) => Value::I128(a / b),
            (Value::ISize(a), Value::ISize(b)) => Value::ISize(a / b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a / b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a / b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a / b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a / b),
            (Value::U128(a), Value::U128(b)) => Value::U128(a / b),
            (Value::USize(a), Value::USize(b)) => Value::USize(a / b),
            (Value::R8(a), Value::R8(b)) => Value::R8(a / b),
            (Value::R16(a), Value::R16(b)) => Value::R16(a / b),
            (Value::R32(a), Value::R32(b)) => Value::R32(a / b),
            (Value::R64(a), Value::R64(b)) => Value::R64(a / b),
            (Value::R128(a), Value::R128(b)) => Value::R128(a / b),
            (Value::RSize(a), Value::RSize(b)) => Value::RSize(a / b),
            (Value::F32(a), Value::F32(b)) => Value::F32(a / b),
            (Value::F64(a), Value::F64(b)) => Value::F64(a / b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a * b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a * b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a * b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a * b),
            (Value::I128(a), Value::I128(b)) => Value::I128(a * b),
            (Value::ISize(a), Value::ISize(b)) => Value::ISize(a * b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a * b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a * b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a * b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a * b),
            (Value::U128(a), Value::U128(b)) => Value::U128(a * b),
            (Value::USize(a), Value::USize(b)) => Value::USize(a * b),
            (Value::R8(a), Value::R8(b)) => Value::R8(a * b),
            (Value::R16(a), Value::R16(b)) => Value::R16(a * b),
            (Value::R32(a), Value::R32(b)) => Value::R32(a * b),
            (Value::R64(a), Value::R64(b)) => Value::R64(a * b),
            (Value::R128(a), Value::R128(b)) => Value::R128(a * b),
            (Value::RSize(a), Value::RSize(b)) => Value::RSize(a * b),
            (Value::F32(a), Value::F32(b)) => Value::F32(a * b),
            (Value::F64(a), Value::F64(b)) => Value::F64(a * b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::MulAssign for Value {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Shl for Value {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::ShlAssign for Value {
    fn shl_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::Shr for Value {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::ShrAssign for Value {
    fn shr_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a - b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a - b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a - b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a - b),
            (Value::I128(a), Value::I128(b)) => Value::I128(a - b),
            (Value::ISize(a), Value::ISize(b)) => Value::ISize(a - b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a - b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a - b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a - b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a - b),
            (Value::U128(a), Value::U128(b)) => Value::U128(a - b),
            (Value::USize(a), Value::USize(b)) => Value::USize(a - b),
            (Value::R8(a), Value::R8(b)) => Value::R8(a - b),
            (Value::R16(a), Value::R16(b)) => Value::R16(a - b),
            (Value::R32(a), Value::R32(b)) => Value::R32(a - b),
            (Value::R64(a), Value::R64(b)) => Value::R64(a - b),
            (Value::R128(a), Value::R128(b)) => Value::R128(a - b),
            (Value::RSize(a), Value::RSize(b)) => Value::RSize(a - b),
            (Value::F32(a), Value::F32(b)) => Value::F32(a - b),
            (Value::F64(a), Value::F64(b)) => Value::F64(a - b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl From<()> for Value {
    fn from(value: ()) -> Self {
        Value::Unit(())
    }
}

impl Into<()> for Value {
    fn into(self) -> () {
        match self {
            Value::Unit(()) => (),
            _ => unreachable!(),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        match self {
            Value::Bool(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::U8(value)
    }
}

impl Into<u8> for Value {
    fn into(self) -> u8 {
        match self {
            Value::U8(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::U16(value)
    }
}

impl Into<u16> for Value {
    fn into(self) -> u16 {
        match self {
            Value::U16(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::U32(value)
    }
}

impl Into<u32> for Value {
    fn into(self) -> u32 {
        match self {
            Value::U32(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::U64(value)
    }
}

impl Into<u64> for Value {
    fn into(self) -> u64 {
        match self {
            Value::U64(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Value::U128(value)
    }
}

impl Into<u128> for Value {
    fn into(self) -> u128 {
        match self {
            Value::U128(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::USize(value)
    }
}

impl Into<usize> for Value {
    fn into(self) -> usize {
        match self {
            Value::USize(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::I8(value)
    }
}

impl Into<i8> for Value {
    fn into(self) -> i8 {
        match self {
            Value::I8(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::I16(value)
    }
}

impl Into<i16> for Value {
    fn into(self) -> i16 {
        match self {
            Value::I16(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl Into<i32> for Value {
    fn into(self) -> i32 {
        match self {
            Value::I32(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl Into<i64> for Value {
    fn into(self) -> i64 {
        match self {
            Value::I64(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::I128(value)
    }
}

impl Into<i128> for Value {
    fn into(self) -> i128 {
        match self {
            Value::I128(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::ISize(value)
    }
}

impl Into<isize> for Value {
    fn into(self) -> isize {
        match self {
            Value::ISize(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::F32(value)
    }
}

impl Into<f32> for Value {
    fn into(self) -> f32 {
        match self {
            Value::F32(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        match self {
            Value::F64(value) => value,
            _ => unreachable!(),
        }
    }
}
