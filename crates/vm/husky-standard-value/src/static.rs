use super::*;
use crate::frozen::{mut_frozen::MutFrozen, Frozen, SnapshotDyn};
use husky_decl_macro_utils::{for_all_primitive_tys, for_all_ritchie_tys};

/// Stand is the static version of a type
pub trait Static: std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type Frozen: Frozen<Static = Self>;
    unsafe fn freeze(&self) -> Self::Frozen;
}

impl<T> Static for *mut T
where
    T: Static,
{
    type Frozen = MutFrozen<T>;

    unsafe fn freeze(&self) -> Self::Frozen {
        MutFrozen::new(*self)
    }
}

pub trait StaticDyn: std::fmt::Debug + std::any::Any + RefUnwindSafe + UnwindSafe {
    unsafe fn snapshot(&self) -> Arc<dyn SnapshotDyn>;
}

impl<T> StaticDyn for T
where
    T: Static,
{
    unsafe fn snapshot(&self) -> Arc<dyn SnapshotDyn> {
        Arc::new(self.freeze())
    }
}

impl<T> Static for Vec<T>
where
    T: Static,
{
    type Frozen = Vec<T::Frozen>;

    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }
}

impl<T> Static for &'static T
where
    T: Static,
{
    type Frozen = Self;

    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }
}

impl<T> Static for Option<T>
where
    T: Static,
{
    type Frozen = Option<T::Frozen>;

    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }
}

macro_rules! impl_static_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl Static for $primitive_ty {
            type Frozen = Self;

            unsafe fn freeze(&self) -> Self::Frozen {
                *self
            }
        }
    };
}

for_all_primitive_tys!(impl_static_for_primitive_ty);

macro_rules! impl_static_for_ritchie_ty {
    (
        [$($input:ident),*], $output:ident
    ) => {
        impl<$($input,)* $output> Static for fn($($input,)*) -> $output
        where
            $($input: Static, )*
            $output: Static, {
            type Frozen = Self;

            unsafe fn freeze(&self) -> Self::Frozen {
                *self
            }
        }
    };
}

for_all_ritchie_tys!(impl_static_for_ritchie_ty);
