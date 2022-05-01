use crate::*;

#[derive(Debug)]
pub struct EnumLiteralValue(Box<dyn EnumLiteralValueDyn>);

impl<'eval> AnyValue<'eval> for EntityRoutePtr {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<EntityRoutePtr>().into()
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "ScopePtr".into()
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        todo!()
    }
}

impl EnumLiteralValueDyn for EntityRoutePtr {
    fn clone_as_boxed(&self) -> Box<dyn EnumLiteralValueDyn> {
        Box::new(self.clone())
    }

    fn eq_dyn(&self, other: &dyn EnumLiteralValueDyn) -> bool {
        self.eq(other.upcast_any().downcast_ref::<EntityRoutePtr>())
    }
}

impl EnumLiteralValue {
    pub fn interpreted<T>(scope: T) -> Self
    where
        T: EnumLiteralValueDyn,
    {
        Self(Box::new(scope))
    }

    pub fn clone_any<'eval>(&self) -> BoxedValue<'eval> {
        BoxedValue {
            inner: self.0.clone_any(),
        }
    }
}

pub trait EnumLiteralValueDyn: for<'a> AnyValueDyn<'a> + 'static {
    fn clone_as_boxed(&self) -> Box<dyn EnumLiteralValueDyn>;
    fn eq_dyn(&self, other: &dyn EnumLiteralValueDyn) -> bool;
}

impl PartialEq for EnumLiteralValue {
    fn eq(&self, other: &EnumLiteralValue) -> bool {
        self.0.eq_dyn(&*other.0)
    }
}

impl Eq for EnumLiteralValue {}

impl Clone for EnumLiteralValue {
    fn clone(&self) -> Self {
        Self(self.0.clone_as_boxed())
    }
}
