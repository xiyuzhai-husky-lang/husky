use super::*;
use husky_entity_route::make_route;
use print_utils::msg_once;
use thin_vec::thin_vec;
use word::RootIdentifier;

impl<'a, T> HasStaticTypeInfo for Vec<T>
where
    T: HasStaticTypeInfo,
{
    type StaticSelf = Vec<T::StaticSelf>;

    fn static_type_name() -> Cow<'static, str> {
        format!("[]{}", T::static_type_name()).into()
    }
}

impl<'eval, 'a: 'eval, T: AnyValue<'a>> AnyValue<'eval> for Vec<T> {
    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{{ len: {}, data: [...] }}", self.len(),)
    }

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
            make_route(RootIdentifier::Vec.into(), thin_vec![T::static_ty().into()])
        })
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
