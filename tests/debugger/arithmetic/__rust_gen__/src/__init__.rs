use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "arithmetic::add",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let x: i32 = __arguments[0].downcast_i32();
                    let y: i32 = __arguments[1].downcast_i32();
                    add(x, y).to_register()
                }
                __wrapper
            },
            opt_fp: Some(add as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "arithmetic::bitor_assign",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    bitor_assign().to_register()
                }
                __wrapper
            },
            opt_fp: Some(bitor_assign as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "arithmetic::bitand_assign",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    bitand_assign().to_register()
                }
                __wrapper
            },
            opt_fp: Some(bitand_assign as *const ()),
        }),
    ),
];
