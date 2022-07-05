use cyclic_slice::CyclicSlice;

use super::*;

impl<'a, T> HasStaticTypeInfo for CyclicSlice<'a, T>
where
    T: HasStaticTypeInfo,
{
    type StaticSelf = CyclicSlice<'static, T::StaticSelf>;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "CyclicSlice".into()
    }
}

impl<'eval, 'a: 'eval, 'b: 'eval, T: AnyValue<'a>> AnyValue<'eval> for CyclicSlice<'b, T> {
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::Value::Array(self.iter().map(|elem| elem.to_json_value()).collect())
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn static_ty() -> EntityRoutePtr {
        todo!()
    }

    fn print_short(&self) -> String {
        todo!()
    }

    fn opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        todo!()
    }
}
