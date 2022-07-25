use cyclic_slice::CyclicSlice;
use husky_entity_route::EntityRoutePtr;
use husky_print_utils::{msg_once, p};
use serde::Serialize;
use word::{CustomIdentifier, IdentPairDict};

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GenericCyclicSlice<'eval> {
    pub data: CyclicSlice<'eval, __Register<'eval>>,
}

impl<'eval> std::ops::Deref for GenericCyclicSlice<'eval> {
    type Target = CyclicSlice<'eval, __Register<'eval>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'eval> std::ops::DerefMut for GenericCyclicSlice<'eval> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'eval> __StaticInfo for GenericCyclicSlice<'eval> {
    type __StaticSelf = GenericCyclicSlice<'static>;

    fn __static_type_name__() -> std::borrow::Cow<'static, str> {
        "CyclicSlice<Any>".into()
    }
}

impl<'eval> __Registrable for GenericCyclicSlice<'eval> {
    unsafe fn __to_register__<'eval0>(self) -> __Register<'eval0> {
        todo!()
    }
}
