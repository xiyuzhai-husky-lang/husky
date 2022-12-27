use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[(
    __StaticLinkageKey::Routine {
        route: "test_misc_horizontal_extend::horizontal_extend",
    },
    transfer_linkage!(
        {
            unsafe fn __wrapper<'eval>(
                __arguments: &mut [__Register<'eval>],
                __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            ) -> __Register<'eval> {
                let a: u32 = __arguments[0].downcast_r32();
                let x: u32 = __arguments[1].downcast_r32();
                horizontal_extend(a, x).to_register()
            }
            __wrapper
        },
        some base horizontal_extend as fn(u32, u32) -> u32
    ),
)];
