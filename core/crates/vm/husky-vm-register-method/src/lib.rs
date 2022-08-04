use husky_any::{VirtualStruct, __VIRTUAL_STRUCT_VTABLE};
use husky_entity_route::EntityRoutePtr;
use husky_vm_binding::Binding;
use husky_vm_interface::{__Register, __RegisterDataKind};

pub trait VMRegisterMethodX<'eval>: Sized {
    fn virtual_struct_field(self, field_idx: u8, field_binding: Binding) -> __Register<'eval>;
}

impl<'eval> VMRegisterMethodX<'eval> for __Register<'eval> {
    fn virtual_struct_field(mut self, field_idx: u8, field_binding: Binding) -> __Register<'eval> {
        assert_eq!(self.vtable as *const _, unsafe {
            &__VIRTUAL_STRUCT_VTABLE as *const _
        });
        match field_binding {
            Binding::EvalRef => match self.data_kind() {
                __RegisterDataKind::PrimitiveValue => todo!(),
                __RegisterDataKind::Box => todo!(),
                __RegisterDataKind::EvalRef => {
                    let this_value: &'eval VirtualStruct =
                        unsafe { self.downcast_eval_ref(&__VIRTUAL_STRUCT_VTABLE) };
                    this_value.bind_field_eval_ref(field_idx)
                }
                __RegisterDataKind::TempRef => todo!(),
                __RegisterDataKind::TempMut => todo!(),
                __RegisterDataKind::Moved => todo!(),
                __RegisterDataKind::Undefined => todo!(),
                __RegisterDataKind::Unreturned => todo!(),
            },
            Binding::TempRef => {
                let this_value: &VirtualStruct =
                    unsafe { self.downcast_temp_ref(&__VIRTUAL_STRUCT_VTABLE) };
                this_value.bind_field_temp_ref(field_idx)
            }
            Binding::TempMut => {
                let this_value: &mut VirtualStruct =
                    unsafe { self.downcast_temp_mut(&__VIRTUAL_STRUCT_VTABLE) };
                this_value.bind_field_mut(field_idx)
            }
            Binding::Move => todo!(),
            Binding::Copy => {
                let this_value: &VirtualStruct =
                    unsafe { self.downcast_temp_ref(&__VIRTUAL_STRUCT_VTABLE) };
                this_value.bind_field_copy(field_idx)
            }
        }
    }
}
