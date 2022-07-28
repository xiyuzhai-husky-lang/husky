use husky_any::{VirtualStruct, __VIRTUAL_STRUCT_VTABLE};
use husky_entity_route::EntityRoutePtr;
use husky_vm_binding::Binding;
use husky_vm_interface::__Register;

pub trait VMRegisterMethodX<'eval>: Sized {
    fn json_value(&self) -> serde_json::Value;
    fn virtual_struct_field(self, field_idx: u8, field_binding: Binding) -> __Register<'eval>;
}

impl<'eval> VMRegisterMethodX<'eval> for __Register<'eval> {
    fn json_value(&self) -> serde_json::Value {
        todo!()
    }
    fn virtual_struct_field(mut self, field_idx: u8, field_binding: Binding) -> __Register<'eval> {
        assert_eq!(self.vtable as *const _, unsafe {
            &__VIRTUAL_STRUCT_VTABLE as *const _
        });
        match field_binding {
            Binding::EvalRef => todo!(),
            Binding::TempRef => todo!(),
            Binding::TempMut => {
                let this_value: &mut VirtualStruct = unsafe { self.downcast_temp_mut() };
                this_value.bind_field_mut(field_idx)
            }
            Binding::Move => todo!(),
            Binding::Copy => {
                let this_value: &VirtualStruct = unsafe { self.downcast_temp_ref() };
                this_value.bind_field_copy(field_idx)
            }
        }
    }
}
