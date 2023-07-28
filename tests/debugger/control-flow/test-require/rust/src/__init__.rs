use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __LinkageGroup)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __LinkageGroup)] = &[(
    __StaticLinkageKey::Routine {
        route: "test_require::try_require_in_func",
    },
    __LinkageGroup::Transfer(__ResolvedLinkage {
        dev_src: static_dev_src!(),
        wrapper: {
            unsafe fn __wrapper<'eval>(
                __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                __arguments: &mut [__Register<'eval>],
            ) -> __Register<'eval> {
                __Register::new_box(try_require_in_func(), &__registration__::__I32_VTABLE)
            }
            __wrapper
        },
        opt_fp: Some(try_require_in_func as *const ()),
    }),
)];
