use std::borrow::Cow;

use super::*;

impl<'temp, T> __StaticInfo for &'temp [T]
where
    T: __StaticInfo,
{
    type __StaticSelf = &'static [T::__StaticSelf];
    fn __static_typename() -> Cow<'static, str> {
        todo!()
    }
}

fn gen_iter<'temp, T>(slice: &'temp [T]) -> Box<dyn Iterator<Item = __RegularValue> + 'temp>
where
    T: __Registrable + 'static,
{
    Box::new(slice.iter().map(|t| __RegularValue::new_temp_ref::<T>(t)))
}
