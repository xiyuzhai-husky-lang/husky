use std::borrow::Cow;

use husky_entity_route::EntityRoutePtr;
use husky_print_utils::{msg_once, p};
use husky_word::{CustomIdentifier, IdentPairDict};
use serde::Serialize;

use super::*;

extern "C" {
    pub static __VIRTUAL_VEC_VTABLE: __RegisterVTable;
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericVec<'eval> {
    data: Vec<__Register<'eval>>,
}

impl<'eval> GenericVec<'eval> {
    pub fn new(data: Vec<__Register<'eval>>) -> Self {
        Self { data }
    }
}

impl<'eval> std::ops::Deref for GenericVec<'eval> {
    type Target = Vec<__Register<'eval>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'eval> std::ops::DerefMut for GenericVec<'eval> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'eval> __StaticInfo for GenericVec<'eval> {
    type __StaticSelf = GenericVec<'static>;

    fn __static_type_name() -> Cow<'static, str> {
        "[]Any".into()
    }
}

impl<'eval> __Registrable for GenericVec<'eval> {
    unsafe fn __to_register__<'eval0>(self) -> __Register<'eval0> {
        __Register::new_box(self, &__VIRTUAL_VEC_VTABLE)
    }

    fn __copy__(&self) -> Self {
        panic!()
    }
}
