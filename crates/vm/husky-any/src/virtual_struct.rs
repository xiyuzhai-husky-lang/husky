use super::*;
use husky_term::Term;
use husky_word::{Ident, IdentPairMap};
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct DeprecatedVirtualStruct<'eval> {
    ty: Term,
    fields: IdentPairMap<__Register<'eval>>,
}

impl<'eval> DeprecatedVirtualStruct<'eval> {
    pub fn new_struct(
        ty: Term,
        arguments: impl Iterator<Item = __Register<'eval>>,
        field_liasons: &[Ident],
    ) -> Self {
        let mut fields = IdentPairMap::<__Register<'eval>>::default();
        for (ident, argument) in std::iter::zip(field_liasons.iter(), arguments) {
            fields.insert_new((*ident, argument)).unwrap();
        }
        DeprecatedVirtualStruct { ty, fields }
    }

    pub fn eval_field(&self, field_idx: u8) -> &__Register<'eval> {
        &self.fields.data()[field_idx as usize].1
    }

    pub fn take_field(&mut self, field_idx: u8) -> __Register<'eval> {
        self.fields.data_mut()[field_idx as usize].1.register_move()
    }

    pub fn bind_field_copy(&self, field_idx: u8) -> __Register<'eval> {
        self.fields.data()[field_idx as usize].1.bind_copy()
    }

    pub fn bind_field_temp_ref(&self, field_idx: u8) -> __Register<'eval> {
        self.fields.data()[field_idx as usize].1.bind_temp_ref()
    }

    pub fn bind_field_eval_ref(&'eval self, field_idx: u8) -> __Register<'eval> {
        self.fields.data()[field_idx as usize]
            .1
            .eval_bind_eval_ref()
    }

    pub fn bind_field_mut(&mut self, field_idx: u8) -> __Register<'eval> {
        self.fields.data_mut()[field_idx as usize].1.bind_temp_mut()
    }
}

impl<'eval> Serialize for DeprecatedVirtualStruct<'eval> {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'eval> __StaticInfo for DeprecatedVirtualStruct<'eval> {
    type __StaticSelf = DeprecatedVirtualStruct<'static>;

    fn __static_typename() -> Cow<'static, str> {
        "AnyStruct".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        std::mem::transmute(self)
    }
}

impl<'eval> __Registrable<'eval> for DeprecatedVirtualStruct<'eval> {
    unsafe fn __to_register(self) -> __Register<'eval> {
        __Register::new_box(self, &__DEPRECATED_VIRTUAL_STRUCT_VTABLE)
    }
}
