use super::*;
use std::collections::HashMap;

impl<K, V> HasStaticTypeInfo for HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + HasStaticTypeInfo,
    V: HasStaticTypeInfo,
{
    type __StaticSelf = HashMap<K::__StaticSelf, V::__StaticSelf>;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval, 'a: 'eval, K: AnyValue<'a>, V: AnyValue<'a>> AnyValue<'eval> for HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    fn __clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }

    fn __print_short(&self) -> String {
        format!("{{ len: {}, data: [...] }}", self.len(),)
    }

    fn __to_json_value(&self) -> serde_json::value::Value {
        todo!()
        // serde_json::value::Value::Array(self.iter().map(|elem| elem.to_json_value()).collect())
    }

    fn __short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        todo!()
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
