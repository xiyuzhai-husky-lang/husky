pub mod owned;

use self::owned::*;
use crate::{
    frozen::{ValueStand, ValueStands},
    r#static::{Static, StaticDyn},
    *,
};
use husky_decl_macro_utils::*;
use husky_task_interface::{val_control_flow::ValControlFlow, value::IsValue};
use serde::Serialize;
use serde_impl::json::SerdeJson;
use serde_impl::IsSerdeImpl;
use std::cmp::Ordering;

pub(crate) const REGULAR_VALUE_SIZE_OVER_I64: usize = 3;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
#[husky_task_interface::value]
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
    Owned(OwnedValue),
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
    EnumU8(u8),
}

// impl Drop for Value {
//     fn drop(&mut self) {
//         match self {
//             Value::Invalid => (),
//             Value::Moved => (),
//             Value::Unit(_) => (),
//             Value::Bool(_) => (),
//             Value::Char(_) => (),
//             Value::I8(_) => (),
//             Value::I16(_) => (),
//             Value::I32(_) => (),
//             Value::I64(_) => (),
//             Value::I128(_) => (),
//             Value::ISize(_) => (),
//             Value::U8(_) => (),
//             Value::U16(_) => (),
//             Value::U32(_) => (),
//             Value::U64(_) => (),
//             Value::U128(_) => (),
//             Value::USize(_) => (),
//             Value::R8(_) => (),
//             Value::R16(_) => (),
//             Value::R32(_) => (),
//             Value::R64(_) => (),
//             Value::R128(_) => (),
//             Value::RSize(_) => (),
//             Value::F32(_) => (),
//             Value::F64(_) => (),
//             Value::StringLiteral(_) => (),
//             Value::Box(boxed_value) => println!(
//                 "boxed value of type `{}` is being dropped",
//                 boxed_value.type_name_dyn()
//             ),
//             Value::Leash(_) => (),
//             Value::Ref(_) => (),
//             Value::Mut(_) => (),
//             Value::OptionBox(_) => (),
//             Value::OptionLeash(_) => (),
//             Value::OptionSizedRef(_) => (),
//             Value::OptionSizedMut(_) => (),
//             Value::EnumU8(_) => (),
//         }
//     }
// }

unsafe impl Send for Value {}

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

impl Value {
    pub fn from_owned<T>(t: T) -> Self
    where
        T: Static,
    {
        Value::Owned(OwnedValue::upcast_from_owned(t))
    }

    pub fn into_owned<T>(self) -> T
    where
        T: 'static,
    {
        match self {
            Value::Owned(slf) => slf.downcast_into_owned(),
            Value::Leash(slf) => *(slf.copy_dyn() as Box<dyn std::any::Any>)
                .downcast()
                .unwrap(),
            _ => unreachable!("self is {self:?}"),
        }
    }

    pub fn from_ref<'a, T>(t: &'a T) -> Self {
        todo!()
    }

    pub fn into_ref<'a, T>(self, value_stands: Option<&mut ValueStands>) -> &'a T
    where
        T: WeakStatic,
    {
        match self {
            Value::Invalid => todo!(),
            Value::Moved => todo!(),
            Value::Unit(_) => todo!(),
            Value::Bool(_) => todo!(),
            Value::Char(_) => todo!(),
            Value::I8(_) => todo!(),
            Value::I16(_) => todo!(),
            Value::I32(_) => todo!(),
            Value::I64(_) => todo!(),
            Value::I128(_) => todo!(),
            Value::ISize(_) => todo!(),
            Value::U8(_) => todo!(),
            Value::U16(_) => todo!(),
            Value::U32(_) => todo!(),
            Value::U64(_) => todo!(),
            Value::U128(_) => todo!(),
            Value::USize(_) => todo!(),
            Value::R8(_) => todo!(),
            Value::R16(_) => todo!(),
            Value::R32(_) => todo!(),
            Value::R64(_) => todo!(),
            Value::R128(_) => todo!(),
            Value::RSize(_) => todo!(),
            Value::F32(_) => todo!(),
            Value::F64(_) => todo!(),
            Value::StringLiteral(_) => todo!(),
            Value::Owned(slf) => {
                // todo: make the whole function unsafe
                let t: &T = slf.downcast_as_ref();
                let t = unsafe { std::mem::transmute(t) };
                value_stands
                    .unwrap()
                    .push(ValueStand::Box(slf.into_inner()));
                t
            }
            Value::Leash(slf) => {
                let slf: &<T as WeakStatic>::Static = ((slf as &dyn StaticDyn)
                    as &dyn std::any::Any)
                    .downcast_ref()
                    .expect("type id is correct");
                unsafe { std::mem::transmute(slf) }
            }
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::OptionBox(_) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
            Value::EnumU8(_) => todo!(),
        }
    }

    pub fn from_leash<T>(t: &'static T) -> Self
    where
        T: Static,
    {
        Value::Leash(t)
    }

    pub fn into_leash<T>(self) -> &'static T {
        todo!()
    }

    pub fn from_mut<'a, T>(t: &'a mut T) -> Self {
        todo!()
    }

    pub fn into_mut<'a, T>(self) -> &'a mut T {
        todo!()
    }

    pub fn from_option_ref<'a, T>(t: Option<&'a T>) -> Self {
        todo!()
    }

    pub fn into_option_ref<'a, T>(self) -> Option<&'a T> {
        todo!()
    }

    pub fn from_option_mut<'a, T>(t: Option<&'a mut T>) -> Self {
        todo!()
    }

    pub fn into_option_mut<'a, T>(self) -> Option<&'a mut T> {
        todo!()
    }

    pub fn from_enum_u8(index_raw: u8) -> Self {
        Value::EnumU8(index_raw)
    }
}

