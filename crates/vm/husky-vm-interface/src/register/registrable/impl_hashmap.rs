use super::*;
use std::collections::HashMap;

impl<K, V> __StaticInfo for HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + __StaticInfo,
    V: __StaticInfo,
    K::__StaticSelf: std::hash::Hash,
    <K as __StaticInfo>::__StaticSelf: Eq + std::hash::Hash,
{
    type __StaticSelf = HashMap<K::__StaticSelf, V::__StaticSelf>;
    fn __static_typename() -> std::borrow::Cow<'static, str> {
        format!(
            "HashMap<{}, {}>",
            K::__static_typename(),
            V::__static_typename()
        )
        .into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        std::mem::transmute(self)
    }
}

// impl<K, V> __Registrable for HashMap<K, V>
// where
//     K: __Registrable + std::cmp::Eq + std::hash::Hash,
//     V: __Registrable,
//     K::__StaticSelf: std::hash::Hash,
//     <K as __StaticInfo>::__StaticSelf: Eq,
// {
//     unsafe fn __to_register__(self) -> __RegularValue {
//         todo!()
//     }

//     fn __copy__(&self) -> Self {
//         panic!()
//     }
// }
