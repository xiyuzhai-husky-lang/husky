use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __LinkageGroup)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __LinkageGroup)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_branch_in_loop1::f1",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    f1().to_register()
                }
                __wrapper
            },
            some base f1 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_branch_in_loop1::f2",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    f2().to_register()
                }
                __wrapper
            },
            some base f2 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_branch_in_loop1::f3",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    f3().to_register()
                }
                __wrapper
            },
            some base f3 as fn() -> i32
        ),
    ),
];
