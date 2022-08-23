use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[(
    __StaticLinkageKey::Routine {
        route: "test_proc_loop2::for_loop4",
    },
    transfer_linkage!(
        {
            unsafe fn __wrapper<'eval>(
                __arguments: &mut [__Register<'eval>],
                __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            ) -> __Register<'eval> {
                for_loop4().to_register()
            }
            __wrapper
        },
        some base for_loop4 as fn() -> i32
    ),
)];
