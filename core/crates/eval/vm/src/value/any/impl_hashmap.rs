use super::*;
use std::collections::HashMap;

impl<'eval, 'a: 'eval, K: AnyValue<'a>, V: AnyValue<'a>> AnyValue<'eval> for HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    fn static_type_id() -> StaticTypeId {
        StaticTypeId::HashMap(Box::new(K::static_type_id()), Box::new(V::static_type_id()))
    }

    fn static_type_name() -> Cow<'static, str> {
        todo!()
    }

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

    fn ty(&self) -> EntityRoutePtr {
        todo!()
    }
}
