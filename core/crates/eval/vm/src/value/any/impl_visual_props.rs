use entity_route::EntityRoutePtr;
use word::RootIdentifier;

use crate::*;

impl HasStaticTypeInfo for VisualData {
    type StaticSelf = Self;

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval> AnyValue<'eval> for VisualData {
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
        RootIdentifier::VisualType.into()
    }
}
