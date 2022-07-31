use super::*;
use husky_vm_interface::__RegistrableSafe;

#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct Label(pub i32);

impl From<u8> for Label {
    fn from(v: u8) -> Self {
        Self(v as i32)
    }
}

impl __StaticInfo for Label {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "Label".into()
    }
}

impl __Registrable for Label {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        self.0.to_register()
    }

    fn __copy__(&self) -> Self {
        *self
    }
}

impl<'eval> From<__Register<'eval>> for Label {
    fn from(reg: __Register<'eval>) -> Self {
        Label(reg.downcast_i32())
    }
}
