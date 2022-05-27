#[cfg(test)]
use check_utils::*;

use crate::*;

use super::*;

pub struct OwnedValue<'vm, 'eval: 'vm>(Box<dyn AnyValueDyn<'eval> + 'vm>);

impl<'vm, 'eval: 'vm> From<Box<dyn AnyValueDyn<'eval> + 'eval>> for OwnedValue<'vm, 'eval> {
    fn from(boxed_value: Box<dyn AnyValueDyn<'eval> + 'eval>) -> Self {
        Self(boxed_value)
    }
}

impl<'vm, 'eval: 'vm> Clone for OwnedValue<'vm, 'eval> {
    fn clone(&self) -> Self {
        Self(self.0.clone_into_box_dyn())
    }
}

impl<'vm, 'eval: 'vm> PartialEq for OwnedValue<'vm, 'eval> {
    fn eq(&self, other: &OwnedValue<'vm, 'eval>) -> bool {
        self.0.equal_any(&*other.0)
    }
}

impl<'vm, 'eval: 'vm> Eq for OwnedValue<'vm, 'eval> {}

impl<'vm, 'eval: 'vm> OwnedValue<'vm, 'eval> {
    pub fn new<T: AnyValueDyn<'eval> + 'eval>(value: T) -> OwnedValue<'vm, 'eval> {
        Self(Box::new(value))
    }

    pub fn from_any_ref(value: &(dyn AnyValueDyn<'eval> + 'eval)) -> OwnedValue<'vm, 'eval> {
        Self(value.clone_into_box_dyn())
    }

    pub fn take<T: AnyValue<'eval>>(self) -> VMRuntimeResult<T> {
        // check type
        if (*self.0).static_type_id_dyn() != T::static_type_id() {
            Err(vm_runtime_error!(format!("type_mismatch")))
        } else {
            let raw_pointer = Box::into_raw(self.0);
            Ok(unsafe { *Box::from_raw(raw_pointer as *mut T) })
        }
    }

    pub fn take_into_arc(self) -> Arc<dyn AnyValueDyn<'eval> + 'vm>
    where
        Self: 'eval,
    {
        let raw_pointer = Box::<(dyn AnyValueDyn<'eval> + 'vm)>::into_raw(self.0);
        unsafe { (*raw_pointer).take_into_arc() }
    }

    pub fn any_ptr(&self) -> *const (dyn AnyValueDyn<'eval> + 'vm) {
        &*(self.0)
    }

    pub fn any_ref(&self) -> &(dyn AnyValueDyn<'eval> + 'vm) {
        &*self.0
    }

    pub fn any_mut_ptr(&mut self) -> *mut (dyn AnyValueDyn<'eval> + 'vm) {
        &mut *self.0
    }

    pub fn downcast_ref<T: AnyValue<'eval> + 'vm>(&self) -> &T {
        self.0.downcast_ref()
        // if T::static_type_id() != self.0.static_type_id() {
        //     panic!()
        // }
        // let ptr: *const dyn AnyValueDyn = &*self.0;
        // let ptr: *const T = ptr as *const T;
        // unsafe { &*ptr }
    }

    pub fn get_json_value(&self) -> serde_json::value::Value {
        self.0.get_json_value_dyn()
    }
}

impl<'vm, 'eval: 'vm> std::fmt::Debug for OwnedValue<'vm, 'eval> {
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
