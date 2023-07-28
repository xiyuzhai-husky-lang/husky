use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __LinkageGroup)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __LinkageGroup)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "branch_in_loop1::f1",
        },
        __LinkageGroup::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    f1().to_register()
                }
                __wrapper
            },
            opt_fp: Some(f1 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "branch_in_loop1::f2",
        },
        __LinkageGroup::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    f2().to_register()
                }
                __wrapper
            },
            opt_fp: Some(f2 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "branch_in_loop1::f3",
        },
        __LinkageGroup::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    f3().to_register()
                }
                __wrapper
            },
            opt_fp: Some(f3 as *const ()),
        }),
    ),
];
