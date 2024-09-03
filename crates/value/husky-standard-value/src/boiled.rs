mod r#mut;
mod option;
mod primitive;
mod r#ref;
mod ritchie;
mod str;
mod vec;

use husky_decl_macro_utils::{for_all_primitive_tys, for_all_ritchie_tys};

use crate::thawed::{r#mut::ThawedMut, r#ref::ThawedRef, Thawed};

pub trait Boiled {
    type Thawed: Thawed;

    fn type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self::Thawed>()
    }

    fn full_type_name() -> std::borrow::Cow<'static, str> {
        std::any::type_name::<Self>().into()
    }

    /// should call `std::mem::transmute` under the hood
    unsafe fn into_thawed(self) -> Self::Thawed
    where
        Self: Sized;
}

// impl<T> Boiled for *mut T
// where
//     T: Boiled,
// {
//     type Thawed = *mut T::Thawed;

//     fn full_type_name() -> std::borrow::Cow<'static, str> {
//         format!("&mut {}", T::full_type_name()).into()
//     }

//     unsafe fn into_thawed(self) -> Self::Thawed {
//         unsafe { std::mem::transmute(self) }
//     }
// }
