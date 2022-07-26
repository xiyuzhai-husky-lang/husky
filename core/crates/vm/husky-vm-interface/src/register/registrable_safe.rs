use super::*;

pub trait __RegistrableSafe<'eval>: __Registrable + 'eval + Sized {
    fn to_register(self) -> __Register<'eval> {
        unsafe { self.__to_register__() }
    }
}

impl<'eval, T> __RegistrableSafe<'eval> for T where T: __Registrable + 'eval {}
