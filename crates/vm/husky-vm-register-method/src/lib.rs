use husky_any::{DeprecatedVirtualStruct, __DEPRECATED_VIRTUAL_STRUCT_VTABLE};
use husky_vm_binding::Binding;
use husky_vm_interface::{__RegisterDataKind, __RegularValue};

pub trait VMRegisterMethodX: Sized {
    fn virtual_struct_field(self, field_idx: u8, field_binding: Binding) -> __RegularValue;
}

impl VMRegisterMethodX for __RegularValue {
    fn virtual_struct_field(mut self, field_idx: u8, field_binding: Binding) -> __RegularValue {
        if self.vtable.typename_str_hash_u64
            != __DEPRECATED_VIRTUAL_STRUCT_VTABLE.typename_str_hash_u64
        {
            panic!(
                "expect virtual struct, but get `{}` instead",
                self.vtable.typename_str
            )
        }
        match field_binding {
            Binding::Leash => match self.data_kind() {
                __RegisterDataKind::PrimitiveValue => todo!(),
                __RegisterDataKind::Box => todo!(),
                __RegisterDataKind::Leash => {
                    let this_value: &'static DeprecatedVirtualStruct =
                        self.downcast_leash(&__DEPRECATED_VIRTUAL_STRUCT_VTABLE);
                    this_value.bind_field_leash(field_idx)
                }
                __RegisterDataKind::TempRef => todo!(),
                __RegisterDataKind::TempMut => todo!(),
                __RegisterDataKind::Moved => todo!(),
                __RegisterDataKind::SomeNone => todo!(),
                __RegisterDataKind::Unreturned => todo!(),
            },
            Binding::TempRef => {
                let this_value: &DeprecatedVirtualStruct =
                    self.downcast_temp_ref(&__DEPRECATED_VIRTUAL_STRUCT_VTABLE);
                this_value.bind_field_temp_ref(field_idx)
            }
            Binding::TempMut => {
                let this_value: &mut DeprecatedVirtualStruct =
                    unsafe { self.downcast_temp_mut(&__DEPRECATED_VIRTUAL_STRUCT_VTABLE) };
                this_value.bind_field_mut(field_idx)
            }
            Binding::Move => todo!(),
            Binding::Copy => {
                let this_value: &DeprecatedVirtualStruct =
                    self.downcast_temp_ref(&__DEPRECATED_VIRTUAL_STRUCT_VTABLE);
                this_value.bind_field_copy(field_idx)
            }
            Binding::DerefCopy => todo!(),
        }
    }
}
