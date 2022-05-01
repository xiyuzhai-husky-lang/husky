use print_utils::p;

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
pub trait AnyValue<'eval>:
    Debug + Send + Sync + Sized + PartialEq + Clone + RefUnwindSafe + 'eval
{
    fn static_type_id() -> StaticTypeId;
    fn static_type_name() -> Cow<'static, str>;
    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>>;

    fn boxed_any(&self) -> Box<dyn AnyValueDyn<'eval>> {
        Box::new(self.clone())
    }

    fn from_stack<'stack>(stack_value: StackValue<'stack, 'eval>) -> Self {
        match stack_value {
            StackValue::Boxed(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn as_primitive(&self) -> PrimitiveValue {
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{:?}", self)
    }
}

// object safe trait
pub trait AnyValueDyn<'eval>: Debug + Send + Sync + RefUnwindSafe + 'eval {
    fn static_type_id(&self) -> StaticTypeId;
    fn static_type_name(&self) -> Cow<'static, str>;
    fn clone_any(&self) -> Box<dyn AnyValueDyn<'eval>>;
    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>>;
    fn equal_any(&self, other: &dyn AnyValueDyn<'eval>) -> bool;
    fn assign<'stack>(&mut self, other: StackValue<'stack, 'eval>);
    fn as_primitive(&self) -> PrimitiveValue;
    fn upcast_any(&self) -> &(dyn AnyValueDyn<'eval> + 'eval);
    fn print_short(&self) -> String;
}

impl<'eval> dyn AnyValueDyn<'eval> {
    #[inline]
    pub fn downcast_ref<T: AnyValue<'eval>>(&self) -> &T {
        if T::static_type_id() != self.static_type_id() {
            panic!()
        }
        let ptr: *const dyn AnyValueDyn = &*self;
        let ptr: *const T = ptr as *const T;
        unsafe { &*ptr }
    }

    #[inline]
    pub fn downcast_mut<T: AnyValue<'eval>>(&mut self) -> &mut T {
        if T::static_type_id() != self.static_type_id() {
            p!(T::static_type_id(), self.static_type_id());
            panic!()
        }
        let ptr: *mut dyn AnyValueDyn = &mut *self;
        let ptr: *mut T = ptr as *mut T;
        unsafe { &mut *ptr }
    }
}

impl<'eval, T: AnyValue<'eval>> AnyValueDyn<'eval> for T {
    fn static_type_id(&self) -> StaticTypeId {
        T::static_type_id().into()
    }

    fn static_type_name(&self) -> Cow<'static, str> {
        T::static_type_name()
    }

    fn clone_any(&self) -> Box<dyn AnyValueDyn<'eval>> {
        T::boxed_any(self)
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        T::snapshot(self)
    }

    fn equal_any(&self, other: &dyn AnyValueDyn) -> bool {
        todo!()
    }

    fn assign<'stack>(&mut self, other: StackValue<'stack, 'eval>) {
        *self = T::from_stack(other)
    }

    fn as_primitive(&self) -> PrimitiveValue {
        T::as_primitive(self)
    }

    fn upcast_any(&self) -> &dyn AnyValueDyn<'eval> {
        self
    }
    fn print_short(&self) -> String {
        T::print_short(self)
    }
}

impl<'eval> AnyValue<'eval> for i32 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "i32".into()
    }

    fn boxed_any(&self) -> Box<dyn AnyValueDyn<'eval>> {
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

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        Arc::new(*self)
    }
}

impl<'eval> AnyValue<'eval> for f32 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "f32".into()
    }

    fn boxed_any(&self) -> Box<dyn AnyValueDyn<'eval>> {
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

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        Arc::new(*self)
    }
}

impl<'eval> AnyValue<'eval> for u32 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "u32".into()
    }

    fn boxed_any(&self) -> Box<dyn AnyValueDyn<'eval>> {
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

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        Arc::new(*self)
    }

    fn print_short(&self) -> String {
        format!("{:#032b}", self)
    }
}

impl<'eval> AnyValue<'eval> for u64 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "u64".into()
    }

    fn boxed_any(&self) -> Box<dyn AnyValueDyn<'eval>> {
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

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        Arc::new(*self)
    }

    fn print_short(&self) -> String {
        format!("{:#064b}", self)
    }
}

impl<'eval> AnyValue<'eval> for bool {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "bool".into()
    }

    fn boxed_any(&self) -> Box<dyn AnyValueDyn<'eval>> {
        Box::new(*self)
    }

    fn from_stack(stack_value: StackValue) -> Self {
        match stack_value {
            StackValue::Primitive(PrimitiveValue::Bool(value)) => value,
            StackValue::Boxed(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        Arc::new(*self)
    }
}

impl<'eval, T: AnyValue<'eval>> AnyValue<'eval> for Vec<T> {
    fn static_type_id() -> StaticTypeId {
        StaticTypeId::VecOf(Box::new(T::static_type_id()))
    }

    fn static_type_name() -> Cow<'static, str> {
        todo!()
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        todo!()
    }
}

impl<'eval> AnyValue<'eval> for Vec<MemberValue<'eval>> {
    fn static_type_id() -> StaticTypeId {
        StaticTypeId::HuskyBuiltin(HuskyBuiltinStaticTypeId::VirtualVec)
    }

    fn static_type_name() -> Cow<'static, str> {
        todo!()
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        todo!()
    }
}
