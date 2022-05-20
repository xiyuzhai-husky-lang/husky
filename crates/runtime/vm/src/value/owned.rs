#[cfg(test)]
use check_utils::*;

use crate::*;

use super::*;

pub struct OwnedValue<'eval>(pub(crate) Box<dyn AnyValueDyn<'eval>>);

impl<'eval> From<Box<dyn AnyValueDyn<'eval>>> for OwnedValue<'eval> {
    fn from(boxed_value: Box<dyn AnyValueDyn<'eval>>) -> Self {
        Self(boxed_value)
    }
}

impl<'eval> Clone for OwnedValue<'eval> {
    fn clone(&self) -> Self {
        Self(self.0.clone_into_box_dyn())
    }
}

impl<'eval> PartialEq for OwnedValue<'eval> {
    fn eq(&self, other: &OwnedValue<'eval>) -> bool {
        self.0.equal_any(&*other.0)
    }
}

impl<'eval> Eq for OwnedValue<'eval> {}

impl<'eval> OwnedValue<'eval> {
    pub fn new<T: AnyValueDyn<'eval>>(value: T) -> OwnedValue<'eval> {
        Self(Box::new(value))
    }

    pub fn take<T: AnyValue<'eval>>(self) -> VMRuntimeResult<T> {
        // check type
        if (*self.0).static_type_id_dyn() != T::static_type_id() {
            Err(vm_runtime_error!(format!("type_mismatch")))
        } else {
            let raw_pointer: *const (dyn AnyValueDyn + 'eval) =
                Box::<(dyn AnyValueDyn + 'eval)>::into_raw(self.0);
            Ok(unsafe { *Box::from_raw(raw_pointer as *mut T) })
        }
    }

    pub fn any_ptr(&self) -> *const (dyn AnyValueDyn<'eval>) {
        &*(self.0)
    }

    pub fn any_ref(&self) -> &dyn AnyValueDyn<'eval> {
        &*self.0
    }

    pub fn any_mut_ptr(&mut self) -> *mut dyn AnyValueDyn<'eval> {
        &mut *self.0
    }

    pub fn downcast_ref<T: AnyValue<'eval>>(&self) -> &T {
        self.0.downcast_ref()
        // if T::static_type_id() != self.0.static_type_id() {
        //     panic!()
        // }
        // let ptr: *const dyn AnyValueDyn = &*self.0;
        // let ptr: *const T = ptr as *const T;
        // unsafe { &*ptr }
    }

    pub fn share(&self) -> SharedValue<'eval> {
        todo!()
    }
}

impl<'eval> std::fmt::Debug for OwnedValue<'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[test]
fn test_owned() {
    let a = OwnedValue::new(0 as i32);
    let b: i32 = a.take().unwrap();
    should_eq!(b, 0);
}

#[test]
fn test_owned_type_mistach() {
    let a = OwnedValue::new(0 as i32);
    let b = a.take::<f32>();
    should!(b.is_err());
}
