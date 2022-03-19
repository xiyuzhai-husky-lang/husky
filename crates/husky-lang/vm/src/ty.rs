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
        memb_var_sigs: &[(CustomIdentifier, MembVarContract)],
    ) -> Self {
        let mut memb_vars = vec![];
        for i in 0..inputs.len() {
            memb_vars.push(VirtualMembVar {
                ident: memb_var_sigs[i].0,
                value: inputs[i].take().into_eval(),
            });
        }
        Self::Struct { memb_vars }
    }

    pub fn memb_var(&self, ident: CustomIdentifier) -> &EvalValue<'eval> {
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
