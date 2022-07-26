use super::*;
use husky_word::{CustomIdentifier, IdentPairDict};
use serde::Serialize;
use std::{borrow::Cow, fmt::Write};

#[derive(Debug, Clone, PartialEq)]
pub struct VirtualStruct<'eval> {
    ty: EntityRoutePtr,
    fields: IdentPairDict<__Register<'eval>>,
}

impl<'eval> VirtualStruct<'eval> {
    pub fn new_struct(
        ty: EntityRoutePtr,
        mut arguments: impl Iterator<Item = __Register<'eval>>,
        field_liasons: &[CustomIdentifier],
    ) -> Self {
        let mut fields = IdentPairDict::<__Register<'eval>>::default();
        for (ident, mut argument) in std::iter::zip(field_liasons.iter(), arguments) {
            fields.insert_new((*ident, argument));
        }
        VirtualStruct { ty, fields }
    }

    pub fn eval_field(&self, field_idx: usize) -> &__Register<'eval> {
        &self.fields.data()[field_idx].1
    }

    pub fn take_field(&mut self, field_idx: usize) -> __Register<'eval> {
        self.fields.data_mut()[field_idx].1.register_move()
    }

    pub fn access_field(&self, field_idx: usize) -> __Register<'eval> {
        todo!()
        // self.fields.data()[field_idx].1.bind(field_binding)
    }

    pub fn field_mut(&mut self, field_idx: usize) -> __Register<'eval> {
        todo!()
        // match field_binding {
        //     Binding::EvalRef => todo!(),
        //     Binding::TempRef => todo!(),
        //     Binding::TempRefMut => {
        //         let field_value = &mut self.fields.data_mut()[field_idx].1;
        //         let ptr: *mut dyn __AnyValueDyn = match field_value {
        //             MemberValue::Copyable(ref mut value) => value.any_mut(),
        //             MemberValue::Boxed(_) => todo!(),
        //             MemberValue::GlobalPure(_) => todo!(),
        //             MemberValue::EvalRef(_) => todo!(),
        //             MemberValue::Moved => todo!(),
        //         };
        //         __TempValue::TempRefMutEval {
        //             value: unsafe { &mut *ptr },
        //             owner,
        //             gen: (),
        //         }
        //     }
        //     Binding::Move => todo!(),
        //     Binding::Copy => todo!(),
        // }
    }
}

impl<'eval> Serialize for VirtualStruct<'eval> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'eval> __StaticInfo for VirtualStruct<'eval> {
    type __StaticSelf = VirtualStruct<'static>;

    fn __static_type_name() -> Cow<'static, str> {
        "AnyStruct".into()
    }
}

impl<'eval> __Registrable for VirtualStruct<'eval> {
    unsafe fn __to_register__<'eval0>(self) -> __Register<'eval0> {
        todo!()
    }

    fn __copy__(&self) -> Self {
        panic!()
    }
}
