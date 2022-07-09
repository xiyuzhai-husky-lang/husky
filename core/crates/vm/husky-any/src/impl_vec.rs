use super::*;
use husky_entity_route::make_route;
use print_utils::msg_once;
use thin_vec::thin_vec;
use word::RootIdentifier;

impl<'a, T> __HasStaticTypeInfo for Vec<T>
where
    T: __HasStaticTypeInfo,
{
    type __StaticSelf = Vec<T::__StaticSelf>;

    fn __static_type_name() -> Cow<'static, str> {
        format!("[]{}", T::__static_type_name()).into()
    }
}

impl<'eval, 'a: 'eval, T: __AnyValue<'a>> __AnyValue<'eval> for Vec<T> {
    fn __clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }

    fn __print_short(&self) -> String {
        format!("{{ len: {}, data: [...] }}", self.len(),)
    }

    fn __to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::Value::Array(self.iter().map(|elem| elem.__to_json_value()).collect())
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        husky_entity_route::ty_route_with::<Self::__StaticSelf>(|| {
            make_route(
                RootIdentifier::Vec.into(),
                thin_vec![T::__static_ty().into()],
            )
        })
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Group(
            self.iter()
                .enumerate()
                .map(|(i, element)| visualize_element(i, element.__short()))
                .collect::<__EvalResult<Vec<_>>>()?,
        )))
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        todo!()
    }
}
