use word::CustomIdentifier;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualTy<'eval> {
    Struct { field_vars: Vec<MemberValue<'eval>> },
}

impl<'stack, 'eval: 'stack> VirtualTy<'eval> {
    pub fn new_struct(
        mut inputs: Vec<StackValue<'stack, 'eval>>,
        field_var_contracts: &[FieldContract],
    ) -> Self {
        let mut field_vars = vec![];
        for i in 0..inputs.len() {
            field_vars.push(inputs[i].bind_move().into_member());
        }
        Self::Struct { field_vars }
    }

    pub fn eval_field_var(&self, field_idx: usize) -> &MemberValue<'eval> {
        match self {
            VirtualTy::Struct { field_vars } => &field_vars[field_idx],
        }
    }

    pub fn take_field_var(&mut self, field_idx: usize) -> StackValue<'stack, 'eval> {
        match self {
            VirtualTy::Struct { field_vars } => {
                std::mem::replace(&mut field_vars[field_idx], MemberValue::Moved).into_stack()
            }
        }
    }

    pub fn eager_field_var(
        &self,
        field_idx: usize,
        contract: EagerContract,
    ) -> StackValue<'stack, 'eval> {
        match contract {
            EagerContract::Pure => todo!(),
            EagerContract::GlobalRef => todo!(),
            EagerContract::Move => todo!(),
            EagerContract::LetInit => todo!(),
            EagerContract::VarInit => todo!(),
            EagerContract::Return => match self {
                VirtualTy::Struct { field_vars } => match field_vars[field_idx] {
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

    pub fn field_var_mut(
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
                VirtualTy::Struct { field_vars } => {
                    let field_var_value = &mut field_vars[field_idx];
                    let ptr: *mut dyn AnyValueDyn = match field_var_value {
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

impl<'eval> AnyValue<'eval> for VirtualTy<'eval> {
    fn static_type_id() -> StaticTypeId {
        HuskyBuiltinStaticTypeId::VirtualTy.into()
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
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
