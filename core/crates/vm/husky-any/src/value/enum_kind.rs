use super::*;
use core::hash::Hash;
use husky_entity_route::EntityRoutePtr;
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
    type __StaticSelf = Self;

    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval> AnyValue<'eval> for EnumKindValue {
    fn __clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }

    fn __take_copyable(&self) -> CopyableValue {
        CopyableValue::EnumKind(*self)
    }

    fn __from_stack<'temp>(stack_value: __TempValue<'temp, 'eval>) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::EnumKind(enum_kind)) => enum_kind,
            _ => {
                p!(Self::__static_type_name());
                p!(stack_value);
                panic!()
            }
        }
    }

    fn __to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }

    fn __short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        panic!()
    }

    fn __ty(&self) -> EntityRoutePtr {
        self.route.parent()
    }

    fn __print_short(&self) -> String {
        format!("{:?}", self.route)
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        todo!()
    }
}
