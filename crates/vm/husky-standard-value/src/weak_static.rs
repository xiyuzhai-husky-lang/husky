use husky_decl_macro_utils::{for_all_primitive_tys, for_all_ritchie_tys};

use crate::r#static::Static;

pub trait WeakStatic {
    type Static: WeakStatic<Static = Self::Static> + Static;

    fn type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self::Static>()
    }

    fn full_type_name() -> std::borrow::Cow<'static, str> {
        std::any::type_name::<Self>().into()
    }

    /// should call `std::mem::transmute` under the hood
    unsafe fn into_static(self) -> Self::Static
    where
        Self: Sized;
}

impl<T> WeakStatic for &T
where
    T: WeakStatic,
{
    type Static = &'static T::Static;

    fn full_type_name() -> std::borrow::Cow<'static, str> {
        format!("&{}", T::full_type_name()).into()
    }

    unsafe fn into_static(self) -> Self::Static {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> WeakStatic for &mut T
where
    T: WeakStatic,
{
    type Static = *mut T::Static;

    fn full_type_name() -> std::borrow::Cow<'static, str> {
        format!("&mut {}", T::full_type_name()).into()
    }

    unsafe fn into_static(self) -> Self::Static {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> WeakStatic for *mut T
where
    T: WeakStatic,
{
    type Static = *mut T::Static;

    fn full_type_name() -> std::borrow::Cow<'static, str> {
        format!("&mut {}", T::full_type_name()).into()
    }

    unsafe fn into_static(self) -> Self::Static {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> WeakStatic for Option<T>
where
    T: WeakStatic,
{
    type Static = Option<T::Static>;

    fn full_type_name() -> std::borrow::Cow<'static, str> {
        format!("?{}", T::full_type_name()).into()
    }

    unsafe fn into_static(self) -> Self::Static {
        self.map(|slf| slf.into_static())
    }
}

impl<T> WeakStatic for Vec<T>
where
    T: WeakStatic,
{
    type Static = Vec<T::Static>;

    fn full_type_name() -> std::borrow::Cow<'static, str> {
        format!("?{}", T::full_type_name()).into()
    }

    unsafe fn into_static(self) -> Self::Static {
        unsafe { std::mem::transmute(self) }
    }
}

macro_rules! impl_weak_static_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl WeakStatic for $primitive_ty {
            type Static = Self;

            unsafe fn into_static(self) -> Self::Static {
                std::mem::transmute(self)
            }
        }
    };
}

for_all_primitive_tys!(impl_weak_static_for_primitive_ty);

macro_rules! impl_weak_static_for_ritchie_ty {
    (
        [$($input:ident),*], $output:ident
    ) => {
        impl<$($input,)* $output> WeakStatic for fn($($input,)*) -> $output
        where
            $($input: WeakStatic, )*
            $output: WeakStatic, {
            type Static = fn($(<$input as WeakStatic>::Static,)*) -> <$output as WeakStatic>::Static;

            unsafe fn into_static(self) -> Self::Static {
                std::mem::transmute(self)
            }
        }
    };
}

for_all_ritchie_tys!(impl_weak_static_for_ritchie_ty);
