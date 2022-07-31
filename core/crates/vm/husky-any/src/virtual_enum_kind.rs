use crate::*;
use husky_entity_route::EntityRoutePtr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VirtualEnum {
    pub kind_idx: u8,
    pub route: EntityRoutePtr,
}

impl __StaticInfo for VirtualEnum {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval> __Registrable<'eval> for VirtualEnum {
    unsafe fn __to_register(self) -> __Register<'eval>
    where
        Self: 'eval,
    {
        __Register::new_box(self, &__VIRTUAL_ENUM_VTABLE)
    }
}
