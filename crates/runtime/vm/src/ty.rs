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

impl<'vm, 'eval: 'vm> VirtualTy<'eval> {
    pub fn new_struct(
        mut arguments: impl Iterator<Item = VMValue<'vm, 'eval>>,
        field_liasons: &[(CustomIdentifier, FieldLiason)],
    ) -> Self {
        let mut fields = IdentPairDict::<MemberValue<'eval>>::default();
        for (i, mut argument) in arguments.enumerate() {
            let (ident, liason) = field_liasons[i];
            msg_once!("check liason");
            fields.insert_new((ident, argument.bind_move().into_member()));
        }
        VirtualTy::Struct { fields }
    }

    pub fn eval_field(&self, field_idx: usize) -> &MemberValue<'eval> {
        match self {
            VirtualTy::Struct { fields } => &fields.data()[field_idx].1,
        }
    }

    pub fn take_field(&mut self, field_idx: usize) -> VMValue<'vm, 'eval> {
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
    ) -> VMValue<'vm, 'eval> {
        match field_access_contract {
            EagerContract::Pure | EagerContract::UseForLetInit => match self {
                VirtualTy::Struct { fields } => match fields.data()[field_idx].1 {
                    MemberValue::Copyable(value) => VMValue::Copyable(value),
                    MemberValue::Boxed(ref value) => {
                        let ptr = value.any_ptr();
                        VMValue::FullyOwnedRef(unsafe { &*ptr })
                    }
                    MemberValue::GlobalPure(_) => todo!(),
                    MemberValue::GlobalRef(_) => todo!(),
                    MemberValue::Moved => todo!(),
                },
            },

            EagerContract::Move => todo!(),
            EagerContract::UseForVarInit => todo!(),
            EagerContract::Return => match self {
                VirtualTy::Struct { fields } => match fields.data()[field_idx].1 {
                    MemberValue::Copyable(value) => VMValue::Copyable(value),
                    MemberValue::Boxed(_) => todo!(),
                    MemberValue::GlobalPure(_) => todo!(),
                    MemberValue::GlobalRef(_) => todo!(),
                    MemberValue::Moved => todo!(),
                },
            },
            EagerContract::RefMut => todo!(),
            EagerContract::MoveMut => todo!(),
            EagerContract::Exec => todo!(),
            EagerContract::UseMemberForLetInit => match self {
                VirtualTy::Struct { fields } => match fields.data()[field_idx].1 {
                    MemberValue::Copyable(_) => todo!(),
                    MemberValue::Boxed(ref value) => {
                        let ptr = value.any_ptr();
                        VMValue::FullyOwnedRef(unsafe { &*ptr })
                    }
                    MemberValue::GlobalPure(_) => todo!(),
                    MemberValue::GlobalRef(_) => todo!(),
                    MemberValue::Moved => todo!(),
                },
            },
            EagerContract::UseMemberForVarInit => todo!(),
            EagerContract::UseForAssignRvalue => todo!(),
        }
    }

    pub fn field_mut(
        &mut self,
        field_idx: usize,
        contract: EagerContract,
        owner: VMStackIdx,
    ) -> VMValue<'vm, 'eval> {
        match contract {
            EagerContract::Pure => todo!(),

            EagerContract::Move => todo!(),
            EagerContract::RefMut => match self {
                VirtualTy::Struct { fields } => {
                    let field_value = &mut fields.data_mut()[field_idx].1;
                    let ptr: *mut dyn AnyValueDyn = match field_value {
                        MemberValue::Copyable(ref mut value) => value.any_mut(),
                        MemberValue::Boxed(_) => todo!(),
                        MemberValue::GlobalPure(_) => todo!(),
                        MemberValue::GlobalRef(_) => todo!(),
                        MemberValue::Moved => todo!(),
                    };
                    VMValue::CopyableOrFullyOwnedMut {
                        value: unsafe { &mut *ptr },
                        owner,
                        gen: (),
                    }
                }
            },
            EagerContract::MoveMut => todo!(),
            EagerContract::Exec => todo!(),
            EagerContract::UseForLetInit => todo!(),
            EagerContract::UseForVarInit => todo!(),
            EagerContract::Return => todo!(),
            EagerContract::UseMemberForLetInit => todo!(),
            EagerContract::UseMemberForVarInit => todo!(),
            EagerContract::UseForAssignRvalue => todo!(),
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
        "VirtualTy".into()
    }

    fn print_short(&self) -> String {
        "VirtualTy(todo)".to_string()
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        match self {
            VirtualTy::Struct { fields } => serde_json::value::Value::Object(
                fields
                    .iter()
                    .map(|(ident, value)| (ident.as_str().to_string(), value.get_json_value()))
                    .collect(),
            ),
        }
    }
}

impl<'vm, 'eval: 'vm> Into<VMValue<'vm, 'eval>> for VirtualTy<'eval> {
    fn into(self) -> VMValue<'vm, 'eval> {
        VMValue::FullyOwned(OwnedValue::new(self))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualMembVar<'eval> {
    ident: CustomIdentifier,
    value: EvalValue<'eval>,
}
