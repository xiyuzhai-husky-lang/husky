use crate::{ugly::*, *};
use husky_ki_repr_interface::ugly::__KiArgumentReprInterface;

#[test]
fn fn_linket_impl_works() {
    use crate::*;

    type __LinketImpl = StandardLinketImpl;
    type __DevEvalContext = DevEvalContext<__LinketImpl>;

    fn_linket_impl!(|| ());
    fn_linket_impl!(|a: &str| ());
    fn_linket_impl!(vm only |a: Vec<&str>| ());
    fn_linket_impl!(|a: Vec<&'static str>| ());
}

#[test]
fn unveil_fn_linket_impl_works() {
    use crate::*;

    type __LinketImpl = StandardLinketImpl;
    type __DevEvalContext = DevEvalContext<__LinketImpl>;
    struct __DevsoulInterface;

    unveil_fn_linket_impl!(|_: i32, ()| -> std::ops::ControlFlow<i32, i32> {
        std::ops::ControlFlow::Continue(0)
    });
}
