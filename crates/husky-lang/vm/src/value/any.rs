use std::{
    any::{Any, TypeId},
    borrow::Cow,
    fmt::Debug,
};

// type level trait
pub trait AnyValue: Debug + Send + Sync {
    fn static_type_id() -> TypeId;
    fn static_type_name() -> Cow<'static, str>;
}

// object safe trait
pub trait AnyValueDyn: Debug + Send + Sync {
    fn static_type_id(&self) -> TypeId;
    fn static_type_name(&self) -> Cow<'static, str>;
}

impl<T: AnyValue> AnyValueDyn for T {
    fn static_type_id(&self) -> TypeId {
        T::static_type_id()
    }
    fn static_type_name(&self) -> Cow<'static, str> {
        T::static_type_name()
    }
}

impl AnyValue for i32 {
    fn static_type_id() -> TypeId {
        TypeId::of::<Self>()
    }

    fn static_type_name() -> Cow<'static, str> {
        "i32".into()
    }
}

impl AnyValue for f32 {
    fn static_type_id() -> TypeId {
        TypeId::of::<Self>()
    }

    fn static_type_name() -> Cow<'static, str> {
        "f32".into()
    }
}

impl AnyValue for bool {
    fn static_type_id() -> TypeId {
        TypeId::of::<Self>()
    }

    fn static_type_name() -> Cow<'static, str> {
        "bool".into()
    }
}
