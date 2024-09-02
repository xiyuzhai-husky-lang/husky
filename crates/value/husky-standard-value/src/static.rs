use super::*;
use crate::frozen::{mut_frozen::MutFrozen, Frozen, FrozenDyn};
use husky_decl_macro_utils::{
    for_all_non_unit_tuple_tys, for_all_primitive_tys, for_all_ritchie_tys,
};
use husky_value_protocol::presentation::ValuePresentation;
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};

/// Stand is the static version of a type
pub trait Static: Sized + std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type Frozen: Frozen<Static = Self>;
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

impl<T> Static for *mut T
where
    T: Static,
{
    type Frozen = MutFrozen<T>;

    fn is_copyable() -> bool {
        false
    }

    fn try_copy(&self) -> Option<Value> {
        None
    }

    unsafe fn freeze(&self) -> Self::Frozen {
        MutFrozen::new(*self)
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        todo!()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }
}

pub trait StaticDyn:
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

impl<T> StaticDyn for T
where
    T: Static,
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

impl<T> Static for Vec<T>
where
    T: Static,
{
    type Frozen = Vec<T::Frozen>;

    fn is_copyable() -> bool {
        false
    }

    fn try_copy(&self) -> Option<Value> {
        todo!()
    }

    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn index_ref<'a>(&'a self, index: usize) -> ExceptedValue {
        Ok(Value::from_ref(&self[index]))
    }

    fn index_leash(&'static self, index: usize) -> ExceptedValue {
        Ok(Value::from_leash(&self[index]))
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        todo!()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        let elements = self
            .iter()
            .map(|t| t.visualize_or_void(visual_synchrotron))
            .collect();
        Visual::new_group_visual(elements, visual_synchrotron)
    }
}

impl<T> Static for &'static T
where
    T: Static,
{
    type Frozen = Self;

    fn is_copyable() -> bool {
        true
    }

    fn try_copy(&self) -> Option<Value> {
        Some(Value::from_ref(self))
    }

    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        todo!()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }
}

impl<T> Static for Option<T>
where
    T: Static,
{
    type Frozen = Option<T::Frozen>;

    fn is_copyable() -> bool {
        T::is_copyable()
    }

    fn try_copy(&self) -> Option<Value> {
        if !Self::is_copyable() {
            return None;
        }
        self.as_ref().map(|v| v.try_copy().unwrap())
    }

    fn is_some(&self) -> bool {
        self.is_some()
    }
    fn is_none(&self) -> bool {
        self.is_none()
    }

    fn unwrap_ref<'a>(&'a self) -> ExceptedValue {
        let slf = self.as_ref().ok_or(Exception::UnwrapNone)?;
        Ok(match slf.try_copy() {
            Some(_) => todo!(),
            None => todo!(),
        })
    }

    fn unwrap_leash(&'static self) -> ExceptedValue {
        let slf = self.as_ref().ok_or(Exception::UnwrapNone)?;
        Ok(match slf.try_copy() {
            Some(slf) => slf,
            None => Value::from_leash(slf),
        })
    }

    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        self.as_ref()
            .map(|slf| slf.serialize_to_value())
            .unwrap_or_default()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }
}

macro_rules! impl_static_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl Static for $primitive_ty {
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

for_all_primitive_tys!(impl_static_for_primitive_ty);

impl Static for &'static str {
    type Frozen = Self;

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
        todo!("&'static str serialize_to_value")
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
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
                todo!("impl_static_for_ritchie_ty serialize_to_value")
            }

            fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
                Visual::Void
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
                todo!("impl_static_for_non_unit_tuple_ty serialize_to_value")
            }

            fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
                todo!()
            }
        }
    };
}

for_all_non_unit_tuple_tys!(impl_static_for_non_unit_tuple_ty);
