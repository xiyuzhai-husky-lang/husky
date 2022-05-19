#[cfg(test)]
use check_utils::*;

use crate::*;

use super::*;

pub struct OwnedValue<'eval> {
    pub(crate) inner: Box<dyn AnyValueDyn<'eval>>,
}

impl<'eval> Clone for OwnedValue<'eval> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_any(),
        }
    }
}

impl<'eval> PartialEq for OwnedValue<'eval> {
    fn eq(&self, other: &OwnedValue<'eval>) -> bool {
        self.inner.equal_any(&*other.inner)
    }
}

impl<'eval> Eq for OwnedValue<'eval> {}

impl<'eval> OwnedValue<'eval> {
    pub fn new<T: AnyValueDyn<'eval>>(value: T) -> OwnedValue<'eval> {
        Self {
            inner: Box::new(value),
        }
    }

    pub fn clone_from(value: &dyn AnyValueDyn<'eval>) -> OwnedValue<'eval> {
        Self {
            inner: value.clone_any(),
        }
    }

    pub fn take<T: AnyValue<'eval>>(self) -> VMRuntimeResult<T> {
        // check type
        if (*self.inner).static_type_id() != T::static_type_id() {
            Err(vm_runtime_error!(format!("type_mismatch")))
        } else {
            let raw_pointer: *const (dyn AnyValueDyn + 'eval) =
                Box::<(dyn AnyValueDyn + 'eval)>::into_raw(self.inner);
            Ok(unsafe { *Box::from_raw(raw_pointer as *mut T) })
        }
    }

    pub fn any_pointer(&self) -> *const (dyn AnyValueDyn<'eval>) {
        &*(self.inner)
    }

    pub fn any_ref(&self) -> &dyn AnyValueDyn<'eval> {
        &*self.inner
    }

    pub fn any_mut_ptr(&mut self) -> *mut dyn AnyValueDyn<'eval> {
        &mut *self.inner
    }

    pub fn downcast_ref<T: AnyValue<'eval>>(&self) -> &T {
        if T::static_type_id() != self.inner.static_type_id() {
            panic!()
        }
        let ptr: *const dyn AnyValueDyn = &*self.inner;
        let ptr: *const T = ptr as *const T;
        unsafe { &*ptr }
    }
}

impl<'eval> std::fmt::Debug for OwnedValue<'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
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
