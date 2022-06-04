use serde::Serialize;

use crate::*;

#[derive(Debug, Clone)]
pub enum MemberValue<'eval> {
    Copyable(CopyableValue),
    Boxed(OwnedValue<'eval, 'eval>),
    GlobalPure(Arc<dyn AnyValueDyn<'eval> + 'eval>),
    EvalRef(&'eval (dyn AnyValueDyn<'eval> + 'eval)),
    Moved,
}

impl<'eval> PartialEq for MemberValue<'eval> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Copyable(l0), Self::Copyable(r0)) => l0 == r0,
            (Self::Boxed(l0), Self::Boxed(r0)) => l0 == r0,
            (Self::GlobalPure(l0), Self::GlobalPure(r0)) => todo!(),
            (Self::EvalRef(l0), Self::EvalRef(r0)) => todo!(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<'eval> Eq for MemberValue<'eval> {}

impl<'temp, 'eval: 'temp> MemberValue<'eval> {
    pub fn into_stack(self) -> TempValue<'temp, 'eval> {
        match self {
            MemberValue::Copyable(value) => TempValue::Copyable(value),
            MemberValue::Boxed(value) => TempValue::OwnedEval(value),
            MemberValue::GlobalPure(value) => TempValue::EvalPure(value),
            MemberValue::EvalRef(value) => TempValue::EvalRef(value),
            MemberValue::Moved => panic!(),
        }
    }

    pub fn any_ref(&self) -> &(dyn AnyValueDyn<'eval> + 'eval) {
        match self {
            MemberValue::Copyable(_) => todo!(),
            MemberValue::Boxed(ref value) => value.any_ref(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::EvalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn any_ptr(&self) -> *const (dyn AnyValueDyn<'eval> + 'eval) {
        match self {
            MemberValue::Copyable(_) => todo!(),
            MemberValue::Boxed(ref value) => value.any_ref(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::EvalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn bind(&self, binding: Binding) -> TempValue<'temp, 'eval> {
        match binding {
            Binding::EvalRef => self.bind_eval_ref(),
            Binding::TempRef => self.bind_temp_ref(),
            Binding::TempRefMut => todo!(),
            Binding::Move => todo!(),
            Binding::Copy => match self {
                MemberValue::Copyable(value) => TempValue::Copyable(*value),
                MemberValue::Boxed(_) => todo!(),
                MemberValue::GlobalPure(_) => todo!(),
                MemberValue::EvalRef(_) => todo!(),
                MemberValue::Moved => todo!(),
            },
        }
        // EagerContract::Move => todo!(),
        // EagerContract::UseForVarInit => todo!(),
        // EagerContract::Return => match self {
        //     VirtualTy::Struct { fields } => match fields.data()[field_idx].1 {
        //         MemberValue::Copyable(value) => VMValue::Copyable(value),
        //         MemberValue::Boxed(_) => todo!(),
        //         MemberValue::GlobalPure(_) => todo!(),
        //         MemberValue::EvalRef(_) => todo!(),
        //         MemberValue::Moved => todo!(),
        //     },
        // },
        // EagerContract::RefMut => todo!(),
        // EagerContract::MoveMut => todo!(),
        // EagerContract::Exec => todo!(),
        // EagerContract::UseMemberForLetInit => match self {
        //     VirtualTy::Struct { fields } => match fields.data()[field_idx].1 {
        //         MemberValue::Copyable(_) => todo!(),
        //         MemberValue::Boxed(ref value) => {
        //             let ptr = value.any_ptr();
        //             VMValue::FullyOwnedRef(unsafe { &*ptr })
        //         }
        //         MemberValue::GlobalPure(_) => todo!(),
        //         MemberValue::EvalRef(_) => todo!(),
        //         MemberValue::Moved => todo!(),
        //     },
        // },
        // EagerContract::UseMemberForVarInit => todo!(),
        // EagerContract::UseForAssignRvalue => todo!(),
    }

    pub fn bind_eval_ref(&self) -> TempValue<'temp, 'eval> {
        match self {
            MemberValue::EvalRef(value) => TempValue::EvalRef(*value),
            MemberValue::Copyable(_) => panic!(),
            MemberValue::Boxed(ref boxed_value) => {
                TempValue::EvalRef(unsafe { &*boxed_value.any_ptr() })
            }
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn bind_temp_ref(&self) -> TempValue<'temp, 'eval> {
        match self {
            MemberValue::Copyable(_) => panic!(),
            MemberValue::Boxed(boxed_value) => {
                TempValue::TempRefEval(unsafe { &*boxed_value.any_ptr() })
            }
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::EvalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn binding_mut<'a>(&'a mut self, owner: VMStackIdx) -> TempValue<'temp, 'eval> {
        let value_mut: *mut dyn AnyValueDyn<'eval> = match self {
            MemberValue::Copyable(value) => value.any_mut(),
            MemberValue::Boxed(value) => value.any_mut_ptr(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::EvalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        };
        TempValue::CopyableOrTempMutEval {
            value: unsafe { &mut *value_mut },
            owner,
            gen: (),
        }
    }

    pub fn share_globally(&self) -> EvalValue<'eval> {
        match self {
            MemberValue::Copyable(value) => EvalValue::Copyable(*value),
            MemberValue::Boxed(_) => todo!(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::EvalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn copy_into_stack(&self) -> TempValue<'temp, 'eval> {
        match self {
            MemberValue::Copyable(value) => TempValue::Copyable(*value),
            MemberValue::Boxed(_) => todo!(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::EvalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }
}

impl<'eval> AnyValue<'eval> for MemberValue<'eval> {
    fn static_type_id() -> StaticTypeId {
        StaticTypeId::AnyMemberValue
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "AnyMemberValue".into()
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        match self {
            MemberValue::Copyable(value) => value.get_primitive_json_value(),
            MemberValue::Boxed(value) => value.get_json_value(),
            MemberValue::GlobalPure(value) => value.get_json_value_dyn(),
            MemberValue::EvalRef(value) => value.get_json_value_dyn(),
            MemberValue::Moved => todo!(),
        }
    }
}
