use super::*;

impl<'temp, 'eval, 'a: 'eval, T: AnyValue<'a> + 'temp> AnyValue<'eval> for &'temp [T] {
    fn static_type_id() -> StaticTypeId {
        StaticTypeId::Vec(Box::new(T::static_type_id()))
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

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }
}

fn gen_iter<'temp, 'eval: 'temp, T>(
    slice: &'temp [T],
) -> Box<dyn Iterator<Item = TempValue<'temp, 'eval>> + 'temp>
where
    T: AnyValueDyn<'eval> + 'eval,
{
    Box::new(slice.iter().map(|t| TempValue::TempRefEval(t)))
}
