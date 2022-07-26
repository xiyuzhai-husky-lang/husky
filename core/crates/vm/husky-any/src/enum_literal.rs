use crate::*;
use husky_entity_route::EntityRoutePtr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EnumKindValue {
    pub kind_idx: u8,
    pub route: EntityRoutePtr,
}

impl __StaticInfo for EnumKindValue {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl __Registrable for EnumKindValue {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval>
    where
        Self: 'eval,
    {
        todo!()
    }

    fn __copy__(&self) -> Self {
        *self
    }
}
