use print_utils::msg_once;
use serde::Serialize;
use word::{CustomIdentifier, IdentPairDict};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualTy<'eval> {
    Struct {
        fields: IdentPairDict<MemberValue<'eval>>,
    },
}

impl<'stack, 'eval: 'stack> VirtualTy<'eval> {
    pub fn new_struct(
        mut inputs: Vec<StackValue<'stack, 'eval>>,
        field_liasons: &[(CustomIdentifier, FieldLiason)],
    ) -> Self {
        let mut fields = IdentPairDict::<MemberValue<'eval>>::default();
        for i in 0..inputs.len() {
            let (ident, liason) = field_liasons[i];
            msg_once!("check liason");
            fields.insert_new((ident, inputs[i].bind_move().into_member()));
        }
        VirtualTy::Struct { fields }
    }

    pub fn eval_field(&self, field_idx: usize) -> &MemberValue<'eval> {
        match self {
            VirtualTy::Struct { fields } => &fields.data()[field_idx].1,
        }
    }

    pub fn take_field(&mut self, field_idx: usize) -> StackValue<'stack, 'eval> {
        match self {
            VirtualTy::Struct { fields } => {
                std::mem::replace(&mut fields.data_mut()[field_idx].1, MemberValue::Moved)
                    .into_stack()
            }
        }
    }

    pub fn eager_field(
        &self,
        field_idx: usize,
        field_access_contract: EagerContract,
    ) -> StackValue<'stack, 'eval> {
        match field_access_contract {
            EagerContract::Pure => match self {
                VirtualTy::Struct { fields } => match fields.data()[field_idx].1 {
                    MemberValue::Primitive(_) => todo!(),
                    MemberValue::Boxed(ref value) => {
                        let ptr = value.any_ptr();
                        StackValue::LocalRef(unsafe { &*ptr })
                    }
                    MemberValue::GlobalPure(_) => todo!(),
                    MemberValue::GlobalRef(_) => todo!(),
                    MemberValue::Moved => todo!(),
                },
            },
            EagerContract::GlobalRef => todo!(),
            EagerContract::Move => todo!(),
            EagerContract::LetInit => todo!(),
            EagerContract::VarInit => todo!(),
            EagerContract::Return => match self {
                VirtualTy::Struct { fields } => match fields.data()[field_idx].1 {
                    MemberValue::Primitive(value) => StackValue::Copyable(value),
                    MemberValue::Boxed(_) => todo!(),
                    MemberValue::GlobalPure(_) => todo!(),
                    MemberValue::GlobalRef(_) => todo!(),
                    MemberValue::Moved => todo!(),
                },
            },
            EagerContract::RefMut => todo!(),
            EagerContract::MoveMut => todo!(),
            EagerContract::Exec => todo!(),
            EagerContract::UseMemberForLetInit => todo!(),
            EagerContract::UseMemberForVarInit => todo!(),
        }
    }

    pub fn field_mut(
        &mut self,
        field_idx: usize,
        contract: EagerContract,
        owner: StackIdx,
    ) -> StackValue<'stack, 'eval> {
        match contract {
            EagerContract::Pure => todo!(),
            EagerContract::GlobalRef => todo!(),
            EagerContract::Move => todo!(),
            EagerContract::RefMut => match self {
                VirtualTy::Struct { fields } => {
                    let field_value = &mut fields.data_mut()[field_idx].1;
                    let ptr: *mut dyn AnyValueDyn = match field_value {
                        MemberValue::Primitive(ref mut value) => value.any_mut(),
                        MemberValue::Boxed(_) => todo!(),
                        MemberValue::GlobalPure(_) => todo!(),
                        MemberValue::GlobalRef(_) => todo!(),
                        MemberValue::Moved => todo!(),
                    };
                    StackValue::RefMut {
                        value: unsafe { &mut *ptr },
                        owner,
                        gen: (),
                    }
                }
            },
            EagerContract::MoveMut => todo!(),
            EagerContract::Exec => todo!(),
            EagerContract::LetInit => todo!(),
            EagerContract::VarInit => todo!(),
            EagerContract::Return => todo!(),
            EagerContract::UseMemberForLetInit => todo!(),
            EagerContract::UseMemberForVarInit => todo!(),
        }
    }
}

impl<'eval> Serialize for VirtualTy<'eval> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'eval> AnyValue<'eval> for VirtualTy<'eval> {
    fn static_type_id() -> StaticTypeId {
        HuskyBuiltinStaticTypeId::VirtualTy.into()
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    fn print_short(&self) -> String {
        "VirtualTy(todo)".to_string()
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        match self {
            VirtualTy::Struct { fields } => serde_json::value::Value::Object(
                fields
                    .iter()
                    .map(|(ident, value)| (ident.as_str().to_string(), value.to_json_value()))
                    .collect(),
            ),
        }
    }
}

impl<'stack, 'eval: 'stack> Into<StackValue<'stack, 'eval>> for VirtualTy<'eval> {
    fn into(self) -> StackValue<'stack, 'eval> {
        StackValue::Owned(OwnedValue::new(self))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualMembVar<'eval> {
    ident: CustomIdentifier,
    value: EvalValue<'eval>,
}
