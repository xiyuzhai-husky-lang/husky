use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "test_unveil_return::some_i32",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    some_i32().to_register()
                }
                __wrapper
            },
            some base some_i32 as fn() -> Option<i32>
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_unveil_return::try_unveil",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    try_unveil().to_register()
                }
                __wrapper
            },
            some base try_unveil as fn() -> Option<i32>
        ),
    ),
];
