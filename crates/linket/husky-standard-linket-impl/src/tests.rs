use crate::{ugly::*, *};
use husky_ki_repr_interface::ugly::__KiArgumentReprInterface;

#[test]
fn fn_linket_impl_works() {
    use crate::IsFnLinketImplSource;
    use husky_devsoul_interface::ugly::*;

    type __LinketImpl = StandardLinketImpl;
    type __DevEvalContext = DevEvalContext<__LinketImpl>;
    struct __DevsoulInterface;
    impl IsDevsoulInterface for __DevsoulInterface {
        type LinketImpl = __LinketImpl;

        fn dev_eval_context() -> DevEvalContext<Self::LinketImpl> {
            todo!()
        }

        unsafe fn set_dev_eval_context(ctx: DevEvalContext<Self::LinketImpl>) {
            todo!()
        }

        unsafe fn unset_dev_eval_context() {
            todo!()
        }
    }

    fn_linket_impl!(|| ());
}

#[test]
fn unveil_fn_linket_impl_works() {
    use crate::{IsFnLinketImplSource, IsUnveilFnLinketImplSource};
    use husky_devsoul_interface::ugly::*;

    type __LinketImpl = StandardLinketImpl;
    type __DevEvalContext = DevEvalContext<__LinketImpl>;
    struct __DevsoulInterface;
    impl IsDevsoulInterface for __DevsoulInterface {
        type LinketImpl = __LinketImpl;

        fn dev_eval_context() -> DevEvalContext<Self::LinketImpl> {
            todo!()
        }

        unsafe fn set_dev_eval_context(ctx: DevEvalContext<Self::LinketImpl>) {
            todo!()
        }

        unsafe fn unset_dev_eval_context() {
            todo!()
        }
    }

    unveil_fn_linket_impl!(|_: i32, ()| -> std::ops::ControlFlow<i32, i32> {
        std::ops::ControlFlow::Continue(0)
    });
}
