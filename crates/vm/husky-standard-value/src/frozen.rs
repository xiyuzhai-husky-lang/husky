pub mod mut_frozen;
pub mod value;

use husky_decl_macro_utils::{for_all_primitive_tys, for_all_ritchie_tys};

use super::*;
use crate::r#static::{Static, StaticDyn};

pub trait Frozen: std::fmt::Debug + Clone + RefUnwindSafe + UnwindSafe + 'static {
    type Static: Static;
    type Stand: std::any::Any;

    /// this function gives back the value snapshoted,
    /// together with a stand so that the value is valid if the stand is not dropped.
    ///
    /// Returns None if Stand is trivial to save a call of `Box::new`.
    fn revive(&self) -> (Option<Self::Stand>, Self::Static);
}

pub trait SnapshotDyn: std::fmt::Debug {
    /// returns a owned type and the stand it needed
    fn revive_dyn(&self) -> (Option<Stand>, Box<dyn StaticDyn>);
    fn revive_ref_dyn(self: Arc<Self>) -> (Stand, *const dyn StaticDyn);
    fn revive_mut_dyn(&self) -> (Stand, *mut dyn StaticDyn);
}

impl<T> SnapshotDyn for T
where
    T: Frozen,
{
    fn revive_dyn(&self) -> (Option<Stand>, Box<dyn StaticDyn>) {
        let (stand, static_self) = self.revive();
        (
            stand.map(|stand| Stand::Box(Box::new(stand))),
            Box::new(static_self),
        )
    }

    fn revive_ref_dyn(self: Arc<Self>) -> (Stand, *const dyn StaticDyn) {
        let slf: *const <Self as Frozen>::Static =
            unsafe { std::mem::transmute(&*self as *const Self) };
        (Stand::Arc(self), slf)
    }

    fn revive_mut_dyn(&self) -> (Stand, *mut dyn StaticDyn) {
        let mut slf = self.clone();
        let slf_mut: *mut <Self as Frozen>::Static =
            unsafe { std::mem::transmute(&mut slf as *mut Self) };
        (Stand::Box(Box::new(slf)), slf_mut)
    }
}

impl<T> Frozen for Vec<T>
where
    T: Frozen,
{
    type Static = Vec<T::Static>;

    type Stand = Vec<T::Stand>;

    fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
        todo!()
    }
}

impl<T> Frozen for &'static T
where
    T: Static,
{
    type Static = Self;

    type Stand = ();

    fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
        (None, *self)
    }
}

impl<T> Frozen for Option<T>
where
    T: Frozen,
{
    type Static = Option<T::Static>;

    type Stand = T::Stand;

    fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
        // (None,self.as_ref().map(|t|t.revive()))
        match self {
            Some(slf) => {
                let (stand, revived) = slf.revive();
                (stand, Some(revived))
            }
            None => (None, None),
        }
    }
}

pub enum Stand {
    Box(Box<dyn std::any::Any>),
    Arc(Arc<dyn SnapshotDyn>),
}

macro_rules! impl_frozen_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl Frozen for $primitive_ty {
            type Static = Self;
            type Stand = ();

            fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
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
            $($input: Static, )*
            $output: Static, {
            type Static = Self;
            type Stand = ();

            fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
                (None, *self)
            }
        }
    };
}

for_all_ritchie_tys!(impl_frozen_for_ritchie_ty);
