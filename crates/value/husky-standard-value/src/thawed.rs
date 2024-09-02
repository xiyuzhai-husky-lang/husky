pub mod r#mut;
pub mod option;
pub mod primitive;
pub mod r#ref;
mod ritchie;
mod str;
pub mod vec;

use super::*;
use crate::frozen::{r#mut::FrozenMut, Frozen, FrozenDyn};
use husky_decl_macro_utils::{
    for_all_non_unit_tuple_tys, for_all_primitive_tys, for_all_ritchie_tys,
};
use husky_value_protocol::presentation::ValuePresentation;
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};

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

// impl<T> Thawed for *mut T
// where
//     T: Thawed,
// {
//     type Frozen = FrozenMut<T>;

//     fn is_copyable() -> bool {
//         false
//     }

//     fn try_copy(&self) -> Option<Value> {
//         None
//     }

//     unsafe fn freeze(&self) -> Self::Frozen {
//         FrozenMut::new(*self)
//     }

//     fn serialize_to_value(&self) -> serde_json::Value {
//         todo!()
//     }

//     fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
//         todo!()
//     }
// }

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

// impl<T> Thawed for &'static T
// where
//     T: Thawed,
// {
//     type Frozen = T::Frozen;

//     fn is_copyable() -> bool {
//         true
//     }

//     fn try_copy(&self) -> Option<Value> {
//         Some(Value::from_ref(self))
//     }

//     unsafe fn freeze(&self) -> Self::Frozen {
//         todo!()
//     }

//     fn serialize_to_value(&self) -> serde_json::Value {
//         todo!()
//     }

//     fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
//         todo!()
//     }
// }

macro_rules! impl_thawed_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl Thawed for $primitive_ty {
            type Frozen = Self;

            fn is_copyable() -> bool {
                true
            }

            fn try_copy(&self) -> Option<Value> {
                Some((*self).into())
            }

            unsafe fn freeze(&self) -> Self::Frozen {
                *self
            }

            fn serialize_to_value(&self) -> serde_json::Value {
                serde_json::to_value(self).unwrap()
            }

            fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
                todo!("")
            }
        }
    };
}

for_all_primitive_tys!(impl_thawed_for_primitive_ty);

// impl Thawed for &'static str {
//     type Frozen = Self;

//     fn is_copyable() -> bool {
//         todo!()
//     }

//     fn try_copy(&self) -> Option<Value> {
//         todo!()
//     }

//     unsafe fn freeze(&self) -> Self::Frozen {
//         todo!()
//     }

//     fn serialize_to_value(&self) -> serde_json::Value {
//         todo!("&'static str serialize_to_value")
//     }

//     fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
//         todo!()
//     }
// }

macro_rules! impl_thawed_for_ritchie_ty {
    (
        [$($input:ident),*], $output:ident
    ) => {
        impl<$($input,)* $output> Thawed for fn($($input,)*) -> $output
        where
            $($input: Thawed, )*
            $output: Thawed, {
            type Frozen = Self;

            fn is_copyable() -> bool {
                todo!()
            }

            fn try_copy(&self) -> Option<Value> {
                todo!()
            }

            unsafe fn freeze(&self) -> Self::Frozen {
                *self
            }

            fn serialize_to_value(&self) -> serde_json::Value {
                todo!("impl_thawed_for_ritchie_ty serialize_to_value")
            }

            fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
                Visual::Void
            }
        }
    };
}

for_all_ritchie_tys!(impl_thawed_for_ritchie_ty);

macro_rules! impl_thawed_for_non_unit_tuple_ty {
    (
        $($field:ident),*
    ) => {
        impl<$($field,)*> Thawed for ($($field,)*)
        where
            $($field: Thawed,)*
        {
            type Frozen = ($(<$field as Thawed>::Frozen,)*);

            fn is_copyable() -> bool {
                todo!()
            }

            fn try_copy(&self) -> Option<Value> {
                todo!()
            }

            unsafe fn freeze(&self) -> Self::Frozen {
                todo!()
            }

            fn serialize_to_value(&self) -> serde_json::Value {
                todo!("impl_thawed_for_non_unit_tuple_ty serialize_to_value")
            }

            fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
                todo!()
            }
        }
    };
}

for_all_non_unit_tuple_tys!(impl_thawed_for_non_unit_tuple_ty);
