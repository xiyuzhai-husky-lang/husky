use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __LinkageGroup)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __LinkageGroup)] = &[
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "control_flow::branch_in_func_feature",
        },
        feature_linkage!(branch_in_func_feature, __registration__::__I32_VTABLE),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "control_flow::match_in_func_feature",
        },
        feature_linkage!(match_in_func_feature, __registration__::__I32_VTABLE),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "control_flow::branch_in_func_function",
        },
        __LinkageGroup::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    branch_in_func_function().to_register()
                }
                __wrapper
            },
            opt_fp: Some(branch_in_func_function as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "control_flow::match_in_func_function",
        },
        __LinkageGroup::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    match_in_func_function().to_register()
                }
                __wrapper
            },
            opt_fp: Some(match_in_func_function as *const ()),
        }),
    ),
];
