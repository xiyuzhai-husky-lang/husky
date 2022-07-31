use cyclic_slice::CyclicSlice;
use husky_entity_route::EntityRoutePtr;
use husky_print_utils::{msg_once, p};
use husky_word::{CustomIdentifier, IdentPairDict};
use serde::Serialize;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VirtualCyclicSlice<'eval> {
    pub data: CyclicSlice<'eval, __Register<'eval>>,
}

impl<'eval> std::ops::Deref for VirtualCyclicSlice<'eval> {
    type Target = CyclicSlice<'eval, __Register<'eval>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'eval> std::ops::DerefMut for VirtualCyclicSlice<'eval> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'eval> __StaticInfo for VirtualCyclicSlice<'eval> {
    type __StaticSelf = VirtualCyclicSlice<'static>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "CyclicSlice<Any>".into()
    }
}

impl<'eval> __Registrable<'eval> for VirtualCyclicSlice<'eval> {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}
