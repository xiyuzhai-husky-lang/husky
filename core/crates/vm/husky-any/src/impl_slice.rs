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

fn gen_iter<'temp, 'eval: 'temp, T>(
    slice: &'temp [T],
) -> Box<dyn Iterator<Item = __Register<'eval>> + 'temp>
where
    T: __Registrable + 'eval,
{
    Box::new(slice.iter().map(|t| __Register::new_temp_ref(t)))
}
