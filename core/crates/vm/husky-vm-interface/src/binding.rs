use std::ffi::CStr;

use husky_print_utils::p;
#[cfg(feature = "binding")]
use husky_vm_binding::Binding;

#[cfg(feature = "extra")]
use crate::*;

#[cfg(feature = "extra")]
impl<'eval> __Register<'eval> {
    pub unsafe fn verbatim_copy(&self) -> __Register<'eval> {
        Self {
            data_kind: self.data_kind,
            data: self.data,
            vtable: self.vtable,
        }
    }

    pub fn snapshot(&self) -> __Register<'eval> {
        unsafe {
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => self.verbatim_copy(),
                __RegisterDataKind::Box => todo!(),
                __RegisterDataKind::EvalRef => todo!(),
                __RegisterDataKind::TempRef => todo!(),
                __RegisterDataKind::TempMut => self.clone_ptr_into_box(),
                __RegisterDataKind::Moved => todo!(),
                __RegisterDataKind::Undefined => todo!(),
                __RegisterDataKind::Unreturned => todo!(),
            }
        }
    }

    pub(crate) unsafe fn clone_ptr_into_box(&self) -> __Register<'eval> {
        let ptr = self.data.as_ptr;
        let data = unsafe {
            __RegisterData {
                as_ptr: self.vtable.clone.unwrap()(ptr),
            }
        };
        Self {
            data_kind: __RegisterDataKind::Box,
            data,
            vtable: self.vtable,
        }
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

    // pub fn primitive(&self) -> PrimitiveValueData {
    //     todo!()
    //     // unsafe { (*self.opt_data.unwrap()).__primitive_dyn__(self.data_kind) }
    // }

    pub fn bind_copy(&self) -> __Register<'eval> {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => __Register {
                data_kind: self.data_kind,
                data: self.data,
                vtable: self.vtable,
            },
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => todo!(),
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    pub fn bind_move(&mut self) -> __Register<'eval> {
        todo!()
    }

    pub fn bind_eval_ref(&self) -> __Register<'eval> {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => todo!(),
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => unsafe { self.verbatim_copy() },
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    pub fn bind_temp_ref(&self) -> __Register<'eval> {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => todo!(),
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => todo!(),
            __RegisterDataKind::TempRef => unsafe { self.verbatim_copy() },
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
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
                    as_ptr: &self.data as *const _ as *mut (),
                };
                __Register {
                    data_kind: __RegisterDataKind::TempMut,
                    data,
                    vtable: self.vtable,
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

    pub fn any_ptr(&self) -> *mut () {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => todo!(),
            __RegisterDataKind::Box
            | __RegisterDataKind::EvalRef
            | __RegisterDataKind::TempRef
            | __RegisterDataKind::TempMut => unsafe { self.data.as_ptr },
            __RegisterDataKind::Moved
            | __RegisterDataKind::Undefined
            | __RegisterDataKind::Unreturned => panic!(),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => {
                (self.vtable.primitive_value_to_bool).unwrap()(self.data)
            }
            __RegisterDataKind::Box
            | __RegisterDataKind::EvalRef
            | __RegisterDataKind::TempRef
            | __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved
            | __RegisterDataKind::Undefined
            | __RegisterDataKind::Unreturned => panic!(),
        }
    }

    pub fn cache_eval(&mut self) {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => unsafe {
                // order is of greate importance here
                // if data_kind is set first, and something happens during setting data
                // then there is a memory problem
                // this ffi call could take some time I guess
                let as_ptr = (self.vtable.primitive_value_to_box.unwrap())(&mut self.data);
                self.data = __RegisterData { as_ptr };
                self.data_kind = __RegisterDataKind::Box;
            },
            _ => (),
        }
    }

    pub fn share_cached(&self) -> __Register<'eval> {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => panic!(),
            __RegisterDataKind::Box | __RegisterDataKind::EvalRef => __Register {
                data_kind: __RegisterDataKind::EvalRef,
                data: self.data,
                vtable: self.vtable,
            },
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => unsafe { self.verbatim_copy() },
            // __Register::Copyable(value) => panic!(),
            // __Register::Owned(value) => __Register::EvalRef(__EvalRef(&*value.any_ptr())),
            // __Register::EvalRef(value) => __Register::EvalRef(*value),
            // __Register::EvalPure(value) => __Register::EvalPure(value.clone()),
            // __Register::Undefined => __Register::Undefined,
            // __Register::Unreturned => __Register::Unreturned,
        }
    }
}
