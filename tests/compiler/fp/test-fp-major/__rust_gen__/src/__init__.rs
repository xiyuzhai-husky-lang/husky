use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[(
    __StaticLinkageKey::Routine {
        route: "test_fp_major::try_fp",
    },
    transfer_linkage!(
        {
            unsafe fn __wrapper<'eval>(
                __arguments: &mut [__Register<'eval>],
                __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            ) -> __Register<'eval> {
                try_fp(__opt_ctx.unwrap()).to_register()
            }
            __wrapper
        },
        some ctx try_fp as fn(&dyn __EvalContext<'static>) -> i32
    ),
)];
