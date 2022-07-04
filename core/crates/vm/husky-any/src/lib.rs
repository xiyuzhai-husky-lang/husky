mod binding;
mod impl_cyclic_slice;
mod impl_hashmap;
mod impl_primitive;
mod impl_slice;
mod impl_vec;
mod impl_visual_props;
mod stack_idx;
mod utils;
mod value;
mod virtual_cyclic_slice;
mod virtual_struct;
mod virtual_vec;

pub use binding::*;
use husky_trace_protocol::*;
pub use stack_idx::*;
pub use value::*;
pub use virtual_cyclic_slice::*;
pub use virtual_struct::*;
pub use virtual_vec::*;

use husky_entity_route::EntityRoutePtr;
use print_utils::p;
use serde::Serialize;
use std::{
    any::TypeId,
    borrow::Cow,
    fmt::Debug,
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Arc,
};
use utils::*;

// #[derive(Debug, PartialEq, Eq)]
// pub enum StaticTypeId {
//     RustBuiltin(TypeId),
//     HuskyBuiltin(HuskyBuiltinStaticTypeId),
//     Vec(Box<StaticTypeId>),
//     HashMap(Box<StaticTypeId>, Box<StaticTypeId>),
//     CyclicSlice(Box<StaticTypeId>),
//     AnyMemberValue,
// }

// impl From<TypeId> for StaticTypeId {
//     fn from(id: TypeId) -> Self {
//         Self::RustBuiltin(id)
//     }
// }

// impl From<HuskyBuiltinStaticTypeId> for StaticTypeId {
//     fn from(id: HuskyBuiltinStaticTypeId) -> Self {
//         Self::HuskyBuiltin(id)
//     }
// }

// #[derive(Debug, PartialEq, Eq)]
// pub enum HuskyBuiltinStaticTypeId {
//     Dataset,
//     VirtualTy,
//     VirtualVec,
// }

pub trait HasStaticTypeInfo {
    type StaticSelf: 'static;
    fn static_type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self::StaticSelf>()
    }
    fn static_type_name() -> Cow<'static, str>;
}

// type level trait
pub trait AnyValue<'eval>:
    Debug + Send + Sync + Sized + PartialEq + Clone + RefUnwindSafe + UnwindSafe + HasStaticTypeInfo
{
    // fn clone_shared(&self) -> Arc<dyn __AnyValueDyn<'eval>>;

    fn clone_into_box<'temp>(&self) -> Box<dyn AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(self.clone())
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval,
    {
        Arc::new(self.clone())
    }

    fn from_stack<'temp>(stack_value: __TempValue<'temp, 'eval>) -> Self {
        match stack_value {
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => {
                p!(Self::static_type_name());
                p!(stack_value);
                panic!()
            }
        }
    }

    fn take_copyable(&self) -> CopyableValue {
        p!(self);
        panic!()
    }

    fn print_short(&self) -> String;
    //     format!("{:?}", self)
    // }
    fn to_json_value(&self) -> serde_json::value::Value;
    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short;
    fn static_ty() -> EntityRoutePtr;
    fn ty(&self) -> EntityRoutePtr {
        Self::static_ty()
    }
}

