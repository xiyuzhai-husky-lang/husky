mod binding;

pub use binding::*;
use husky_entity_route::EntityRoutePtr;
use husky_trace_protocol::PrimitiveValueData;
use husky_vm_interface::__Register;

pub trait VMRegisterMethod<'eval> {
    fn __snapshot__(&self) -> __Register<'eval>;
    fn __stack__(&self) -> __Register<'eval>;
    fn __eval__(&self) -> __Register<'eval>;
    fn __eq__(&self, other: &__Register<'eval>) -> bool;
    fn primitive(&self) -> PrimitiveValueData;
    unsafe fn __bind__(&mut self, binding: Binding) -> __Register<'eval>;
    fn __print_short__(&self) -> String;
    fn __to_bool__(self) -> bool;
    fn __ty__(&self) -> EntityRoutePtr;
}

impl<'eval> VMRegisterMethod<'eval> for __Register<'eval> {
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

    unsafe fn __bind__(&mut self, binding: Binding) -> __Register<'eval> {
        todo!()
    }

    fn __print_short__(&self) -> String {
        todo!()
    }

    fn __to_bool__(self) -> bool {
        self.primitive().to_bool()
    }

    fn __eq__(&self, other: &__Register<'eval>) -> bool {
        todo!()
    }

    fn __ty__(&self) -> EntityRoutePtr {
        todo!()
    }
}
