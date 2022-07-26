#[cfg(feature = "binding")]
use husky_vm_binding::Binding;

#[cfg(feature = "extra")]
use crate::*;

#[cfg(feature = "extra")]
impl<'eval> __Register<'eval> {
    pub fn snapshot(&self) -> __Register<'eval> {
        todo!()
    }

    pub fn stack(&self) -> __Register<'eval> {
        todo!()
    }

    pub fn eval(&self) -> __Register<'eval> {
        todo!()
    }

    pub fn into_eval(self) -> __Register<'eval> {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue
            | __RegisterDataKind::Box
            | __RegisterDataKind::EvalRef
            | __RegisterDataKind::Undefined => self,
            __RegisterDataKind::TempRef => panic!(),
            __RegisterDataKind::TempMut => panic!(),
            __RegisterDataKind::Moved => panic!(),
            __RegisterDataKind::Unreturned => panic!(),
        }
    }

    pub fn primitive(&self) -> PrimitiveValueData {
        todo!()
        // unsafe { (*self.opt_data.unwrap()).__primitive_dyn__(self.data_kind) }
    }

    pub fn bind_copy(&self) -> __Register<'eval> {
        todo!()
    }

    pub fn bind_move(&mut self) -> __Register<'eval> {
        todo!()
    }

    pub fn bind_eval_ref(&self) -> __Register<'eval> {
        todo!()
    }

    pub fn bind_temp_ref(&self) -> __Register<'eval> {
        todo!()
    }

    pub fn bind_temp_mut(&self) -> __Register<'eval> {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => {
                // C standard (N1570, 6.7.2.1 Structure and union specifiers) says:
                // 16 The size of a union is sufficient to contain the largest of its members.
                // The value of at most one of the members can be stored in a union object at any time.
                // A pointer to a union object, suitably converted, points to each of its members
                // (or if a member is a bit- field, then to the unit in which it resides),
                // and vice versa.
                // so it's ultimately safe to do this
                let data = __RegisterData {
                    as_opt_ptr: Some(&self.data as *const _ as *mut ()),
                };
                __Register {
                    data_kind: __RegisterDataKind::TempMut,
                    data,
                    proto: self.proto,
                }
            }
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => todo!(),
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    pub unsafe fn bind(&mut self, binding: Binding) -> __Register<'eval> {
        match binding {
            Binding::EvalRef => self.bind_eval_ref(),
            Binding::TempRef => self.bind_temp_ref(),
            Binding::TempMut => self.bind_temp_mut(),
            Binding::Move => self.bind_move(),
            Binding::Copy => self.bind_copy(),
        }
    }

    pub fn print_short(&self) -> String {
        // print short ad hoc
        "...".to_string()
    }

    pub fn to_bool(self) -> bool {
        self.primitive().to_bool()
    }
}
