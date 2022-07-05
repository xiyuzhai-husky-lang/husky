use cyclic_slice::CyclicSlice;
use husky_entity_route::{entity_route_menu, make_route};
use thin_vec::thin_vec;
use word::RootIdentifier;

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
        husky_entity_route::ty_route_with::<Self::StaticSelf>(|| {
            make_route(
                entity_route_menu().std_slice_cyclic_slice,
                thin_vec![T::static_ty().into()],
            )
        })
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
        Ok(Some(VisualData::Group(
            self.iter()
                .enumerate()
                .map(|(i, element)| visualize_element(i, element.short()))
                .collect::<__EvalResult<Vec<_>>>()?,
        )))
    }
}
