use std::borrow::Cow;

use husky_entity_route::EntityRoutePtr;
use husky_print_utils::{msg_once, p};
use husky_word::{CustomIdentifier, IdentPairDict};
use serde::Serialize;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VirtualVec<'eval> {
    data: Vec<__Register<'eval>>,
}

impl<'eval> VirtualVec<'eval> {
    pub fn new(data: Vec<__Register<'eval>>) -> Self {
        Self { data }
    }
}

impl<'eval> std::ops::Deref for VirtualVec<'eval> {
    type Target = Vec<__Register<'eval>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'eval> std::ops::DerefMut for VirtualVec<'eval> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'eval> __StaticInfo for VirtualVec<'eval> {
    type __StaticSelf = VirtualVec<'static>;

    fn __static_typename() -> Cow<'static, str> {
        "[]Any".into()
    }
}

impl<'eval> __Registrable<'eval> for VirtualVec<'eval> {
    unsafe fn __to_register(self) -> __Register<'eval> {
        __Register::new_box(self, &__VIRTUAL_VEC_VTABLE)
    }
}
