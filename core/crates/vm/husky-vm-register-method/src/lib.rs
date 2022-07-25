mod binding;

pub use binding::*;
use husky_entity_route::EntityRoutePtr;
use husky_vm_binding::Binding;
use husky_vm_interface::{PrimitiveValueData, __Register};

pub trait VMRegisterMethodX<'eval> {
    fn __snapshot__(&self) -> __Register<'eval>;
    fn __stack__(&self) -> __Register<'eval>;
    fn __eval__(&self) -> __Register<'eval>;
    fn primitive(&self) -> PrimitiveValueData;
    fn bind_copy(&self) -> __Register<'eval>;
    fn bind_move(&mut self) -> __Register<'eval>;
    fn bind_eval_ref(&'eval self) -> __Register<'eval>;
    fn bind_temp_ref(&self) -> __Register<'eval>;
    fn bind_temp_mut(&self) -> __Register<'eval>;
    unsafe fn bind(&mut self, binding: Binding) -> __Register<'eval>;
    fn print_short(&self) -> String;
    fn to_bool(self) -> bool;
    fn ty(&self) -> EntityRoutePtr;
}

impl<'eval> VMRegisterMethodX<'eval> for __Register<'eval> {
    fn __snapshot__(&self) -> __Register<'eval> {
        todo!()
    }

    fn __stack__(&self) -> __Register<'eval> {
        todo!()
    }

    fn __eval__(&self) -> __Register<'eval> {
        todo!()
    }

    fn primitive(&self) -> PrimitiveValueData {
        todo!()
    }
    fn bind_copy(&self) -> __Register<'eval> {
        todo!()
    }

    fn bind_move(&mut self) -> __Register<'eval> {
        todo!()
    }

    fn bind_eval_ref(&'eval self) -> __Register<'eval> {
        todo!()
    }

    fn bind_temp_ref(&self) -> __Register<'eval> {
        todo!()
    }

    fn bind_temp_mut(&self) -> __Register<'eval> {
        todo!()
    }

    unsafe fn bind(&mut self, binding: Binding) -> __Register<'eval> {
        todo!()
    }

    fn print_short(&self) -> String {
        todo!()
    }

    fn to_bool(self) -> bool {
        self.primitive().to_bool()
    }

    fn ty(&self) -> EntityRoutePtr {
        todo!()
    }
}
