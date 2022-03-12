use crate::*;
use std::{
    any::{Any, TypeId},
    borrow::Cow,
    fmt::Debug,
    sync::Arc,
};

// type level trait
pub trait AnyValue: Debug + Send + Sync + Sized {
    fn static_type_id() -> TypeId;
    fn static_type_name() -> Cow<'static, str>;
    fn clone_any(&self) -> Box<dyn AnyValueDyn>;
    fn snapshot(&self) -> Arc<dyn AnyValueDyn>;
    fn from_stack(stack_value: StackValue) -> Self {
        match stack_value {
            StackValue::Boxed(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }
    fn as_primitive(&self) -> PrimitiveValue {
        panic!()
    }
}

// object safe trait
pub trait AnyValueDyn: Debug + Send + Sync {
    fn static_type_id(&self) -> TypeId;
    fn static_type_name(&self) -> Cow<'static, str>;
    fn clone_any(&self) -> Box<dyn AnyValueDyn>;
    fn snapshot(&self) -> Arc<dyn AnyValueDyn>;
    fn equal_any(&self, other: &dyn AnyValueDyn) -> bool;
    fn assign<'stack, 'eval>(&mut self, other: StackValue<'stack, 'eval>);
    fn as_primitive(&self) -> PrimitiveValue;
}

impl<T: AnyValue> AnyValueDyn for T {
    fn static_type_id(&self) -> TypeId {
        T::static_type_id()
    }

    fn static_type_name(&self) -> Cow<'static, str> {
        T::static_type_name()
    }

    fn clone_any(&self) -> Box<dyn AnyValueDyn> {
        T::clone_any(self)
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn> {
        T::snapshot(self)
    }

    fn equal_any(&self, other: &dyn AnyValueDyn) -> bool {
        todo!()
    }

    fn assign<'stack, 'eval>(&mut self, other: StackValue<'stack, 'eval>) {
        *self = T::from_stack(other)
    }

    fn as_primitive(&self) -> PrimitiveValue {
        T::as_primitive(self)
    }
}

impl AnyValue for i32 {
    fn static_type_id() -> TypeId {
        TypeId::of::<Self>()
    }

    fn static_type_name() -> Cow<'static, str> {
        "i32".into()
    }

    fn clone_any(&self) -> Box<dyn AnyValueDyn> {
        Box::new(*self)
    }

    fn as_primitive(&self) -> PrimitiveValue {
        self.into()
    }

    fn from_stack(stack_value: StackValue) -> Self {
        match stack_value {
            StackValue::Primitive(PrimitiveValue::I32(value)) => value,
            StackValue::Boxed(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn> {
        Arc::new(*self)
    }
}

impl AnyValue for f32 {
    fn static_type_id() -> TypeId {
        TypeId::of::<Self>()
    }

    fn static_type_name() -> Cow<'static, str> {
        "f32".into()
    }

    fn clone_any(&self) -> Box<dyn AnyValueDyn> {
        Box::new(*self)
    }

    fn as_primitive(&self) -> PrimitiveValue {
        self.into()
    }

    fn from_stack(stack_value: StackValue) -> Self {
        match stack_value {
            StackValue::Primitive(PrimitiveValue::F32(value)) => value,
            StackValue::Boxed(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn> {
        Arc::new(*self)
    }
}

impl AnyValue for u32 {
    fn static_type_id() -> TypeId {
        TypeId::of::<Self>()
    }

    fn static_type_name() -> Cow<'static, str> {
        "u32".into()
    }

    fn clone_any(&self) -> Box<dyn AnyValueDyn> {
        Box::new(*self)
    }

    fn as_primitive(&self) -> PrimitiveValue {
        self.into()
    }

    fn from_stack(stack_value: StackValue) -> Self {
        match stack_value {
            StackValue::Primitive(PrimitiveValue::B32(value)) => value,
            StackValue::Boxed(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn> {
        Arc::new(*self)
    }
}

impl AnyValue for u64 {
    fn static_type_id() -> TypeId {
        TypeId::of::<Self>()
    }

    fn static_type_name() -> Cow<'static, str> {
        "u64".into()
    }

    fn clone_any(&self) -> Box<dyn AnyValueDyn> {
        Box::new(*self)
    }

    fn as_primitive(&self) -> PrimitiveValue {
        self.into()
    }

    fn from_stack(stack_value: StackValue) -> Self {
        match stack_value {
            StackValue::Primitive(PrimitiveValue::B64(value)) => value,
            StackValue::Boxed(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn> {
        Arc::new(*self)
    }
}

impl AnyValue for bool {
    fn static_type_id() -> TypeId {
        TypeId::of::<Self>()
    }

    fn static_type_name() -> Cow<'static, str> {
        "bool".into()
    }

    fn clone_any(&self) -> Box<dyn AnyValueDyn> {
        Box::new(*self)
    }

    fn from_stack(stack_value: StackValue) -> Self {
        match stack_value {
            StackValue::Primitive(PrimitiveValue::Bool(value)) => value,
            StackValue::Boxed(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn> {
        Arc::new(*self)
    }
}
