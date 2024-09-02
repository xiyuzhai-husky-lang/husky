pub mod r#mut;
mod option;
mod primitive;
pub mod r#ref;
mod ritchie;
mod str;
pub mod value;
mod vec;

use husky_decl_macro_utils::{
    for_all_non_unit_tuple_tys, for_all_primitive_tys, for_all_ritchie_tys,
};
use smallvec::SmallVec;

use super::*;
use crate::thawed::{Thawed, ThawedDyn};

pub trait Frozen:
    std::fmt::Debug + Clone + RefUnwindSafe + UnwindSafe + Send + Sync + 'static
{
    type Thawed: Thawed;
    type Slush: std::any::Any;

    /// this function gives back the value snapshoted,
    /// together with a stand so that the value is valid if the stand is not dropped.
    ///
    /// Returns None if Slush is trivial to save a call of `Box::new`.
    fn revive(&self) -> (Option<Self::Slush>, Self::Thawed);
}

pub trait FrozenDyn: std::fmt::Debug + Send + Sync {
    /// returns a owned type and the stand it needed
    fn revive_dyn(&self) -> (Option<SlushValue>, Box<dyn ThawedDyn>);
    fn revive_ref_dyn(self: Arc<Self>) -> (SlushValue, *const dyn ThawedDyn);
    fn revive_mut_dyn(&self) -> (SlushValue, *mut dyn ThawedDyn);
}

impl<T> FrozenDyn for T
where
    T: Frozen,
{
    fn revive_dyn(&self) -> (Option<SlushValue>, Box<dyn ThawedDyn>) {
        let (stand, thawed_self) = self.revive();
        (
            stand.map(|stand| SlushValue::Box(Box::<T::Slush>::new(stand))),
            Box::<T::Thawed>::new(thawed_self),
        )
    }

    fn revive_ref_dyn(self: Arc<Self>) -> (SlushValue, *const dyn ThawedDyn) {
        todo!()
        // let slf: *const <Self as Frozen>::Thawed =
        //     unsafe { std::mem::transmute(&*self as *const Self) };
        // (SlushValue::Arc(self), slf)
    }

    fn revive_mut_dyn(&self) -> (SlushValue, *mut dyn ThawedDyn) {
        todo!()
        // let mut slf = self.clone();
        // let slf_mut: *mut <Self as Frozen>::Thawed =
        //     unsafe { std::mem::transmute(&mut slf as *mut Self) };
        // (SlushValue::Box(Box::new(slf)), slf_mut)
    }
}

impl<T> Frozen for Vec<T>
where
    T: Frozen,
{
    type Thawed = Vec<T::Thawed>;

    type Slush = Vec<T::Slush>;

    fn revive(&self) -> (Option<Self::Slush>, Self::Thawed) {
        todo!()
    }
}

// impl<T> Frozen for &'static T
// where
//     T: Thawed + Send + Sync,
// {
//     type Thawed = Self;

//     type Slush = ();

//     fn revive(&self) -> (Option<Self::Slush>, Self::Thawed) {
//         (None, *self)
//     }
// }

pub enum SlushValue {
    Box(Box<dyn std::any::Any>),
    Arc(Arc<dyn FrozenDyn>),
}

pub type SlushValues = SmallVec<[SlushValue; 8]>;

macro_rules! impl_frozen_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl Frozen for $primitive_ty {
            type Thawed = Self;
            type Slush = ();

            fn revive(&self) -> (Option<Self::Slush>, Self::Thawed) {
                (None, *self)
            }
        }
    };
}

for_all_primitive_tys!(impl_frozen_for_primitive_ty);

macro_rules! impl_frozen_for_ritchie_ty {
    (
        [$($input:ident),*], $output:ident
    ) => {
        impl<$($input,)* $output>  Frozen for fn($($input,)*) -> $output
        where
            $($input: Thawed, )*
            $output: Thawed, {
            type Thawed = Self;
            type Slush = ();

            fn revive(&self) -> (Option<Self::Slush>, Self::Thawed) {
                (None, *self)
            }
        }
    };
}

for_all_ritchie_tys!(impl_frozen_for_ritchie_ty);

macro_rules! impl_frozen_for_tuple_ty {
    (
        $($field:ident),*
    ) => {
        impl<$($field,)*> Frozen for ($($field,)*)
        where
            $($field: Frozen,)*
        {
            type Thawed = ($(<$field as Frozen>::Thawed,)*);
            type Slush = ($(<$field as Frozen>::Slush,)*);

            fn revive(&self) -> (Option<Self::Slush>, Self::Thawed) {
                todo!()
            }
        }
    };
}

for_all_non_unit_tuple_tys!(impl_frozen_for_tuple_ty);
