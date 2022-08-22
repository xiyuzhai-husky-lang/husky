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
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    some_i32().to_register()
                }
                __wrapper
            },
            opt_fp: Some(some_i32 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_unveil_return::try_unveil",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    try_unveil().to_register()
                }
                __wrapper
            },
            opt_fp: Some(try_unveil as *const ()),
        }),
    ),
];
