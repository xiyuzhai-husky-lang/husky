use super::*;

impl<'eval, 'a: 'eval, T: AnyValue<'a>> AnyValue<'eval> for Vec<T> {
    fn static_type_id() -> StaticTypeId {
        StaticTypeId::Vec(Box::new(T::static_type_id()))
    }

    fn static_type_name() -> Cow<'static, str> {
        format!("[]{}", T::static_type_name()).into()
    }

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
}
