pub mod r#mut;
pub mod option;
pub mod owned;
pub mod primitive;
pub mod r#ref;
mod ritchie;
mod str;
mod tuple;
pub mod vec;

use super::*;
use crate::{
    frozen::{r#mut::FrozenMut, Frozen, FrozenDyn},
    slush::{SlushValue, SlushValues},
    *,
};
use husky_decl_macro_utils::{
    for_all_non_unit_tuple_tys, for_all_primitive_tys, for_all_ritchie_tys,
};
use husky_value::{vm_control_flow::VmControlFlow, IsThawedValue};
use husky_value_macros::thawed_value_ty;
use husky_value_protocol::presentation::{EnumUnitValuePresenter, ValuePresentation};
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};
use owned::OwnedThawedValue;

/// Slush is the static version of a type
pub trait Thawed: Sized + std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type Frozen: Frozen<Thawed = Self>;
    unsafe fn freeze(&self) -> Self::Frozen;

    fn is_copyable() -> bool;

    /// copy if the type is copyable
    ///
    /// note that it should always be either some or none for a fixed type
    fn try_copy(&self) -> Option<Value>;

    fn is_some(&self) -> bool {
        panic!("type `{}` is not an Option", std::any::type_name::<Self>())
    }

    fn is_none(&self) -> bool {
        panic!("type `{}` is not an Option", std::any::type_name::<Self>())
    }

    fn index_owned(self, index: usize) -> ExceptedValue {
        panic!(
            "type `{}` doesn't support indexing owned",
            std::any::type_name::<Self>()
        )
    }

    fn index_ref<'a>(&'a self, index: usize) -> ExceptedValue {
        panic!(
            "type `{}` doesn't support indexing ref",
            std::any::type_name::<Self>()
        )
    }

    fn index_leash(&'static self, index: usize) -> ExceptedValue {
        panic!(
            "type `{}` doesn't support indexing leash",
            std::any::type_name::<Self>()
        )
    }

    fn unwrap_ref<'a>(&'a self) -> ExceptedValue {
        panic!(
            "type `{}` doesn't support unwrap",
            std::any::type_name::<Self>()
        )
    }

    fn unwrap_leash(&'static self) -> ExceptedValue {
        panic!(
            "type `{}` doesn't support unwrap",
            std::any::type_name::<Self>()
        )
    }

    fn serialize_to_value(&self) -> serde_json::Value;

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual;
}

pub trait ThawedDyn:
    std::fmt::Debug + std::any::Any + RefUnwindSafe + UnwindSafe + 'static
{
    unsafe fn snapshot(&self) -> Arc<dyn FrozenDyn>;

    fn type_name_dyn(&self) -> &'static str;

    fn is_some_dyn(&self) -> bool;

    fn is_none_dyn(&self) -> bool;

    fn index_owned_dyn(self: Box<Self>, index: usize) -> ExceptedValue;
    fn index_ref_dyn<'a>(&'a self, index: usize) -> ExceptedValue;
    fn index_leash_dyn(&'static self, index: usize) -> ExceptedValue;

    // todo: unwrap owned
    fn unwrap_ref_dyn<'a>(&'a self) -> ExceptedValue;
    fn unwrap_leash_dyn(&'static self) -> ExceptedValue;

    fn try_copy_dyn(&self) -> Option<Value>;

    fn present_dyn(&self) -> ValuePresentation;

    fn visualize_or_void_dyn(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual;
}

impl<T> ThawedDyn for T
where
    T: Thawed,
{
    unsafe fn snapshot(&self) -> Arc<dyn FrozenDyn> {
        Arc::new(self.freeze())
    }

    fn type_name_dyn(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn is_some_dyn(&self) -> bool {
        self.is_some()
    }

    fn is_none_dyn(&self) -> bool {
        self.is_none()
    }

    fn index_owned_dyn(self: Box<Self>, index: usize) -> ExceptedValue {
        self.index_owned(index)
    }

    fn index_ref_dyn<'a>(&'a self, index: usize) -> ExceptedValue {
        self.index_ref(index)
    }

    fn index_leash_dyn(&'static self, index: usize) -> ExceptedValue {
        self.index_leash(index)
    }

    fn unwrap_ref_dyn<'a>(&'a self) -> ExceptedValue {
        T::unwrap_ref(self)
    }

    fn unwrap_leash_dyn(&'static self) -> ExceptedValue {
        T::unwrap_leash(self)
    }

    fn try_copy_dyn(&self) -> Option<Value> {
        self.try_copy()
    }

    fn present_dyn(&self) -> ValuePresentation {
        // self.present()
        // ad hoc
        ValuePresentation::AdHoc(format!("{self:?}"))
    }

    fn visualize_or_void_dyn(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        self.visualize_or_void(visual_synchrotron)
    }
}

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
#[thawed_value_ty]
#[derive(Debug)]
#[repr(u8)]
pub enum ThawedValue {
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
    Owned(OwnedThawedValue),
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

impl IsThawedValue for ThawedValue {
    type Value = Value;

    fn r#move(&mut self) -> Self {
        std::mem::replace(self, ThawedValue::Moved)
    }
}

