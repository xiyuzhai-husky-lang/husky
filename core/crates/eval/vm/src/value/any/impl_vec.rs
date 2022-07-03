use super::*;
use husky_entity_route::{lazy_entity_route, make_route};
use thin_vec::thin_vec;
use word::RootIdentifier;

impl<'a, T> HasStaticTypeInfo for Vec<T>
where
    T: HasStaticTypeInfo,
{
    type StaticSelf = Vec<T::StaticSelf>;

    fn static_type_name() -> Cow<'static, str> {
        format!("[]{}", T::static_type_name()).into()
    }
}

impl<'eval, 'a: 'eval, T: AnyValue<'a>> AnyValue<'eval> for Vec<T> {
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

    fn static_ty() -> EntityRoutePtr {
        lazy_entity_route!({
            make_route(RootIdentifier::Vec.into(), thin_vec![T::static_ty().into()])
        })
    }

    fn ty(&self) -> EntityRoutePtr {
        Self::static_ty()
    }
}
