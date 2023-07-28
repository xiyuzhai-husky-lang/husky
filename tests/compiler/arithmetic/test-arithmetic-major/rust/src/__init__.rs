use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __LinkageGroup)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __LinkageGroup)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "test_arithmetic_major::add",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let x: i32 = __arguments[0].downcast_i32();
                    let y: i32 = __arguments[1].downcast_i32();
                    add(x, y).to_register()
                }
                __wrapper
            },
            some base add as fn(i32, i32) -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_arithmetic_major::bitor_assign",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    bitor_assign().to_register()
                }
                __wrapper
            },
            some base bitor_assign as fn() -> bool
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_arithmetic_major::bitand_assign",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    bitand_assign().to_register()
                }
                __wrapper
            },
            some base bitand_assign as fn() -> bool
        ),
    ),
];