impl ThawedValue {
    pub fn from_owned<T>(t: T) -> Self
    where
        T: Thawed,
    {
        ThawedValue::Owned(OwnedThawedValue::upcast_from_owned(t))
    }

    pub fn into_ref<'a, T>(self, slush_values: Option<&mut SlushValues>) -> &'a T
    where
        T: Boiled,
    {
        match self {
            ThawedValue::Uninit => todo!(),
            ThawedValue::Invalid => todo!(),
            ThawedValue::Moved => todo!(),
            ThawedValue::Unit(_) => todo!(),
            ThawedValue::Bool(_) => todo!(),
            ThawedValue::Char(_) => todo!(),
            ThawedValue::I8(_) => todo!(),
            ThawedValue::I16(_) => todo!(),
            ThawedValue::I32(_) => todo!(),
            ThawedValue::I64(_) => todo!(),
            ThawedValue::I128(_) => todo!(),
            ThawedValue::ISize(_) => todo!(),
            ThawedValue::U8(_) => todo!(),
            ThawedValue::U16(_) => todo!(),
            ThawedValue::U32(_) => todo!(),
            ThawedValue::U64(_) => todo!(),
            ThawedValue::U128(_) => todo!(),
            ThawedValue::USize(_) => todo!(),
            ThawedValue::R8(_) => todo!(),
            ThawedValue::R16(_) => todo!(),
            ThawedValue::R32(_) => todo!(),
            ThawedValue::R64(_) => todo!(),
            ThawedValue::R128(_) => todo!(),
            ThawedValue::RSize(_) => todo!(),
            ThawedValue::F32(_) => todo!(),
            ThawedValue::F64(_) => todo!(),
            ThawedValue::StringLiteral(_) => todo!(),
            ThawedValue::Owned(slf) => {
                // todo: make the whole function unsafe
                let t: &T = slf.downcast_as_ref();
                let t = unsafe { std::mem::transmute(t) };
                slush_values
                    .unwrap()
                    .push(SlushValue::Box(slf.into_inner()));
                t
            }
            ThawedValue::Leash(slf) => {
                let slf: &<T as Boiled>::Thawed = ((slf as &dyn ThawedDyn) as &dyn std::any::Any)
                    .downcast_ref()
                    .expect("type id is correct");
                unsafe { std::mem::transmute(slf) }
            }
            ThawedValue::Ref(_) => todo!(),
            ThawedValue::Mut(_) => todo!(),
            ThawedValue::OptionBox(_) => todo!(),
            ThawedValue::OptionLeash(_) => todo!(),
            ThawedValue::OptionSizedRef(_) => todo!(),
            ThawedValue::OptionSizedMut(_) => todo!(),
            ThawedValue::EnumUnit { .. } => todo!(),
        }
    }
}

impl std::ops::Not for ThawedValue {
    type Output = Self;

    fn not(self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Shl for ThawedValue {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::ShlAssign for ThawedValue {
    fn shl_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::Shr for ThawedValue {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::ShrAssign for ThawedValue {
    fn shr_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::Sub for ThawedValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ThawedValue::I8(a), ThawedValue::I8(b)) => ThawedValue::I8(a - b),
            (ThawedValue::I16(a), ThawedValue::I16(b)) => ThawedValue::I16(a - b),
            (ThawedValue::I32(a), ThawedValue::I32(b)) => ThawedValue::I32(a - b),
            (ThawedValue::I64(a), ThawedValue::I64(b)) => ThawedValue::I64(a - b),
            (ThawedValue::I128(a), ThawedValue::I128(b)) => ThawedValue::I128(a - b),
            (ThawedValue::ISize(a), ThawedValue::ISize(b)) => ThawedValue::ISize(a - b),
            (ThawedValue::U8(a), ThawedValue::U8(b)) => ThawedValue::U8(a - b),
            (ThawedValue::U16(a), ThawedValue::U16(b)) => ThawedValue::U16(a - b),
            (ThawedValue::U32(a), ThawedValue::U32(b)) => ThawedValue::U32(a - b),
            (ThawedValue::U64(a), ThawedValue::U64(b)) => ThawedValue::U64(a - b),
            (ThawedValue::U128(a), ThawedValue::U128(b)) => ThawedValue::U128(a - b),
            (ThawedValue::USize(a), ThawedValue::USize(b)) => ThawedValue::USize(a - b),
            (ThawedValue::R8(a), ThawedValue::R8(b)) => ThawedValue::R8(a - b),
            (ThawedValue::R16(a), ThawedValue::R16(b)) => ThawedValue::R16(a - b),
            (ThawedValue::R32(a), ThawedValue::R32(b)) => ThawedValue::R32(a - b),
            (ThawedValue::R64(a), ThawedValue::R64(b)) => ThawedValue::R64(a - b),
            (ThawedValue::R128(a), ThawedValue::R128(b)) => ThawedValue::R128(a - b),
            (ThawedValue::RSize(a), ThawedValue::RSize(b)) => ThawedValue::RSize(a - b),
            (ThawedValue::F32(a), ThawedValue::F32(b)) => ThawedValue::F32(a - b),
            (ThawedValue::F64(a), ThawedValue::F64(b)) => ThawedValue::F64(a - b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::SubAssign for ThawedValue {
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}
