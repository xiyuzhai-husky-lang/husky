use super::*;
use core::hash::Hash;
use entity_route::EntityRoutePtr;
use serde::{ser::SerializeStruct, Serialize};
use std::{any::TypeId, borrow::Cow};
use word::RootIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnumKindValue {
    pub kind_idx: u8,
    pub route: EntityRoutePtr,
}

impl Serialize for EnumKindValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl HasStaticTypeInfo for EnumKindValue {
    type StaticSelf = Self;

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval> AnyValue<'eval> for EnumKindValue {
    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }

    fn take_copyable(&self) -> CopyableValue {
        CopyableValue::EnumKind(*self)
    }

    fn from_stack<'temp>(stack_value: TempValue<'temp, 'eval>) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::EnumKind(enum_kind)) => enum_kind,
            _ => {
                p!(Self::static_type_name());
                p!(stack_value);
                panic!()
            }
        }
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn ty(&self) -> EntityRoutePtr {
        self.route.parent()
    }

    fn print_short(&self) -> String {
        format!("{:?}", self.route)
    }
}
