use std::borrow::Cow;

use entity_route::EntityRoutePtr;
use print_utils::{msg_once, p};
use serde::Serialize;
use word::{CustomIdentifier, IdentPairDict};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualVec<'eval> {
    ty: EntityRoutePtr,
    data: Vec<MemberValue<'eval>>,
}

impl<'eval> VirtualVec<'eval> {
    pub fn new(ty: EntityRoutePtr) -> Self {
        Self { ty, data: vec![] }
    }
}

impl<'eval> std::ops::Deref for VirtualVec<'eval> {
    type Target = Vec<MemberValue<'eval>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'eval> std::ops::DerefMut for VirtualVec<'eval> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'eval> HasStaticTypeInfo for VirtualVec<'eval> {
    type StaticSelf = VirtualVec<'static>;

    fn static_type_name() -> Cow<'static, str> {
        "[]Any".into()
    }
}

impl<'eval, 'eval0: 'eval> AnyValue<'eval> for VirtualVec<'eval0> {
    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        todo!()
    }

    fn ty(&self) -> EntityRoutePtr {
        self.ty
    }
}
