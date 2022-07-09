#[cfg(test)]
use check_utils::*;
use husky_vm_runtime_error::vm_runtime_error;

use crate::*;

use super::*;

pub struct __OwnedValue<'temp, 'eval: 'temp>(Box<dyn __AnyValueDyn<'eval> + 'temp>);

impl<'temp, 'eval: 'temp> From<Box<dyn __AnyValueDyn<'eval> + 'eval>>
    for __OwnedValue<'temp, 'eval>
{
    fn from(boxed_value: Box<dyn __AnyValueDyn<'eval> + 'eval>) -> Self {
        Self(boxed_value)
    }
}

impl<'temp, 'eval: 'temp> Clone for __OwnedValue<'temp, 'eval> {
    fn clone(&self) -> Self {
        Self(self.0.__clone_into_box_dyn())
    }
}

impl<'temp, 'eval: 'temp> PartialEq for __OwnedValue<'temp, 'eval> {
    fn eq(&self, other: &__OwnedValue<'temp, 'eval>) -> bool {
        self.0.__equal_any(&*other.0)
    }
}

impl<'temp, 'eval: 'temp> Eq for __OwnedValue<'temp, 'eval> {}

impl<'temp, 'eval: 'temp> __OwnedValue<'temp, 'eval> {
    pub fn new<T: __AnyValueDyn<'eval> + 'eval>(value: T) -> __OwnedValue<'temp, 'eval> {
        Self(Box::new(value))
    }

    pub fn from_any_ref(value: &(dyn __AnyValueDyn<'eval> + 'eval)) -> __OwnedValue<'temp, 'eval> {
        Self(value.__clone_into_box_dyn())
    }

    pub fn downcast_move<T: __AnyValue<'eval>>(self) -> __EvalResult<T> {
        // check type
        if (*self.0).__static_type_id_dyn() != T::__static_type_id() {
            Err(vm_runtime_error!(format!("type_mismatch")))
        } else {
            let raw_pointer = Box::into_raw(self.0);
            Ok(unsafe { *Box::from_raw(raw_pointer as *mut T) })
        }
    }

    pub fn take_into_arc(self) -> Arc<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'eval,
    {
        let raw_pointer = Box::<(dyn __AnyValueDyn<'eval> + 'temp)>::into_raw(self.0);
        unsafe { (*raw_pointer).__take_into_arc() }
    }

    pub fn any_ptr(&self) -> *const (dyn __AnyValueDyn<'eval> + 'temp) {
        &*(self.0)
    }

    pub fn any_ref(&self) -> &(dyn __AnyValueDyn<'eval> + 'temp) {
        &*self.0
    }

    pub fn any_mut_ptr(&mut self) -> *mut (dyn __AnyValueDyn<'eval> + 'temp) {
        &mut *self.0
    }

    pub fn downcast_ref<T: __AnyValue<'eval> + 'temp>(&self) -> &T {
        self.0.__downcast_ref()
        // if T::static_type_id() != self.0.static_type_id() {
        //     panic!()
        // }
        // let ptr: *const dyn __AnyValueDyn = &*self.0;
        // let ptr: *const T = ptr as *const T;
        // unsafe { &*ptr }
    }

    pub fn get_json_value(&self) -> serde_json::value::Value {
        self.0.__to_json_value_dyn()
    }
}

impl<'temp, 'eval: 'temp> std::fmt::Debug for __OwnedValue<'temp, 'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[test]
fn test_owned() {
    let a = __OwnedValue::new(0 as i32);
    let b: i32 = a.downcast_move().unwrap();
    should_eq!(b, 0);
}

#[test]
fn test_owned_type_mistach() {
    let a = __OwnedValue::new(0 as i32);
    let b = a.downcast_move::<f32>();
    should!(b.is_err());
}
