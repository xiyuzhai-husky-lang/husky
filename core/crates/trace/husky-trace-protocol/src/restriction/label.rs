use super::*;

#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct Label(pub usize);

impl From<u8> for Label {
    fn from(v: u8) -> Self {
        Self(v as usize)
    }
}

impl __StaticInfo for Label {
    type __StaticSelf = Self;

    fn __static_type_name__() -> std::borrow::Cow<'static, str> {
        "Label".into()
    }
}

impl __Registrable for Label {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        todo!()
    }
}
