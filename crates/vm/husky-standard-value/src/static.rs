use super::*;
use crate::frozen::{mut_frozen::MutFrozen, Frozen, SnapshotDyn};
use husky_decl_macro_utils::{
    for_all_non_unit_tuple_tys, for_all_primitive_tys, for_all_ritchie_tys,
};
use husky_value_protocol::presentation::ValuePresentation;
use serde::Serialize;

/// Stand is the static version of a type
pub trait Static: std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type Frozen: Frozen<Static = Self>;
    unsafe fn freeze(&self) -> Self::Frozen;

    fn copy(&self) -> Box<dyn StaticDyn> {
        panic!(
            "type `{}` is not copyable",
            std::any::type_name_of_val(self)
        )
    }

    fn is_some(&self) -> bool {
        panic!(
            "type `{}` is not an Option",
            std::any::type_name_of_val(self)
        )
    }

    fn is_none(&self) -> bool {
        panic!(
            "type `{}` is not an Option",
            std::any::type_name_of_val(self)
        )
    }

    fn index_ref<'a>(&'a self, index: usize) -> &'a dyn StaticDyn {
        panic!(
            "type `{}` doesn't support indexing",
            std::any::type_name_of_val(self)
        )
    }

    fn serialize_to_value(&self) -> serde_json::Value;
}

impl<T> Static for *mut T
where
    T: Static,
{
    type Frozen = MutFrozen<T>;

    unsafe fn freeze(&self) -> Self::Frozen {
        MutFrozen::new(*self)
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        todo!()
    }
}

pub trait StaticDyn:
    std::fmt::Debug + std::any::Any + RefUnwindSafe + UnwindSafe + 'static
{
    unsafe fn snapshot(&self) -> Arc<dyn SnapshotDyn>;

    fn type_name_dyn(&self) -> &'static str;

    fn is_some_dyn(&self) -> bool;

    fn is_none_dyn(&self) -> bool;

    fn index_ref_dyn<'a>(&'a self, index: usize) -> &'a dyn StaticDyn;

    fn copy_dyn(&self) -> Box<dyn StaticDyn>;

    fn present_dyn(&self) -> ValuePresentation;
}

impl<T> StaticDyn for T
where
    T: Static,
{
    unsafe fn snapshot(&self) -> Arc<dyn SnapshotDyn> {
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

    fn index_ref_dyn<'a>(&'a self, index: usize) -> &'a dyn StaticDyn {
        self.index_ref(index)
    }

    fn copy_dyn(&self) -> Box<dyn StaticDyn> {
        self.copy()
    }

    fn present_dyn(&self) -> ValuePresentation {
        self.present()
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

    fn index_ref<'a>(&'a self, index: usize) -> &'a dyn StaticDyn {
        &self[index]
    }

    fn serialize_to_value(&self) -> serde_json::Value {
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

    fn serialize_to_value(&self) -> serde_json::Value {
        todo!()
    }
}

impl<T> Static for Option<T>
where
    T: Static,
{
    type Frozen = Option<T::Frozen>;

    fn is_some(&self) -> bool {
        self.is_some()
    }
    fn is_none(&self) -> bool {
        self.is_none()
    }

    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        self.as_ref()
            .map(|slf| slf.serialize_to_value())
            .unwrap_or_default()
    }
}

macro_rules! impl_static_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl Static for $primitive_ty {
            type Frozen = Self;

            unsafe fn freeze(&self) -> Self::Frozen {
                *self
            }

            fn serialize_to_value(&self) -> serde_json::Value {
                serde_json::to_value(self).unwrap()
            }
        }
    };
}

for_all_primitive_tys!(impl_static_for_primitive_ty);

impl Static for &'static str {
    type Frozen = Self;

    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        todo!("&'static str serialize_to_value")
    }
}

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

            fn serialize_to_value(&self) -> serde_json::Value {
                todo!("impl_static_for_ritchie_ty serialize_to_value")
            }
        }
    };
}

for_all_ritchie_tys!(impl_static_for_ritchie_ty);

macro_rules! impl_static_for_non_unit_tuple_ty {
    (
        $($field:ident),*
    ) => {
        impl<$($field,)*> Static for ($($field,)*)
        where
            $($field: Static,)*
        {
            type Frozen = ($(<$field as Static>::Frozen,)*);

            unsafe fn freeze(&self) -> Self::Frozen {
                todo!()
            }

            fn serialize_to_value(&self) -> serde_json::Value {
                todo!("impl_static_for_non_unit_tuple_ty serialize_to_value")
            }
        }
    };
}

for_all_non_unit_tuple_tys!(impl_static_for_non_unit_tuple_ty);