// object safe trait
pub trait AnyValueDyn<'eval>: Debug + Send + Sync + RefUnwindSafe + UnwindSafe {
    fn static_type_id_dyn(&self) -> std::any::TypeId;
    fn static_type_name_dyn(&self) -> Cow<'static, str>;
    // fn clone_into_copyable_dyn<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    // where
    //     Self: 'temp;
    fn clone_into_box_dyn<'temp>(&self) -> Box<dyn AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp;
    fn clone_into_arc_dyn(&self) -> Arc<dyn AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval;
    fn equal_any(&self, other: &dyn AnyValueDyn<'eval>) -> bool;
    fn assign<'temp>(&mut self, other: __TempValue<'temp, 'eval>);
    fn take_copyable_dyn(&self) -> CopyableValue;
    fn upcast_any(&self) -> &(dyn AnyValueDyn<'eval>);
    unsafe fn upcast_arb_any<'a>(&self) -> &'a (dyn AnyValueDyn<'eval>)
    where
        Self: 'a;
    fn print_short(&self) -> String;
    fn short_dyn<'shorter_eval>(&'eval self) -> &'shorter_eval dyn AnyValueDyn<'shorter_eval>
    where
        'eval: 'shorter_eval;
    // consume the memory pointed at to create an Arc
    unsafe fn take_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval;
    fn to_json_value_dyn(&self) -> serde_json::value::Value;
    fn ty_dyn(&self) -> EntityRoutePtr;
}

impl<'temp, 'eval: 'temp> dyn AnyValueDyn<'eval> + 'temp {
    #[inline]
    pub fn downcast_ref<'a, T: AnyValue<'eval>>(&'a self) -> &'a T {
        if T::static_type_id() != self.static_type_id_dyn() {
            panic!(
                "expect type `{}`, but got `{}` instead",
                T::static_type_name(),
                self.static_type_name_dyn()
            )
        }
        let ptr: *const dyn AnyValueDyn = &*self;
        let ptr: *const T = ptr as *const T;
        unsafe { &*ptr }
    }
    #[inline]
    pub fn downcast_copy<'a, T: AnyValue<'eval> + Copy>(&'a self) -> T {
        if T::static_type_id() != self.static_type_id_dyn() {
            panic!(
                "expect type `{}`, but got `{}` instead",
                T::static_type_name(),
                self.static_type_name_dyn()
            )
        }
        let ptr: *const dyn AnyValueDyn = &*self;
        let ptr: *const T = ptr as *const T;
        unsafe { *ptr }
    }

    #[inline]
    pub fn downcast_mut<T: AnyValue<'eval>>(&mut self) -> &mut T {
        if T::static_type_id() != self.static_type_id_dyn() {
            panic!(
                "expect type `{}`, but got `{}` instead",
                T::static_type_name(),
                self.static_type_name_dyn()
            )
        }
        let ptr: *mut dyn AnyValueDyn = &mut *self;
        let ptr: *mut T = ptr as *mut T;
        unsafe { &mut *ptr }
    }
}

impl<'eval, T: AnyValue<'eval>> AnyValueDyn<'eval> for T {
    fn static_type_id_dyn(&self) -> std::any::TypeId {
        T::static_type_id()
    }

    fn static_type_name_dyn(&self) -> Cow<'static, str> {
        T::static_type_name()
    }

    fn clone_into_box_dyn<'temp>(&self) -> Box<dyn AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        T::clone_into_box(self)
    }

    fn clone_into_arc_dyn(&self) -> Arc<dyn AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval,
    {
        T::clone_into_arc(self)
    }

    fn equal_any(&self, other: &dyn AnyValueDyn) -> bool {
        todo!()
    }

    fn assign<'temp>(&mut self, other: __TempValue<'temp, 'eval>) {
        *self = T::from_stack(other)
    }

    fn take_copyable_dyn(&self) -> CopyableValue {
        T::take_copyable(self)
    }

    fn upcast_any(&self) -> &dyn AnyValueDyn<'eval> {
        self
    }

    unsafe fn upcast_arb_any<'a>(&self) -> &'a (dyn AnyValueDyn<'eval>)
    where
        T: 'a,
    {
        let ptr: *const T = self;
        &*ptr
    }

    fn print_short(&self) -> String {
        T::print_short(self)
    }

    // must use this for a raw pointer dropped from box
    unsafe fn take_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval,
    {
        let ptr: *mut Self = self as *const Self as *mut Self;
        let this: Self = *Box::from_raw(ptr);
        Arc::new(this)
    }

    fn to_json_value_dyn(&self) -> serde_json::value::Value {
        self.to_json_value()
    }

    fn short_dyn<'short>(&'eval self) -> &'short dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self.short()
    }

    fn ty_dyn(&self) -> EntityRoutePtr {
        self.ty()
    }
}
