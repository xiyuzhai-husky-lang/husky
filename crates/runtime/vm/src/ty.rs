use word::CustomIdentifier;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualTy<'eval> {
    Struct {
        memb_vars: Vec<VirtualMembVar<'eval>>,
    },
}

impl<'stack, 'eval: 'stack> VirtualTy<'eval> {
    pub fn new_struct(
        mut inputs: Vec<StackValue<'stack, 'eval>>,
        memb_var_sigs: &[(CustomIdentifier, MembAccessContract)],
    ) -> Self {
        let mut memb_vars = vec![];
        for i in 0..inputs.len() {
            memb_vars.push(VirtualMembVar {
                ident: memb_var_sigs[i].0,
                value: inputs[i].bind_move().into_eval(),
            });
        }
        Self::Struct { memb_vars }
    }

    pub fn eval_memb_var(&self, ident: CustomIdentifier) -> &EvalValue<'eval> {
        match self {
            VirtualTy::Struct { memb_vars } => {
                &memb_vars
                    .iter()
                    .find(|memb_var| memb_var.ident == ident)
                    .unwrap()
                    .value
            }
        }
    }

    pub fn take_memb_var(self, ident: CustomIdentifier) -> StackValue<'stack, 'eval> {
        match self {
            VirtualTy::Struct { memb_vars } => memb_vars
                .into_iter()
                .find(|memb_var| memb_var.ident == ident)
                .unwrap()
                .value
                .into_stack()
                .unwrap(),
        }
    }

    pub fn eager_memb_var(
        &self,
        ident: CustomIdentifier,
        contract: EagerContract,
    ) -> StackValue<'stack, 'eval> {
        match contract {
            EagerContract::Pure => todo!(),
            EagerContract::GlobalRef => todo!(),
            EagerContract::Move => todo!(),
            EagerContract::LetInit => todo!(),
            EagerContract::VarInit => todo!(),
            EagerContract::Return => match self {
                VirtualTy::Struct { memb_vars } => match memb_vars
                    .iter()
                    .find(|memb_var| memb_var.ident == ident)
                    .unwrap()
                    .value
                {
                    EvalValue::Primitive(value) => StackValue::Primitive(value),
                    EvalValue::Boxed(_) => todo!(),
                    EvalValue::GlobalPure(_) => todo!(),
                    EvalValue::GlobalRef(_) => todo!(),
                    EvalValue::Undefined => todo!(),
                },
            },
            EagerContract::BorrowMut => todo!(),
            EagerContract::TakeMut => todo!(),
            EagerContract::Exec => todo!(),
        }
    }

    pub fn memb_var_mut(
        &mut self,
        ident: CustomIdentifier,
        contract: EagerContract,
        owner: StackIdx,
    ) -> StackValue<'stack, 'eval> {
        match contract {
            EagerContract::Pure => todo!(),
            EagerContract::GlobalRef => todo!(),
            EagerContract::Move => todo!(),
            EagerContract::BorrowMut => match self {
                VirtualTy::Struct { memb_vars } => {
                    let memb_var_value = &mut memb_vars
                        .iter_mut()
                        .find(|memb_var| memb_var.ident == ident)
                        .unwrap()
                        .value;
                    let ptr: *mut dyn AnyValueDyn = match memb_var_value {
                        EvalValue::Primitive(ref mut value) => value.any_mut(),
                        EvalValue::Boxed(_) => todo!(),
                        EvalValue::GlobalPure(_) => todo!(),
                        EvalValue::GlobalRef(_) => todo!(),
                        EvalValue::Undefined => todo!(),
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
