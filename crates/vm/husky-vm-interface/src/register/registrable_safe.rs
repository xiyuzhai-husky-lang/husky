use super::*;

pub trait __RegistrableSafe: __Registrable + 'static + Sized {
    fn to_register(self) -> __RegularValue {
        unsafe { self.__to_register() }
    }
}

impl<T> __RegistrableSafe for T where T: __Registrable + 'static {}
