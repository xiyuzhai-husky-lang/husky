use std::marker::PhantomData;

mod impl_cyclic_slice;
mod impl_hashmap;
mod impl_primitive;

#[derive(Hash)]
pub struct __Register<'eval> {
    pub data_kind: __RegisterDataKind,
    pub opt_data: Option<*mut dyn __RegistrableDyn>,
    phantom: PhantomData<&'eval ()>,
}

unsafe impl<'eval> Send for __Register<'eval> {}
unsafe impl<'eval> Sync for __Register<'eval> {}

impl<'eval> std::fmt::Debug for __Register<'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub trait __RegistrableSafe<'eval>: __Registrable + 'eval + Sized {
    fn to_register(self) -> __Register<'eval> {
        unsafe { self.__to_register__() }
    }
}

impl<'eval, T> __RegistrableSafe<'eval> for T where T: __Registrable + 'eval {}

impl<'eval> Clone for __Register<'eval> {
    fn clone(&self) -> Self {
        Self {
            data_kind: self.data_kind,
            opt_data: match self.data_kind {
                __RegisterDataKind::Value => self.opt_data,
                __RegisterDataKind::Box => todo!(),
                __RegisterDataKind::EvalRef => self.opt_data,
                __RegisterDataKind::TempRef => self.opt_data,
                __RegisterDataKind::TempMut => panic!(),
                __RegisterDataKind::Moved => panic!(),
                __RegisterDataKind::Undefined => todo!(),
            },
            phantom: PhantomData,
        }
    }
}

impl<'eval> PartialEq for __Register<'eval> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
        // self.data_kind == other.data_kind && self.opt_data == other.opt_data
    }
}
impl<'eval> Eq for __Register<'eval> {}

pub trait __StaticInfo {
    type __StaticSelf: __StaticInfo<__StaticSelf = Self::__StaticSelf> + __Registrable + 'static;

    fn __static_type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self::__StaticSelf>()
    }

    fn __static_type_name__() -> std::borrow::Cow<'static, str>;
}

pub trait __Registrable: __StaticInfo + std::fmt::Debug + Send + Sync {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval>;
}

impl<T> __RegistrableDyn for T
where
    T: __Registrable,
{
    unsafe fn drop_dyn(&mut self) {
        let ptr: *mut T = self;
        drop(Box::from_raw(ptr));
    }
}

pub trait __RegistrableDyn: std::fmt::Debug + Send + Sync {
    unsafe fn drop_dyn(&mut self);
}

impl<'eval> __Register<'eval> {
    pub unsafe fn new_direct<'a, T: __Registrable + 'a>(value: u64) -> __Register<'eval>
    where
        T: Copy,
    {
        __Register {
            data_kind: __RegisterDataKind::Value,
            opt_data: Some(value as *const () as *mut T::__StaticSelf as *mut dyn __RegistrableDyn),
            phantom: PhantomData,
        }
    }

