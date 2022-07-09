use husky_entity_route::EntityRoutePtr;
use husky_trace_protocol::VisualData;
use word::RootIdentifier;

use crate::*;

impl __HasStaticTypeInfo for VisualData {
    type __StaticSelf = Self;

    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval> __AnyValue<'eval> for VisualData {
    fn __to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        RootIdentifier::VisualType.into()
    }

    fn __ty(&self) -> EntityRoutePtr {
        RootIdentifier::VisualType.into()
    }

    fn __print_short(&self) -> String {
        panic!()
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        panic!()
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        todo!()
    }
}
