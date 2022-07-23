use crate::*;
use __husky_root::__init_utils::*;

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[(
    __StaticLinkageKey::Routine {
        routine: "test_fp_major::try_fp",
    },
    specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    try_fp()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some try_fp),
)];