    pub fn new_box<'a, T: __Registrable + 'a>(value: T) -> __Register<'eval> {
        let data: *mut T = Box::<T>::into_raw(Box::new(value));
        __Register {
            data_kind: __RegisterDataKind::Box,
            opt_data: Some(data as *mut T::__StaticSelf as *mut dyn __RegistrableDyn),
            phantom: PhantomData,
        }
    }

    pub unsafe fn new_eval_ref<T: __Registrable + 'eval>(value: &'eval T) -> __Register {
        let ptr: *const T = value;
        __Register {
            data_kind: __RegisterDataKind::EvalRef,
            opt_data: Some(ptr as *mut T::__StaticSelf as *mut dyn __RegistrableDyn),
            phantom: PhantomData,
        }
    }

    pub unsafe fn new_temp_ref<T: __Registrable>(value: &T) -> __Register<'eval> {
        let ptr: *const T = value;
        __Register {
            data_kind: __RegisterDataKind::TempRef,
            opt_data: Some(ptr as *mut T::__StaticSelf as *mut dyn __RegistrableDyn),
            phantom: PhantomData,
        }
    }

    pub unsafe fn new_temp_mut<T: __Registrable>(value: &mut T) -> __Register<'eval> {
        let ptr: *const T = value;
        __Register {
            data_kind: __RegisterDataKind::TempMut,
            opt_data: Some(ptr as *mut T::__StaticSelf as *mut dyn __RegistrableDyn),
            phantom: PhantomData,
        }
    }

    pub unsafe fn new_moved() -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::Moved,
            opt_data: None,
            phantom: PhantomData,
        }
    }

    pub fn new_undefined() -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::Undefined,
            opt_data: None,
            phantom: PhantomData,
        }
    }

    pub unsafe fn new_undefined_with_message() -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::Undefined,
            opt_data: None,
            phantom: PhantomData,
        }
    }

    pub unsafe fn __copy__(&self) -> Self {
        todo!()
        // Self {
        //     data_kind: self.data_kind,
        //     data: match self.data_kind {
        //         __RegisterDataKind::Data => todo!(),
        //         __RegisterDataKind::Box => todo!(),
        //         __RegisterDataKind::EvalRef => todo!(),
        //         __RegisterDataKind::TempRef => todo!(),
        //         __RegisterDataKind::TempMut => todo!(),
        //         __RegisterDataKind::Moved => todo!(),
        //     },
        // }
    }

    pub unsafe fn downcast<T>(&mut self) -> T {
        todo!()
    }

    pub unsafe fn downcast_eval_ref<T: 'eval>(&self) -> &'eval T {
        todo!()
    }

    pub unsafe fn downcast_temp_ref<T>(&self) -> &T {
        todo!()
    }

    pub unsafe fn downcast_temp_mut<T>(&mut self) -> &mut T {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum __RegisterDataKind {
    Value,
    Box,
    EvalRef,
    TempRef,
    TempMut,
    Moved,
    Undefined,
}

impl<'eval> Drop for __Register<'eval> {
    fn drop(&mut self) {
        match self.data_kind {
            __RegisterDataKind::Box | __RegisterDataKind::Undefined => unsafe {
                // when undefined, opt_data might hold a message
                (*self.opt_data.unwrap()).drop_dyn()
            },
            _ => (),
        }
    }
}

#[macro_export]
macro_rules! register_new_copyable {
    ($argument: expr, $Type: ty, direct) => {{
        __Register::new_direct::<$Type>($argument as u64)
    }};
}

// impl<'temp, 'eval: 'temp> dyn __RegistrableDyn + 'temp {
//     #[inline]
//     pub fn __downcast_ref<'a, T: __Registrable>(&'a self) -> &'a T {
//         if T::__static_type_id() != self.__static_type_id_dyn() {
//             panic!(
//                 "expect type `{}`, but got `{}` instead",
//                 T::__static_type_name(),
//                 self.__static_type_name_dyn()
//             )
//         }
//         let ptr: *const dyn __AnyValueDyn = &*self;
//         let ptr: *const T = ptr as *const T;
//         unsafe { &*ptr }
//     }
//     #[inline]
//     pub fn __downcast_copy<'a, T: __Registrable + Copy>(&'a self) -> T {
//         if T::__static_type_id() != self.__static_type_id_dyn() {
//             panic!(
//                 "expect type `{}`, but got `{}` instead",
//                 T::__static_type_name(),
//                 self.__static_type_name_dyn()
//             )
//         }
//         let ptr: *const dyn __AnyValueDyn = &*self;
//         let ptr: *const T = ptr as *const T;
//         unsafe { *ptr }
//     }

//     #[inline]
//     pub fn __downcast_mut<T: __Registrable>(&mut self) -> &mut T {
//         if T::__static_type_id() != self.__static_type_id_dyn() {
//             panic!(
//                 "expect type `{}`, but got `{}` instead",
//                 T::__static_type_name(),
//                 self.__static_type_name_dyn()
//             )
//         }
//         let ptr: *mut dyn __AnyValueDyn = &mut *self;
//         let ptr: *mut T = ptr as *mut T;
//         unsafe { &mut *ptr }
//     }
// }
