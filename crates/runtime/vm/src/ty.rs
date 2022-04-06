use word::CustomIdentifier;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualTy<'eval> {
    Struct {
        memb_vars: Vec<StructMembValue<'eval>>,
    },
}

impl<'stack, 'eval: 'stack> VirtualTy<'eval> {
    pub fn new_struct(
        mut inputs: Vec<StackValue<'stack, 'eval>>,
        memb_var_contracts: &[MembAccessContract],
    ) -> Self {
        let mut memb_vars = vec![];
        for i in 0..inputs.len() {
            memb_vars.push(inputs[i].bind_move().into_struct_memb());
        }
        Self::Struct { memb_vars }
    }

    pub fn eval_memb_var(&self, memb_idx: usize) -> &StructMembValue<'eval> {
        match self {
            VirtualTy::Struct { memb_vars } => &memb_vars[memb_idx],
        }
    }

    pub fn take_memb_var(&mut self, memb_idx: usize) -> StackValue<'stack, 'eval> {
        match self {
            VirtualTy::Struct { memb_vars } => {
                std::mem::replace(&mut memb_vars[memb_idx], StructMembValue::Moved).into_stack()
            }
        }
    }

    pub fn eager_memb_var(
        &self,
        memb_idx: usize,
        contract: EagerContract,
    ) -> StackValue<'stack, 'eval> {
        match contract {
            EagerContract::Pure => todo!(),
            EagerContract::GlobalRef => todo!(),
            EagerContract::Move => todo!(),
            EagerContract::LetInit => todo!(),
            EagerContract::VarInit => todo!(),
            EagerContract::Return => match self {
                VirtualTy::Struct { memb_vars } => match memb_vars[memb_idx] {
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

    pub fn memb_var_mut(
        &mut self,
        memb_idx: usize,
        contract: EagerContract,
        owner: StackIdx,
    ) -> StackValue<'stack, 'eval> {
        match contract {
            EagerContract::Pure => todo!(),
            EagerContract::GlobalRef => todo!(),
            EagerContract::Move => todo!(),
            EagerContract::BorrowMut => match self {
                VirtualTy::Struct { memb_vars } => {
                    let memb_var_value = &mut memb_vars[memb_idx];
                    let ptr: *mut dyn AnyValueDyn = match memb_var_value {
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
