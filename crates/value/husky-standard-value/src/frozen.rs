mod leash;
pub mod r#mut;
mod option;
mod primitive;
pub mod r#ref;
mod ritchie;
mod str;
mod tuple;
mod vec;

use super::*;
use crate::thawed::{Thawed, ThawedDyn};
use husky_decl_macro_utils::{
    for_all_non_unit_tuple_tys, for_all_primitive_tys, for_all_ritchie_tys,
};
use husky_value_interface::IsFrozenValue;
use husky_value_protocol::presentation::EnumUnitValuePresenter;
use slush::SlushValue;
use smallvec::SmallVec;

pub trait Frozen:
    std::fmt::Debug + Clone + RefUnwindSafe + UnwindSafe + Send + Sync + 'static
{
    type Thawed: Thawed;
    type Slush: std::any::Any;

    /// this function gives back the value snapshoted,
    /// together with a stand so that the value is valid if the stand is not dropped.
    ///
    /// Returns None if Slush is trivial to save a call of `Box::new`.
    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed);
}

pub trait FrozenDyn: std::fmt::Debug + Send + Sync {
    /// returns a owned type and the stand it needed
    fn thaw_dyn(&self) -> (Option<SlushValue>, Box<dyn ThawedDyn>);
    fn thaw_ref_dyn(self: Arc<Self>) -> (SlushValue, *const dyn ThawedDyn);
    fn thaw_mut_dyn(&self) -> (SlushValue, *mut dyn ThawedDyn);
}

impl<T> FrozenDyn for T
where
    T: Frozen,
{
    fn thaw_dyn(&self) -> (Option<SlushValue>, Box<dyn ThawedDyn>) {
        let (stand, thawed_self) = self.thaw();
        (
            stand.map(|stand| SlushValue::Box(Box::<T::Slush>::new(stand))),
            Box::<T::Thawed>::new(thawed_self),
        )
    }

    fn thaw_ref_dyn(self: Arc<Self>) -> (SlushValue, *const dyn ThawedDyn) {
        todo!()
        // let slf: *const <Self as Frozen>::Thawed =
        //     unsafe { std::mem::transmute(&*self as *const Self) };
        // (SlushValue::Arc(self), slf)
    }

    fn thaw_mut_dyn(&self) -> (SlushValue, *mut dyn ThawedDyn) {
        todo!()
        // let mut slf = self.clone();
        // let slf_mut: *mut <Self as Frozen>::Thawed =
        //     unsafe { std::mem::transmute(&mut slf as *mut Self) };
        // (SlushValue::Box(Box::new(slf)), slf_mut)
    }
}

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
///
/// we use Arc for everything on heap to reduce clone costs
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum FrozenValue {
    /// useful for snapshot caching on stack
    None,
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
    EnumUsize {
        index: usize,
        presenter: EnumUnitValuePresenter,
    },
    Box(Arc<dyn FrozenDyn>),
    Leash(&'static dyn FrozenDyn),
    SizedRef(Arc<dyn FrozenDyn>),
    SizedRefMut(Arc<dyn FrozenDyn>),
    OptionBox(Option<Arc<dyn FrozenDyn>>),
    OptionLeash(Option<&'static dyn FrozenDyn>),
    OptionSizedRef(Option<Arc<dyn FrozenDyn>>),
    OptionSizedRefMut(Option<Arc<dyn FrozenDyn>>),
    Intrinsic(Arc<dyn FrozenDyn>),
}

impl IsFrozenValue for FrozenValue {
    type Value = Value;

    fn thaw(&self) -> (SlushValue, Value) {
        todo!()
    }
}
