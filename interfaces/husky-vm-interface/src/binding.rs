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
        unsafe { (*self.opt_data.unwrap()).__primitive_dyn__(self.data_kind) }
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
                let ptr = &self.opt_data as *const _;
                let ptr = ptr as *mut dyn __RegistrableDyn;
                __Register {
                    data_kind: todo!(),
                    opt_data: todo!(),
                    phantom: std::marker::PhantomData,
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
