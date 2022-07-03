use super::*;
use std::collections::HashMap;

impl<K, V> HasStaticTypeInfo for HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + HasStaticTypeInfo,
    V: HasStaticTypeInfo,
{
    type StaticSelf = HashMap<K::StaticSelf, V::StaticSelf>;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval, 'a: 'eval, K: AnyValue<'a>, V: AnyValue<'a>> AnyValue<'eval> for HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{{ len: {}, data: [...] }}", self.len(),)
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
        // serde_json::value::Value::Array(self.iter().map(|elem| elem.to_json_value()).collect())
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
}
