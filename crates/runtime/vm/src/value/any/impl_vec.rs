use super::*;

impl<'eval, T: AnyValue<'eval>> AnyValue<'eval> for Vec<T> {
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

impl<'eval> AnyValue<'eval> for Vec<MemberValue<'eval>> {
    fn static_type_id() -> StaticTypeId {
        StaticTypeId::HuskyBuiltin(HuskyBuiltinStaticTypeId::VirtualVec)
    }

    fn static_type_name() -> Cow<'static, str> {
        "Vec".into()
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{{ len: {}, data: [...] }}", self.len(),)
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::Value::Array(self.iter().map(|elem| elem.get_json_value()).collect())
    }
}
