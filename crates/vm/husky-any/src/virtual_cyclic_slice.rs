use super::*;
use cyclic_slice::CyclicSlice;

#[derive(Debug, Clone, PartialEq)]
pub struct DeprecatedVirtualCyclicSlice {
    pub data: CyclicSlice<'static, __RegularValue>,
}

impl std::ops::Deref for DeprecatedVirtualCyclicSlice {
    type Target = CyclicSlice<'static, __RegularValue>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for DeprecatedVirtualCyclicSlice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl __StaticInfo for DeprecatedVirtualCyclicSlice {
    type __StaticSelf = DeprecatedVirtualCyclicSlice;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "CyclicSlice<Any>".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        std::mem::transmute(self)
    }
}

// impl __Registrable for DeprecatedVirtualCyclicSlice {
//     unsafe fn __to_register(self) -> __RegularValue {
//         todo!()
//     }
// }
