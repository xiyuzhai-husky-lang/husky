mod binding;
mod impl_cyclic_slice;
mod impl_hashmap;
mod impl_label;
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
use husky_print_utils::p;
use serde::Serialize;
use std::{
    any::TypeId,
    borrow::Cow,
    fmt::Debug,
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Arc,
};
use utils::*;

pub trait __HasStaticTypeInfo {
    type __StaticSelf: 'static;
    fn __static_type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self::__StaticSelf>()
    }
    fn __static_type_name() -> Cow<'static, str>;
}

// type level trait
pub trait __AnyValue<'eval>:
    Debug + Send + Sync + Sized + PartialEq + Clone + RefUnwindSafe + UnwindSafe + __HasStaticTypeInfo
{
    // fn clone_shared(&self) -> Arc<dyn __AnyValueDyn<'eval>>;

    fn __clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(self.clone())
    }

    fn __clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval,
    {
        Arc::new(self.clone())
    }

    fn __from_stack<'temp>(stack_value: __TempValue<'temp, 'eval>) -> Self {
        match stack_value {
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => {
                p!(Self::__static_type_name());
                p!(stack_value);
                panic!()
            }
        }
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp;

    fn __into_eval_value(self) -> __EvalValue<'eval>
    where
        Self: 'eval;

    fn __take_copyable(&self) -> CopyableValue {
        p!(self);
        panic!()
    }

    fn __print_short(&self) -> String;
    //     format!("{:?}", self)
    // }
    fn __to_json_value(&self) -> serde_json::value::Value;
    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short;
    fn __static_ty() -> EntityRoutePtr;
    fn __ty(&self) -> EntityRoutePtr {
        Self::__static_ty()
    }
    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(None)
    }
}

// object safe trait
pub trait __AnyValueDyn<'eval>: Debug + Send + Sync + RefUnwindSafe + UnwindSafe {
    fn __static_type_id_dyn(&self) -> std::any::TypeId;
    fn __static_type_name_dyn(&self) -> Cow<'static, str>;
    // fn clone_into_copyable_dyn<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    // where
    //     Self: 'temp;
    fn __clone_into_box_dyn<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp;
    fn __clone_into_arc_dyn(&self) -> Arc<dyn __AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval;
    fn __equal_any(&self, other: &dyn __AnyValueDyn<'eval>) -> bool;
    fn __assign<'temp>(&mut self, other: __TempValue<'temp, 'eval>);
    fn __take_copyable_dyn(&self) -> CopyableValue;
    fn __upcast_any(&self) -> &(dyn __AnyValueDyn<'eval>);
    unsafe fn __upcast_arb_any_ref<'a>(&self) -> &'a (dyn __AnyValueDyn<'eval>)
    where
        Self: 'a;
    unsafe fn __upcast_arb_any_mut<'a>(&mut self) -> &'a mut (dyn __AnyValueDyn<'eval>)
    where
        Self: 'a;
    fn __print_short(&self) -> String;
    fn __short_dyn<'shorter_eval>(&self) -> &dyn __AnyValueDyn<'shorter_eval>
    where
        'eval: 'shorter_eval;
    // consume the memory pointed at to create an Arc
    unsafe fn __take_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval;
    fn __to_json_value_dyn(&self) -> serde_json::value::Value;
    fn __ty_dyn(&self) -> EntityRoutePtr;
    fn __opt_visualize_dyn(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>>;
}

impl<'temp, 'eval: 'temp> dyn __AnyValueDyn<'eval> + 'temp {
    #[inline]
    pub fn __downcast_ref<'a, T: __AnyValue<'eval>>(&'a self) -> &'a T {
        if T::__static_type_id() != self.__static_type_id_dyn() {
            panic!(
                "expect type `{}`, but got `{}` instead",
                T::__static_type_name(),
                self.__static_type_name_dyn()
            )
        }
        let ptr: *const dyn __AnyValueDyn = &*self;
        let ptr: *const T = ptr as *const T;
        unsafe { &*ptr }
    }
    #[inline]
    pub fn __downcast_copy<'a, T: __AnyValue<'eval> + Copy>(&'a self) -> T {
        if T::__static_type_id() != self.__static_type_id_dyn() {
            panic!(
                "expect type `{}`, but got `{}` instead",
                T::__static_type_name(),
                self.__static_type_name_dyn()
            )
        }
        let ptr: *const dyn __AnyValueDyn = &*self;
        let ptr: *const T = ptr as *const T;
        unsafe { *ptr }
    }

    #[inline]
    pub fn __downcast_mut<T: __AnyValue<'eval>>(&mut self) -> &mut T {
        if T::__static_type_id() != self.__static_type_id_dyn() {
            panic!(
                "expect type `{}`, but got `{}` instead",
                T::__static_type_name(),
                self.__static_type_name_dyn()
            )
        }
        let ptr: *mut dyn __AnyValueDyn = &mut *self;
        let ptr: *mut T = ptr as *mut T;
        unsafe { &mut *ptr }
    }
}

impl<'eval, T: __AnyValue<'eval>> __AnyValueDyn<'eval> for T {
    fn __static_type_id_dyn(&self) -> std::any::TypeId {
        T::__static_type_id()
    }

    fn __static_type_name_dyn(&self) -> Cow<'static, str> {
        T::__static_type_name()
    }

    fn __clone_into_box_dyn<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        T::__clone_into_box(self)
    }

    fn __clone_into_arc_dyn(&self) -> Arc<dyn __AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval,
    {
        T::__clone_into_arc(self)
    }

    fn __equal_any(&self, other: &dyn __AnyValueDyn<'eval>) -> bool {
        let other: &T = other.__downcast_ref();
        self == other
    }

    fn __assign<'temp>(&mut self, other: __TempValue<'temp, 'eval>) {
        *self = T::__from_stack(other)
    }

    fn __take_copyable_dyn(&self) -> CopyableValue {
        T::__take_copyable(self)
    }

    fn __upcast_any(&self) -> &dyn __AnyValueDyn<'eval> {
        self
    }

    unsafe fn __upcast_arb_any_ref<'a>(&self) -> &'a (dyn __AnyValueDyn<'eval>)
    where
        T: 'a,
    {
        let ptr: *const T = self;
        &*ptr
    }

    unsafe fn __upcast_arb_any_mut<'a>(&mut self) -> &'a mut (dyn __AnyValueDyn<'eval>)
    where
        Self: 'a,
    {
        let ptr: *mut T = self;
        &mut *ptr
    }

    fn __print_short(&self) -> String {
        T::__print_short(self)
    }

    // must use this for a raw pointer dropped from box
    unsafe fn __take_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval,
    {
        let ptr: *mut Self = self as *const Self as *mut Self;
        let this: Self = *Box::from_raw(ptr);
        Arc::new(this)
    }

    fn __to_json_value_dyn(&self) -> serde_json::value::Value {
        self.__to_json_value()
    }

    fn __short_dyn<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self.__short()
    }

    fn __ty_dyn(&self) -> EntityRoutePtr {
        self.__ty()
    }

    fn __opt_visualize_dyn(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        self.__opt_visualize(visualize_element)
    }
}
