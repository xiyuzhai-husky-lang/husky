mod impl_primitive;
mod impl_slice;
mod impl_vec;

use print_utils::p;
use serde::Serialize;

use crate::*;
use std::{any::TypeId, borrow::Cow, fmt::Debug, panic::RefUnwindSafe, sync::Arc};

#[derive(Debug, PartialEq, Eq)]
pub enum StaticTypeId {
    RustBuiltin(TypeId),
    HuskyBuiltin(HuskyBuiltinStaticTypeId),
    VecOf(Box<StaticTypeId>),
}

impl From<TypeId> for StaticTypeId {
    fn from(id: TypeId) -> Self {
        Self::RustBuiltin(id)
    }
}

impl From<HuskyBuiltinStaticTypeId> for StaticTypeId {
    fn from(id: HuskyBuiltinStaticTypeId) -> Self {
        Self::HuskyBuiltin(id)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum HuskyBuiltinStaticTypeId {
    Dataset,
    VirtualTy,
    VirtualVec,
}

// type level trait
pub trait AnyValue<'eval>: Debug + Send + Sync + Sized + PartialEq + Clone + RefUnwindSafe {
    fn static_type_id() -> StaticTypeId;
    fn static_type_name() -> Cow<'static, str>;
    // fn clone_shared(&self) -> Arc<dyn AnyValueDyn<'eval>>;

    fn clone_into_box<'vm>(&self) -> Box<dyn AnyValueDyn<'eval> + 'vm>
    where
        Self: 'vm,
    {
        Box::new(self.clone())
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval,
    {
        Arc::new(self.clone())
    }

    fn from_stack<'vm>(stack_value: VMValue<'vm, 'eval>) -> Self {
        match stack_value {
            VMValue::FullyOwned(boxed_value) => boxed_value.take().unwrap(),
            _ => {
                p!(Self::static_type_name());
                p!(stack_value);
                panic!()
            }
        }
    }

    fn as_copyable(&self) -> CopyableValue {
        p!(self);
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{:?}", self)
    }
    fn to_json_value(&self) -> serde_json::value::Value;
}

// object safe trait
pub trait AnyValueDyn<'eval>: Debug + Send + Sync + RefUnwindSafe {
    fn static_type_id_dyn(&self) -> StaticTypeId;
    fn static_type_name_dyn(&self) -> Cow<'static, str>;
    fn clone_into_box_dyn<'vm>(&self) -> Box<dyn AnyValueDyn<'eval> + 'vm>
    where
        Self: 'vm;
    fn clone_into_arc_dyn(&self) -> Arc<dyn AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval;
    fn equal_any(&self, other: &dyn AnyValueDyn<'eval>) -> bool;
    fn assign<'vm>(&mut self, other: VMValue<'vm, 'eval>);
    fn take_copyable(&self) -> CopyableValue;
    fn upcast_any(&self) -> &(dyn AnyValueDyn<'eval>);
    fn print_short(&self) -> String;
    // consume the memory pointed at to create an Arc
    unsafe fn take_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval> + 'eval>
    where
        Self: 'eval;
    fn get_json_value_dyn(&self) -> serde_json::value::Value;
}

impl<'vm, 'eval: 'vm> dyn AnyValueDyn<'eval> + 'vm {
    #[inline]
    pub fn downcast_ref<T: AnyValue<'eval>>(&self) -> &T {
        if T::static_type_id() != self.static_type_id_dyn() {
            p!(self.static_type_name_dyn(), T::static_type_name());
            panic!()
        }
        let ptr: *const dyn AnyValueDyn = &*self;
        let ptr: *const T = ptr as *const T;
        unsafe { &*ptr }
    }

    #[inline]
    pub fn downcast_mut<T: AnyValue<'eval>>(&mut self) -> &mut T {
        if T::static_type_id() != self.static_type_id_dyn() {
            p!(T::static_type_id(), self.static_type_id_dyn());
            panic!()
        }
        let ptr: *mut dyn AnyValueDyn = &mut *self;
        let ptr: *mut T = ptr as *mut T;
        unsafe { &mut *ptr }
    }
}

impl<'eval, T: AnyValue<'eval>> AnyValueDyn<'eval> for T {
    fn static_type_id_dyn(&self) -> StaticTypeId {
        T::static_type_id().into()
    }

    fn static_type_name_dyn(&self) -> Cow<'static, str> {
        T::static_type_name()
    }

    fn clone_into_box_dyn<'vm>(&self) -> Box<dyn AnyValueDyn<'eval> + 'vm>
    where
        Self: 'vm,
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

    fn assign<'vm>(&mut self, other: VMValue<'vm, 'eval>) {
        *self = T::from_stack(other)
    }

    fn take_copyable(&self) -> CopyableValue {
        T::as_copyable(self)
    }

    fn upcast_any(&self) -> &dyn AnyValueDyn<'eval> {
        self
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

    fn get_json_value_dyn(&self) -> serde_json::value::Value {
        self.to_json_value()
    }
}
