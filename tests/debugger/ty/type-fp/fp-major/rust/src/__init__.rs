use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __LinkageGroup)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __LinkageGroup)] = &[(
    __StaticLinkageKey::Routine {
        route: "fp_major::try_fp",
    },
    __LinkageGroup::Transfer(__ResolvedLinkage {
        dev_src: static_dev_src!(),
        wrapper: {
            unsafe fn __wrapper<'eval>(
                __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                __arguments: &mut [__Register<'eval>],
            ) -> __Register<'eval> {
                try_fp().to_register()
            }
            __wrapper
        },
        opt_fp: Some(try_fp as *const ()),
    }),
)];
