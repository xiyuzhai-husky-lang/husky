use super::*;

impl<'a, T> HasStaticTypeInfo for Vec<T>
where
    T: HasStaticTypeInfo,
{
    type StaticSelf = Vec<T::StaticSelf>;

    fn static_type_name() -> Cow<'static, str> {
        format!("[]{}", T::static_type_name()).into()
    }
}

impl<'eval, 'a: 'eval, T: __AnyValue<'a>> __AnyValue<'eval> for Vec<T> {
    fn clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{{ len: {}, data: [...] }}", self.len(),)
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::Value::Array(self.iter().map(|elem| elem.to_json_value()).collect())
    }

    fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn ty(&self) -> EntityRoutePtr {
        todo!()
    }
}
