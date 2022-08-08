use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct __VirtualEnum {
    pub kind_idx: i32,
    // pub route: EntityRoutePtr,
}

impl __StaticInfo for __VirtualEnum {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval> __Registrable<'eval> for __VirtualEnum {
    unsafe fn __to_register(self) -> __Register<'eval>
    where
        Self: 'eval,
    {
        __Register::new_box(self, &__VIRTUAL_ENUM_VTABLE)
    }
}
