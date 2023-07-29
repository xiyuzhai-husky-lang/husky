use super::*;
use husky_coword::{Ident, IdentPairMap};
use husky_ethereal_term::EtherealTerm;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct DeprecatedVirtualStruct {
    ty: EtherealTerm,
    fields: IdentPairMap<__RegularValue>,
}

impl DeprecatedVirtualStruct {
    pub fn new_struct(
        ty: EtherealTerm,
        arguments: impl Iterator<Item = __RegularValue>,
        field_liasons: &[Ident],
    ) -> Self {
        let mut fields = IdentPairMap::<__RegularValue>::default();
        for (ident, argument) in std::iter::zip(field_liasons.iter(), arguments) {
            fields.insert_new((*ident, argument)).unwrap();
        }
        DeprecatedVirtualStruct { ty, fields }
    }

    pub fn eval_field(&self, field_idx: u8) -> &__RegularValue {
        &self.fields.data()[field_idx as usize].1
    }

    pub fn take_field(&mut self, field_idx: u8) -> __RegularValue {
        self.fields.data_mut()[field_idx as usize].1.register_move()
    }

    pub fn bind_field_copy(&self, field_idx: u8) -> __RegularValue {
        self.fields.data()[field_idx as usize].1.bind_copy()
    }

    pub fn bind_field_temp_ref(&self, field_idx: u8) -> __RegularValue {
        self.fields.data()[field_idx as usize].1.bind_temp_ref()
    }

    pub fn bind_field_leash(&'static self, field_idx: u8) -> __RegularValue {
        self.fields.data()[field_idx as usize].1.eval_bind_leash()
    }

    pub fn bind_field_mut(&mut self, field_idx: u8) -> __RegularValue {
        self.fields.data_mut()[field_idx as usize].1.bind_temp_mut()
    }
}

impl Serialize for DeprecatedVirtualStruct {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl __StaticInfo for DeprecatedVirtualStruct {
    type __StaticSelf = DeprecatedVirtualStruct;

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

// impl __Registrable for DeprecatedVirtualStruct {
//     unsafe fn __to_register(self) -> __RegularValue {
//         __RegularValue::new_box(self, &__DEPRECATED_VIRTUAL_STRUCT_VTABLE)
//     }
// }
