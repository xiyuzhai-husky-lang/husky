use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[(
    __StaticLinkageKey::Routine {
        route: "horizontal_extend::horizontal_extend",
    },
    __Linkage::Transfer(__ResolvedLinkage {
        dev_src: static_dev_src!(),
        wrapper: {
            unsafe fn __wrapper<'eval>(
                __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                __arguments: &mut [__Register<'eval>],
            ) -> __Register<'eval> {
                let a: u32 = __arguments[0].downcast_r32();
                let x: u32 = __arguments[1].downcast_r32();
                horizontal_extend(a, x).to_register()
            }
            __wrapper
        },
        opt_fp: Some(horizontal_extend as *const ()),
    }),
)];
