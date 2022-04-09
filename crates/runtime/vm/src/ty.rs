use word::CustomIdentifier;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualTy<'eval> {
    Struct {
        field_vars: Vec<StructMembValue<'eval>>,
    },
}

impl<'stack, 'eval: 'stack> VirtualTy<'eval> {
    pub fn new_struct(
        mut inputs: Vec<StackValue<'stack, 'eval>>,
        field_var_contracts: &[MembAccessContract],
    ) -> Self {
        let mut field_vars = vec![];
        for i in 0..inputs.len() {
            field_vars.push(inputs[i].bind_move().into_struct_memb());
        }
        Self::Struct { field_vars }
    }

    pub fn eval_field_var(&self, field_idx: usize) -> &StructMembValue<'eval> {
        match self {
            VirtualTy::Struct { field_vars } => &field_vars[field_idx],
        }
    }

    pub fn take_field_var(&mut self, field_idx: usize) -> StackValue<'stack, 'eval> {
        match self {
            VirtualTy::Struct { field_vars } => {
                std::mem::replace(&mut field_vars[field_idx], StructMembValue::Moved).into_stack()
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
                    StructMembValue::Primitive(value) => StackValue::Primitive(value),
                    StructMembValue::Boxed(_) => todo!(),
                    StructMembValue::GlobalPure(_) => todo!(),
                    StructMembValue::GlobalRef(_) => todo!(),
                    StructMembValue::Moved => todo!(),
                },
            },
            EagerContract::BorrowMut => todo!(),
            EagerContract::TakeMut => todo!(),
            EagerContract::Exec => todo!(),
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
            EagerContract::BorrowMut => match self {
                VirtualTy::Struct { field_vars } => {
                    let field_var_value = &mut field_vars[field_idx];
                    let ptr: *mut dyn AnyValueDyn = match field_var_value {
                        StructMembValue::Primitive(ref mut value) => value.any_mut(),
                        StructMembValue::Boxed(_) => todo!(),
                        StructMembValue::GlobalPure(_) => todo!(),
                        StructMembValue::GlobalRef(_) => todo!(),
                        StructMembValue::Moved => todo!(),
                    };
                    StackValue::MutLocalRef {
                        value: unsafe { &mut *ptr },
                        owner,
                        gen: (),
                    }
                }
            },
            EagerContract::TakeMut => todo!(),
            EagerContract::Exec => todo!(),
            EagerContract::LetInit => todo!(),
            EagerContract::VarInit => todo!(),
            EagerContract::Return => todo!(),
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

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        todo!()
    }
}

impl<'stack, 'eval: 'stack> Into<StackValue<'stack, 'eval>> for VirtualTy<'eval> {
    fn into(self) -> StackValue<'stack, 'eval> {
        StackValue::Boxed(BoxedValue::new(self))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualMembVar<'eval> {
    ident: CustomIdentifier,
    value: EvalValue<'eval>,
}