impl IsValue for Value {
    fn from_enum_u8(index_raw: u8) -> Self {
        Value::EnumU8(index_raw)
    }

    fn share(&'static self) -> Self {
        match *self {
            Value::Invalid => Value::Invalid,
            Value::Moved => Value::Moved,
            Value::Unit(slf) => Value::Unit(slf),
            Value::Bool(slf) => Value::Bool(slf),
            Value::Char(slf) => Value::Char(slf),
            Value::I8(slf) => Value::I8(slf),
            Value::I16(slf) => Value::I16(slf),
            Value::I32(slf) => Value::I32(slf),
            Value::I64(slf) => Value::I64(slf),
            Value::I128(slf) => Value::I128(slf),
            Value::ISize(slf) => Value::ISize(slf),
            Value::U8(slf) => Value::U8(slf),
            Value::U16(slf) => Value::U16(slf),
            Value::U32(slf) => Value::U32(slf),
            Value::U64(slf) => Value::U64(slf),
            Value::U128(slf) => Value::U128(slf),
            Value::USize(slf) => Value::USize(slf),
            Value::R8(slf) => Value::R8(slf),
            Value::R16(slf) => Value::R16(slf),
            Value::R32(slf) => Value::R32(slf),
            Value::R64(slf) => Value::R64(slf),
            Value::R128(slf) => Value::R128(slf),
            Value::RSize(slf) => Value::RSize(slf),
            Value::F32(slf) => Value::F32(slf),
            Value::F64(slf) => Value::F64(slf),
            Value::StringLiteral(slf) => Value::StringLiteral(slf),
            Value::Owned(ref slf) => Value::Leash(slf.as_ref()), // Clone the boxed value
            Value::Leash(slf) => Value::Leash(slf),
            Value::Ref(slf) => unreachable!(),
            Value::Mut(slf) => unreachable!(),
            Value::OptionBox(ref slf) => Value::OptionLeash(slf.as_ref().map(|v| &**v)), // Clone the boxed option
            Value::OptionLeash(slf) => Value::OptionLeash(slf),
            Value::OptionSizedRef(slf) => unreachable!("not expecting temporary ref for sharing"),
            Value::OptionSizedMut(slf) => unreachable!("not expecting temporary mut for sharing"),
            Value::EnumU8(slf) => Value::EnumU8(slf),
        }
    }

    fn to_bool(self) -> bool {
        match self {
            Value::Invalid => todo!(),
            Value::Moved => todo!(),
            Value::Unit(_) => todo!(),
            Value::Bool(b) => b,
            Value::Char(_) => todo!(),
            Value::I8(_) => todo!(),
            Value::I16(_) => todo!(),
            Value::I32(_) => todo!(),
            Value::I64(_) => todo!(),
            Value::I128(_) => todo!(),
            Value::ISize(i) => i != 0,
            Value::U8(_) => todo!(),
            Value::U16(_) => todo!(),
            Value::U32(_) => todo!(),
            Value::U64(_) => todo!(),
            Value::U128(_) => todo!(),
            Value::USize(_) => todo!(),
            Value::R8(_) => todo!(),
            Value::R16(_) => todo!(),
            Value::R32(_) => todo!(),
            Value::R64(_) => todo!(),
            Value::R128(_) => todo!(),
            Value::RSize(_) => todo!(),
            Value::F32(_) => todo!(),
            Value::F64(_) => todo!(),
            Value::StringLiteral(_) => todo!(),
            Value::Owned(_) => todo!(),
            Value::Leash(_) => todo!(),
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::OptionBox(_) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
            Value::EnumU8(_) => todo!(),
        }
    }

