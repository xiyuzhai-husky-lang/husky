use super::*;

impl<'vm, 'eval: 'vm, T: AnyValue<'eval> + 'vm> AnyValue<'eval> for &'vm [T] {
    fn static_type_id() -> StaticTypeId {
        StaticTypeId::VecOf(Box::new(T::static_type_id()))
    }

    fn static_type_name() -> Cow<'static, str> {
        todo!()
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
        // serde_json::value::to_value(self).unwrap()
    }
}

fn gen_iter<'vm, 'eval: 'vm, T>(
    slice: &'vm [T],
) -> Box<dyn Iterator<Item = TempValue<'vm, 'eval>> + 'vm>
where
    T: AnyValueDyn<'eval> + 'eval,
{
    Box::new(slice.iter().map(|t| TempValue::FullyOwnedRef(t)))
}
