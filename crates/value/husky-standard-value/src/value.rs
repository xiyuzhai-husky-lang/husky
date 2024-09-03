pub mod owned;

use self::owned::*;
use crate::exception::Excepted;
use crate::{
    slush::{SlushValue, SlushValues},
    thawed::{Thawed, ThawedDyn},
    *,
};
use frozen::FrozenValue;
use husky_decl_macro_utils::*;
#[cfg(feature = "constant")]
use husky_term_prelude::literal::StringLiteralTokenData;
use husky_value_interface::ki_control_flow::KiControlFlow;
use husky_value_interface::IsValue;
use husky_value_macros::value_ty;
use husky_value_protocol::presentation::{
    synchrotron::ValuePresentationSynchrotron, EnumUnitValuePresenter, ValuePresentation,
    ValuePresenterCache,
};
use husky_visual_protocol::{
    synchrotron::VisualSynchrotron,
    visual::{primitive::PrimitiveVisual, Visual},
};
use std::cmp::Ordering;

pub(crate) const REGULAR_VALUE_SIZE_OVER_I64: usize = 4;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
#[value_ty]
#[derive(Debug)]
#[repr(u8)]
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
    StringLiteral(StringLiteralId),
    /// `Box<T>`
    Owned(OwnedValue),
    // ad hoc
    /// `~T`
    Leash(&'static dyn ThawedDyn),
    /// `&T` for T Sized
    Ref(*const dyn ThawedDyn),
    /// `&mut T` for T Sized
    Mut(*mut dyn ThawedDyn),
    OptionBox(Option<Box<dyn ThawedDyn>>),
    OptionLeash(Option<&'static dyn ThawedDyn>),
    OptionSizedRef(Option<*const dyn ThawedDyn>),
    OptionSizedMut(Option<*mut dyn ThawedDyn>),
    EnumUnit {
        index: usize,
        presenter: EnumUnitValuePresenter,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StringLiteralId(NonZeroU32);

#[cfg(feature = "constant")]
impl From<StringLiteralTokenData> for StringLiteralId {
    fn from(lit: StringLiteralTokenData) -> Self {
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

impl From<std::convert::Infallible> for Value {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

impl Value {
    pub fn from_owned<T>(t: T) -> Self
    where
        T: Thawed,
    {
        Value::Owned(OwnedValue::upcast_from_owned(t))
    }

    pub fn into_owned<T>(self) -> T
    where
        T: 'static,
    {
        match self {
            Value::Owned(slf) => slf.downcast_into_owned(),
            // ad hoc
            Value::Leash(slf) => slf.try_copy_dyn().unwrap().into_owned(),
            // *(slf.try_copy_dyn() as Box<dyn std::any::Any>)
            //     .downcast()
            //     .unwrap(),
            _ => unreachable!("self is {self:?}"),
        }
    }

    pub fn from_ref<'a, T>(t: &'a T) -> Self {
        todo!()
    }

    #[deprecated(note = "Value is going to be Send Sync, eternal")]
    pub fn into_ref<'a, T>(self, slush_values: Option<&mut SlushValues>) -> &'a T
    where
        T: Boiled,
    {
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
            Value::StringLiteral(_) => todo!(),
            Value::Owned(slf) => {
                // todo: make the whole function unsafe
                let t: &T = slf.downcast_as_ref();
                let t = unsafe { std::mem::transmute(t) };
                slush_values
                    .unwrap()
                    .push(SlushValue::Box(slf.into_inner()));
                t
            }
            Value::Leash(slf) => {
                let slf: &<T as Boiled>::Thawed = ((slf as &dyn ThawedDyn) as &dyn std::any::Any)
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
            Value::EnumUnit { .. } => todo!(),
        }
    }

    pub fn from_leash<T>(t: &'static T) -> Self
    where
        T: Thawed,
    {
        Value::Leash(t)
    }

    pub fn into_leash<T>(self) -> &'static T {
        match self {
            // ad hoc, we maybe encounter &'static Leash<T> here, so can't always just unwrap it
            Value::Leash(slf) => (slf as &dyn std::any::Any).downcast_ref().unwrap(),
            _ => unreachable!(),
        }
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

    pub fn from_enum_index(index: usize, presenter: EnumUnitValuePresenter) -> Self {
        Value::EnumUnit { index, presenter }
    }
}

impl IsValue for Value {
    type Exception = Exception;

    fn new_uninit() -> Self {
        Value::Uninit
    }

    fn from_enum_index(index: usize, presenter: EnumUnitValuePresenter) -> Self {
        Value::EnumUnit { index, presenter }
    }

    fn share(&'static self) -> Self {
        match *self {
            Value::Uninit => Value::Uninit,
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
            Value::EnumUnit { index, presenter } => Value::EnumUnit { index, presenter },
        }
    }

    fn to_bool(self) -> bool {
        match self {
            Value::Bool(val) => val,
            Value::Char(val) => val != Default::default(),
            Value::I8(val) => val != 0,
            Value::I16(val) => val != 0,
            Value::I32(val) => val != 0,
            Value::I64(val) => val != 0,
            Value::I128(val) => val != 0,
            Value::ISize(val) => val != 0,
            Value::U8(val) => val != 0,
            Value::U16(val) => val != 0,
            Value::U32(val) => val != 0,
            Value::U64(val) => val != 0,
            Value::U128(val) => val != 0,
            Value::USize(val) => val != 0,
            Value::R8(val) => val != 0,
            Value::R16(val) => val != 0,
            Value::R32(val) => val != 0,
            Value::R64(val) => val != 0,
            Value::R128(val) => val != 0,
            Value::RSize(val) => val != 0,
            _ => unreachable!(),
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
            Value::I8(slf) => slf as usize,
            Value::I16(slf) => slf as usize,
            Value::I32(slf) => slf as usize,
            Value::I64(slf) => slf as usize,
            Value::I128(slf) => slf as usize,
            Value::ISize(slf) => slf as usize,
            Value::U8(slf) => slf as usize,
            Value::U16(slf) => slf as usize,
            Value::U32(slf) => slf as usize,
            Value::U64(slf) => slf as usize,
            Value::U128(slf) => slf as usize,
            Value::USize(slf) => slf,
            Value::R8(slf) => slf as usize,
            Value::R16(slf) => slf as usize,
            Value::R32(slf) => slf as usize,
            Value::R64(slf) => slf as usize,
            Value::R128(slf) => slf as usize,
            Value::RSize(slf) => slf as usize,
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::EnumUnit { .. } => todo!(),
            _ => unreachable!(),
        }
    }

    fn index(self, index: usize) -> Excepted<Self> {
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
            Value::StringLiteral(_) => todo!(),
            Value::Owned(slf) => slf.index_owned_dyn(index),
            Value::Leash(slf) => slf.index_leash_dyn(index),
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::OptionBox(_) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
            Value::EnumUnit { .. } => todo!(),
        }
    }

    fn present(
        &self,
        cache: &mut ValuePresenterCache,
        value_presentation_synchrotron: &mut ValuePresentationSynchrotron,
    ) -> ValuePresentation {
        match *self {
            Value::Uninit => todo!(),
            Value::Invalid => unreachable!(),
            Value::Moved => unreachable!(),
            Value::Unit(_) => ValuePresentation::Unit(()),
            Value::Bool(b) => ValuePresentation::Bool(b),
            Value::Char(c) => ValuePresentation::Char(c),
            Value::I8(i) => ValuePresentation::I8(i),
            Value::I16(i) => ValuePresentation::I16(i),
            Value::I32(i) => ValuePresentation::I32(i),
            Value::I64(i) => ValuePresentation::I64(i),
            Value::I128(i) => ValuePresentation::I128(i),
            Value::ISize(i) => ValuePresentation::ISize(i),
            Value::U8(u) => ValuePresentation::U8(u),
            Value::U16(u) => ValuePresentation::U16(u),
            Value::U32(u) => ValuePresentation::U32(u),
            Value::U64(u) => ValuePresentation::U64(u),
            Value::U128(u) => ValuePresentation::U128(u),
            Value::USize(u) => ValuePresentation::USize(u),
            Value::R8(r) => ValuePresentation::R8(r),
            Value::R16(r) => ValuePresentation::R16(r),
            Value::R32(r) => ValuePresentation::R32(r),
            Value::R64(r) => ValuePresentation::R64(r),
            Value::R128(r) => ValuePresentation::R128(r),
            Value::RSize(r) => ValuePresentation::RSize(r),
            Value::F32(f) => ValuePresentation::F32(f.into()),
            Value::F64(f) => ValuePresentation::F64(f.into()),
            Value::StringLiteral(_) => todo!(),
            Value::Owned(ref value) => value.present_dyn(),
            Value::Leash(value) => value.present_dyn(),
            Value::Ref(value) => unsafe { (*value).present_dyn() },
            Value::Mut(value) => unsafe { (*value).present_dyn() },
            Value::OptionBox(ref value) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
            Value::EnumUnit { index, presenter } => {
                presenter(index, cache, value_presentation_synchrotron)
            }
        }
    }

    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        use husky_visual_protocol::visualize::Visualize;
        match *self {
            Value::Uninit => todo!(),
            Value::Invalid => unreachable!(),
            Value::Moved => unreachable!(),
            Value::Unit(_) => Visual::Void,
            Value::Bool(_) => todo!(),
            Value::Char(_) => todo!(),
            Value::I8(value) => PrimitiveVisual::I8(value).into(),
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
            Value::F32(f) => f.visualize(visual_synchrotron),
            Value::F64(_) => todo!(),
            Value::StringLiteral(_) => todo!(),
            Value::Owned(ref value) => value.visualize_or_void_dyn(visual_synchrotron),
            Value::Leash(value) => value.visualize_or_void_dyn(visual_synchrotron),
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::OptionBox(_) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
            Value::EnumUnit { .. } => Visual::Void,
        }
    }

    fn from_str_literal(str_value: Arc<str>) -> Self {
        todo!()
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
            Value::StringLiteral(_) => todo!(),
            Value::Owned(_) => todo!(),
            Value::Leash(slf) => slf.unwrap_leash_dyn(),
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::OptionBox(_) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
            Value::EnumUnit { index, presenter } => todo!(),
        }
    }

    type FrozenValue = FrozenValue;

    fn freeze(&self) -> Self::FrozenValue {
        match *self {
            Value::Uninit => todo!(),
            Value::Moved => FrozenValue::Moved,
            Value::Invalid => FrozenValue::Invalid,
            Value::Unit(_) => FrozenValue::Unit(()),
            Value::Bool(val) => FrozenValue::Bool(val),
            Value::Char(val) => FrozenValue::Char(val),
            Value::I8(val) => FrozenValue::I8(val),
            Value::I16(val) => FrozenValue::I16(val),
            Value::I32(val) => FrozenValue::I32(val),
            Value::I64(val) => FrozenValue::I64(val),
            Value::I128(val) => FrozenValue::I128(val),
            Value::ISize(val) => FrozenValue::ISize(val),
            Value::U8(val) => FrozenValue::U8(val),
            Value::U16(val) => FrozenValue::U16(val),
            Value::U32(val) => FrozenValue::U32(val),
            Value::U64(val) => FrozenValue::U64(val),
            Value::U128(val) => FrozenValue::U128(val),
            Value::USize(val) => FrozenValue::USize(val),
            Value::R8(val) => FrozenValue::R8(val),
            Value::R16(val) => FrozenValue::R16(val),
            Value::R32(val) => FrozenValue::R32(val),
            Value::R64(val) => FrozenValue::R64(val),
            Value::R128(val) => FrozenValue::R128(val),
            Value::RSize(val) => FrozenValue::RSize(val),
            Value::F32(val) => FrozenValue::F32(val),
            Value::F64(val) => FrozenValue::F64(val),
            Value::StringLiteral(id) => FrozenValue::StringLiteral(id),
            Value::EnumUnit { index, presenter } => FrozenValue::EnumUsize { index, presenter },
            Value::Owned(ref slf) => todo!(),
            Value::Leash(_) => todo!(),
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::OptionBox(_) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
        }
    }

    type SlushValue = SlushValue;
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
            (StringLiteral(l0), StringLiteral(r0)) => todo!(),
            (Value::Owned(l0), Value::Owned(r0)) => todo!(),
            (Leash(l0), Leash(r0)) => todo!(),
            (Ref(l0), Ref(r0)) => todo!(),
            (Mut(l0), Mut(r0)) => todo!(),
            (OptionBox(l0), OptionBox(r0)) => todo!(),
            (OptionLeash(l0), OptionLeash(r0)) => todo!(),
            (OptionSizedRef(l0), OptionSizedRef(r0)) => todo!(),
            (OptionSizedMut(l0), OptionSizedMut(r0)) => todo!(),
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
            Value::I8(i) => Value::I8(-i),
            Value::I16(i) => Value::I16(-i),
            Value::I32(i) => Value::I32(-i),
            Value::I64(i) => Value::I64(-i),
            Value::I128(i) => Value::I128(-i),
            Value::ISize(i) => Value::ISize(-i),
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
            Value::F32(f) => Value::F32(-f),
            Value::F64(f) => Value::F64(-f),
            Value::StringLiteral(_) => todo!(),
            Value::Owned(_) => todo!(),
            Value::Leash(_) => todo!(),
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::OptionBox(_) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
            Value::EnumUnit { index, presenter } => todo!(),
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
