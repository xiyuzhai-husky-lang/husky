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
            __RegisterDataKind::Value
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
    }
    pub fn bind_copy(&self) -> __Register<'eval> {
        todo!()
    }

    pub fn bind_move(&mut self) -> __Register<'eval> {
        todo!()
    }

    pub fn bind_eval_ref(&'eval self) -> __Register<'eval> {
        todo!()
    }

    pub fn bind_temp_ref(&self) -> __Register<'eval> {
        todo!()
    }

    pub fn bind_temp_mut(&self) -> __Register<'eval> {
        todo!()
    }

    pub unsafe fn bind(&mut self, binding: Binding) -> __Register<'eval> {
        todo!()
    }

    pub fn print_short(&self) -> String {
        todo!()
    }

    pub fn to_bool(self) -> bool {
        self.primitive().to_bool()
    }
}
