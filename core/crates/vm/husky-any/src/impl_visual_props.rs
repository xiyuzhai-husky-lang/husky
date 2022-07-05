use husky_entity_route::EntityRoutePtr;
use husky_trace_protocol::VisualData;
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

    fn static_ty() -> EntityRoutePtr {
        RootIdentifier::VisualType.into()
    }

    fn ty(&self) -> EntityRoutePtr {
        RootIdentifier::VisualType.into()
    }

    fn print_short(&self) -> String {
        panic!()
    }

    fn opt_visualize(
        &'static self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'static dyn AnyValueDyn<'static>,
        ) -> __EvalResult<VisualData>,
    ) -> Option<VisualData> {
        panic!()
    }
}
