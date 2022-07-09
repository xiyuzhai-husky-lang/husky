use super::*;
use std::collections::HashMap;

impl<K, V> __HasStaticTypeInfo for HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + __HasStaticTypeInfo,
    V: __HasStaticTypeInfo,
{
    type __StaticSelf = HashMap<K::__StaticSelf, V::__StaticSelf>;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval, 'a: 'eval, K, V> __AnyValue<'eval> for HashMap<K, V>
where
    K: __AnyValue<'a> + std::cmp::Eq + std::hash::Hash + 'a,
    V: __AnyValue<'a> + 'a,
{
    fn __clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }

    fn __print_short(&self) -> String {
        format!("{{ len: {}, data: [...] }}", self.len(),)
    }

    fn __to_json_value(&self) -> serde_json::value::Value {
        todo!()
        // serde_json::value::Value::Array(self.iter().map(|elem| elem.to_json_value()).collect())
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
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
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        todo!()
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        __EvalValue::Owned(__OwnedValue::new(self))
    }
}