    fn r#move(&mut self) -> Self {
        std::mem::replace(self, Value::Moved)
    }

    fn is_none(self) -> bool {
        match self {
            Value::OptionBox(opt) => opt.is_none(),
            Value::OptionLeash(opt) => opt.is_none(),
            Value::OptionSizedRef(opt) => opt.is_none(),
            Value::OptionSizedMut(opt) => opt.is_none(),
            Value::Leash(opt) => opt.is_none_dyn(),
            _ => {
                use husky_print_utils::p;
                unreachable!()
            }
        }
    }

    fn is_some(self) -> bool {
        match self {
            Value::OptionBox(opt) => opt.is_some(),
            Value::OptionLeash(opt) => opt.is_some(),
            Value::OptionSizedRef(opt) => opt.is_some(),
            Value::OptionSizedMut(opt) => opt.is_some(),
            Value::Leash(opt) => opt.is_some_dyn(),
            _ => unreachable!(),
        }
    }

    fn to_usize(self) -> usize {
        match self {
            Value::Invalid => todo!(),
            Value::Moved => todo!(),
            Value::Unit(_) => todo!(),
            Value::Bool(_) => todo!(),
            Value::Char(_) => todo!(),
            Value::I8(_) => todo!(),
            Value::I16(_) => todo!(),
            Value::I32(slf) => slf as usize,
            Value::I64(_) => todo!(),
            Value::I128(_) => todo!(),
            Value::ISize(_) => todo!(),
            Value::U8(_) => todo!(),
            Value::U16(_) => todo!(),
            Value::U32(_) => todo!(),
            Value::U64(_) => todo!(),
            Value::U128(_) => todo!(),
            Value::USize(slf) => slf,
            Value::R8(_) => todo!(),
            Value::R16(_) => todo!(),
            Value::R32(_) => todo!(),
            Value::R64(_) => todo!(),
            Value::R128(_) => todo!(),
            Value::RSize(_) => todo!(),
            Value::F32(_) => todo!(),
            Value::F64(_) => todo!(),
            Value::StringLiteral(_) => todo!(),
            Value::Owned(_) => todo!(),
            Value::Leash(_) => todo!(),
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::OptionBox(_) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
            Value::EnumU8(_) => todo!(),
        }
    }

    fn index(self, index: usize) -> Self {
        match self {
            Value::Invalid => todo!(),
            Value::Moved => todo!(),
            Value::Unit(_) => todo!(),
            Value::Bool(_) => todo!(),
            Value::Char(_) => todo!(),
            Value::I8(_) => todo!(),
            Value::I16(_) => todo!(),
            Value::I32(_) => todo!(),
            Value::I64(_) => todo!(),
            Value::I128(_) => todo!(),
            Value::ISize(_) => todo!(),
            Value::U8(_) => todo!(),
            Value::U16(_) => todo!(),
            Value::U32(_) => todo!(),
            Value::U64(_) => todo!(),
            Value::U128(_) => todo!(),
            Value::USize(_) => todo!(),
            Value::R8(_) => todo!(),
            Value::R16(_) => todo!(),
            Value::R32(_) => todo!(),
            Value::R64(_) => todo!(),
            Value::R128(_) => todo!(),
            Value::RSize(_) => todo!(),
            Value::F32(_) => todo!(),
            Value::F64(_) => todo!(),
            Value::StringLiteral(_) => todo!(),
            Value::Owned(_) => todo!(),
            Value::Leash(slf) => Value::Leash(slf.index_ref_dyn(index)),
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::OptionBox(_) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
            Value::EnumU8(_) => todo!(),
        }
    }

    fn serialize_to_value(&self) -> <Self::SerdeImpl as IsSerdeImpl>::Value {
        match self {
            Value::Invalid => unreachable!(),
            Value::Moved => unreachable!(),
            Value::Unit(_) => todo!(),
            Value::Bool(b) => <Self::SerdeImpl as IsSerdeImpl>::to_value(b).unwrap(),
            Value::Char(_) => todo!(),
            Value::I8(_) => todo!(),
            Value::I16(_) => todo!(),
            Value::I32(_) => todo!(),
            Value::I64(_) => todo!(),
            Value::I128(_) => todo!(),
            Value::ISize(_) => todo!(),
            Value::U8(_) => todo!(),
            Value::U16(_) => todo!(),
            Value::U32(_) => todo!(),
            Value::U64(_) => todo!(),
            Value::U128(_) => todo!(),
            Value::USize(_) => todo!(),
            Value::R8(_) => todo!(),
            Value::R16(_) => todo!(),
            Value::R32(_) => todo!(),
            Value::R64(_) => todo!(),
            Value::R128(_) => todo!(),
            Value::RSize(_) => todo!(),
            Value::F32(_) => todo!(),
            Value::F64(_) => todo!(),
            Value::StringLiteral(_) => todo!(),
            Value::Owned(_) => todo!(),
            Value::Leash(_) => todo!(),
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::OptionBox(_) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
            Value::EnumU8(_) => todo!(),
        }
    }

    type SerdeImpl = SerdeJson;
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unit(l0), Self::Unit(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::I8(l0), Self::I8(r0)) => l0 == r0,
            (Self::I16(l0), Self::I16(r0)) => l0 == r0,
            (Self::I32(l0), Self::I32(r0)) => l0 == r0,
            (Self::I64(l0), Self::I64(r0)) => l0 == r0,
            (Self::I128(l0), Self::I128(r0)) => l0 == r0,
            (Self::ISize(l0), Self::ISize(r0)) => l0 == r0,
            (Self::U8(l0), Self::U8(r0)) => l0 == r0,
            (Self::U16(l0), Self::U16(r0)) => l0 == r0,
            (Self::U32(l0), Self::U32(r0)) => l0 == r0,
            (Self::U64(l0), Self::U64(r0)) => l0 == r0,
            (Self::U128(l0), Self::U128(r0)) => l0 == r0,
            (Self::USize(l0), Self::USize(r0)) => l0 == r0,
            (Self::R8(l0), Self::R8(r0)) => l0 == r0,
            (Self::R16(l0), Self::R16(r0)) => l0 == r0,
            (Self::R32(l0), Self::R32(r0)) => l0 == r0,
            (Self::R64(l0), Self::R64(r0)) => l0 == r0,
            (Self::R128(l0), Self::R128(r0)) => l0 == r0,
            (Self::RSize(l0), Self::RSize(r0)) => l0 == r0,
            (Self::F32(l0), Self::F32(r0)) => l0 == r0,
            (Self::F64(l0), Self::F64(r0)) => l0 == r0,
            (Self::StringLiteral(l0), Self::StringLiteral(r0)) => todo!(),
            (Self::Owned(l0), Self::Owned(r0)) => todo!(),
            (Self::Leash(l0), Self::Leash(r0)) => todo!(),
            (Self::Ref(l0), Self::Ref(r0)) => todo!(),
            (Self::Mut(l0), Self::Mut(r0)) => todo!(),
            (Self::OptionBox(l0), Self::OptionBox(r0)) => todo!(),
            (Self::OptionLeash(l0), Self::OptionLeash(r0)) => todo!(),
            (Self::OptionSizedRef(l0), Self::OptionSizedRef(r0)) => todo!(),
            (Self::OptionSizedMut(l0), Self::OptionSizedMut(r0)) => todo!(),
            (Self::EnumU8(l0), Self::EnumU8(r0)) => l0 == r0,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Value::*;
        match (self, other) {
            (Unit(_), Unit(_)) => Some(Ordering::Equal),
            (Bool(b1), Bool(b2)) => b1.partial_cmp(b2),
            (Char(c1), Char(c2)) => c1.partial_cmp(c2),
            (I8(i1), I8(i2)) => i1.partial_cmp(i2),
            (I16(i1), I16(i2)) => i1.partial_cmp(i2),
            (I32(i1), I32(i2)) => i1.partial_cmp(i2),
            (I64(i1), I64(i2)) => i1.partial_cmp(i2),
            (I128(i1), I128(i2)) => i1.partial_cmp(i2),
            (ISize(i1), ISize(i2)) => i1.partial_cmp(i2),
            (U8(u1), U8(u2)) => u1.partial_cmp(u2),
            (U16(u1), U16(u2)) => u1.partial_cmp(u2),
            (U32(u1), U32(u2)) => u1.partial_cmp(u2),
            (U64(u1), U64(u2)) => u1.partial_cmp(u2),
            (U128(u1), U128(u2)) => u1.partial_cmp(u2),
            (USize(u1), USize(u2)) => u1.partial_cmp(u2),
            (F32(f1), F32(f2)) => f1.partial_cmp(f2),
            (F64(f1), F64(f2)) => f1.partial_cmp(f2),
            (StringLiteral(l0), StringLiteral(r0)) => todo!(),
            (Value::Owned(l0), Value::Owned(r0)) => todo!(),
            (Leash(l0), Leash(r0)) => todo!(),
            (Ref(l0), Ref(r0)) => todo!(),
            (Mut(l0), Mut(r0)) => todo!(),
            (OptionBox(l0), OptionBox(r0)) => todo!(),
            (OptionLeash(l0), OptionLeash(r0)) => todo!(),
            (OptionSizedRef(l0), OptionSizedRef(r0)) => todo!(),
            (OptionSizedMut(l0), OptionSizedMut(r0)) => todo!(),
            (EnumU8(l0), EnumU8(r0)) => todo!(),
            _ => unreachable!(),
        }
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
            _ => {
                println!("self = {:?}", self);
                unreachable!()
            }
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
