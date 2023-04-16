use super::*;
use husky_ethereal_term::make_route;
use husky_print_utils::msg_once;
use husky_word::RootBuiltinIdent;
use thin_vec::thin_vec;

impl<'a, T> __StaticInfo for Vec<T>
where
    T: __StaticInfo,
{
    type __StaticSelf = Vec<T::__StaticSelf>;

    fn __static_typename() -> Cow<'static, str> {
        format!("[]{}", T::__static_typename()).into()
    }
}
