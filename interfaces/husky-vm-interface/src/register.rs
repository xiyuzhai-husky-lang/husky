mod impl_primitive;

pub struct __Register {
    pub data_kind: __RegisterDataKind,
    pub data: *mut dyn __RegistrableDyn,
}

pub trait __StaticInfo {
    type __StaticSelf: __Registrable + 'static;

    fn __static_type_name() -> std::borrow::Cow<'static, str>;
}

pub trait __Registrable: __StaticInfo {
    unsafe fn __to_register(self) -> __Register;
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

pub trait __RegistrableDyn {
    unsafe fn drop_dyn(&mut self);
}

impl __Register {
    pub unsafe fn new_direct<'a, T: __Registrable + 'a>(value: u64) -> __Register
    where
        T: Copy,
    {
        let data = value as *const () as *mut T::__StaticSelf;
        __Register {
            data_kind: __RegisterDataKind::Value,
            data,
        }
    }

    pub unsafe fn new_box<'a, T: __Registrable + 'a>(value: T) -> __Register {
        let data: *mut T = Box::<T>::into_raw(Box::new(value));
        let data = data as *mut T::__StaticSelf;
        __Register {
            data_kind: __RegisterDataKind::Box,
            data,
        }
    }

    pub unsafe fn new_eval_ref<'eval, T: __Registrable + 'eval>(value: &'eval T) -> __Register {
        let ptr: *const T = value;
        let ptr = ptr as *mut T::__StaticSelf;
        __Register {
            data_kind: __RegisterDataKind::EvalRef,
            data: ptr,
        }
    }

    pub unsafe fn copy(&self) -> Self {
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

    pub unsafe fn downcast_eval_ref<'eval, T: 'eval>(&self) -> &'eval T {
        todo!()
    }

    pub unsafe fn downcast_temp_ref<T>(&self) -> &T {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum __RegisterDataKind {
    Value,
    Box,
    EvalRef,
    TempRef,
    TempMut,
    Moved,
}

impl Drop for __Register {
    fn drop(&mut self) {
        match self.data_kind {
            __RegisterDataKind::Box => unsafe { (*self.data).drop_dyn() },
            _ => (),
        }
    }
}
