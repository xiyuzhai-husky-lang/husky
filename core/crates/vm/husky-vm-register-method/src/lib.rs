use husky_entity_route::EntityRoutePtr;
use husky_vm_binding::Binding;
use husky_vm_interface::{PrimitiveValueData, __Register};

pub trait VMRegisterMethodX<'eval> {
    fn json_value(&self) -> serde_json::Value;
}

impl<'eval> VMRegisterMethodX<'eval> for __Register<'eval> {
    fn json_value(&self) -> serde_json::Value {
        todo!()
    }
}
