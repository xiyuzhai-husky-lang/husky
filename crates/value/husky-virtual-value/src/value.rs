mod r#enum;
mod list;
mod r#struct;

use husky_value_interface::IsValue;
use husky_value_protocol::presentation::{
    synchrotron::ValuePresentationSynchrotron, EnumUnitValuePresenter, ValuePresentation,
    ValuePresenterCache,
};
use husky_visual_protocol::synchrotron::VisualSynchrotron;
use husky_visual_protocol::visual::Visual;
use std::{cmp::Ordering, sync::Arc};

use crate::exception::{ExceptedValue, Exception};

#[derive(Debug)]
pub enum Value {
    Uninit,
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
    EnumUnit {
        index: u8,
        presenter: EnumUnitValuePresenter,
    },
    Vec(Vec<Value>),
    StringLiteral(Arc<str>),
}

impl From<std::convert::Infallible> for Value {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

impl IsValue for Value {
    type Exception = Exception;

    fn new_uninit() -> Self {
        Value::Uninit
    }

    fn from_enum_index(index: usize, presenter: EnumUnitValuePresenter) -> Self {
        todo!()
    }

    fn share(&'static self) -> Self {
        todo!()
    }

    fn to_bool(self) -> bool {
        todo!()
    }

    fn to_usize(self) -> usize {
        todo!()
    }

    fn r#move(&mut self) -> Self {
        todo!()
    }

    fn is_none(self) -> bool {
        todo!()
    }

    fn is_some(self) -> bool {
        todo!()
    }

    fn index(self, index: usize) -> Self {
        todo!()
    }

    fn present(
        &self,
        value_presenter_cache: &mut ValuePresenterCache,
        value_presentation_synchrotron: &mut ValuePresentationSynchrotron,
    ) -> ValuePresentation {
        todo!()
    }

    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }

    fn from_str_literal(str_value: std::sync::Arc<str>) -> Self {
        Value::StringLiteral(str_value)
    }

    fn unwrap(self) -> ExceptedValue {
        match self {
            Value::Uninit => todo!(),
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
            Value::EnumUnit { index, presenter } => todo!(),
            Value::Vec(_) => todo!(),
            Value::StringLiteral(_) => todo!(),
        }
    }
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
            (Self::EnumUnit { index: l0, .. }, Self::EnumUnit { index: r0, .. }) => l0 == r0,
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
            (EnumUnit { index: l0, .. }, EnumUnit { index: r0, .. }) => todo!(),
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

impl std::ops::BitAnd for Value {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::R8(a), Value::R8(b)) => Value::R8(a & b),
            (Value::R16(a), Value::R16(b)) => Value::R16(a & b),
            (Value::R32(a), Value::R32(b)) => Value::R32(a & b),
            (Value::R64(a), Value::R64(b)) => Value::R64(a & b),
            (Value::R128(a), Value::R128(b)) => Value::R128(a & b),
            (Value::RSize(a), Value::RSize(b)) => Value::RSize(a & b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::BitAndAssign for Value {
    fn bitand_assign(&mut self, rhs: Self) {
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
        match self {
            Value::Uninit => todo!(),
            Value::Invalid => todo!(),
            Value::Moved => todo!(),
            Value::Unit(_) => todo!(),
            Value::Bool(_) => todo!(),
            Value::Char(_) => todo!(),
            Value::I8(_) => todo!(),
            Value::I16(_) => todo!(),
            Value::I32(i) => (-i).into(),
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
            Value::EnumUnit { index, presenter } => todo!(),
            Value::Vec(_) => todo!(),
            Value::StringLiteral(_) => todo!(),
        }
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

impl From<char> for Value {
    fn from(value: char) -> Self {
        Value::Char(value)
    }
}

impl Into<char> for Value {
    fn into(self) -> char {
        match self {
            Value::Char(value) => value,
            _ => unreachable!(),
        }
    }
}
