use super::*;
use std::fmt::Write;
use word::IdentPairDict;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualStruct<'eval> {
    ty: EntityRoutePtr,
    fields: IdentPairDict<MemberValue<'eval>>,
}

impl<'temp, 'eval: 'temp> VirtualStruct<'eval> {
    pub fn new_struct(
        ty: EntityRoutePtr,
        mut arguments: impl Iterator<Item = __TempValue<'temp, 'eval>>,
        field_liasons: &[CustomIdentifier],
    ) -> Self {
        let mut fields = IdentPairDict::<MemberValue<'eval>>::default();
        for (ident, mut argument) in std::iter::zip(field_liasons.iter(), arguments) {
            fields.insert_new((*ident, argument.into_member()));
        }
        VirtualStruct { ty, fields }
    }

    pub fn eval_field(&self, field_idx: usize) -> &MemberValue<'eval> {
        &self.fields.data()[field_idx].1
    }

    pub fn take_field(&mut self, field_idx: usize) -> __TempValue<'temp, 'eval> {
        std::mem::replace(&mut self.fields.data_mut()[field_idx].1, MemberValue::Moved).into_stack()
    }

    pub fn access_field(
        &self,
        field_idx: usize,
        field_binding: Binding,
    ) -> __TempValue<'temp, 'eval> {
        self.fields.data()[field_idx].1.bind(field_binding)
    }

    pub fn field_mut(
        &mut self,
        field_idx: usize,
        field_binding: Binding,
        owner: VMStackIdx,
    ) -> __TempValue<'temp, 'eval> {
        match field_binding {
            Binding::EvalRef => todo!(),
            Binding::TempRef => todo!(),
            Binding::TempRefMut => {
                let field_value = &mut self.fields.data_mut()[field_idx].1;
                let ptr: *mut dyn AnyValueDyn = match field_value {
                    MemberValue::Copyable(ref mut value) => value.any_mut(),
                    MemberValue::Boxed(_) => todo!(),
                    MemberValue::GlobalPure(_) => todo!(),
                    MemberValue::EvalRef(_) => todo!(),
                    MemberValue::Moved => todo!(),
                };
                __TempValue::TempRefMutEval {
                    value: unsafe { &mut *ptr },
                    owner,
                    gen: (),
                }
            }
            Binding::Move => todo!(),
            Binding::Copy => todo!(),
        }
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

impl<'eval> HasStaticTypeInfo for VirtualStruct<'eval> {
    type StaticSelf = VirtualStruct<'static>;

    fn static_type_name() -> Cow<'static, str> {
        "AnyStruct".into()
    }
}

impl<'eval, 'a> AnyValue<'eval> for VirtualStruct<'a>
where
    'a: 'eval,
{
    fn print_short(&self) -> String {
        print_sequence(
            "{ ",
            self.fields.iter(),
            &|(key, value)| format!("{}: {}", key.0, value.any_ref().print_short()),
            " }",
            40,
        )
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::Value::Object(
            self.fields
                .iter()
                .map(|(ident, value)| {
                    (
                        ident.as_str().to_string(),
                        value.any_ref().to_json_value_dyn(),
                    )
                })
                .collect(),
        )
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn ty(&self) -> EntityRoutePtr {
        self.ty
    }
}

impl<'temp, 'eval: 'temp> Into<__TempValue<'temp, 'eval>> for VirtualStruct<'eval> {
    fn into(self) -> __TempValue<'temp, 'eval> {
        __TempValue::OwnedEval(OwnedValue::new(self))
    }
}
