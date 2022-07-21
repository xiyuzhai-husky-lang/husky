use crate::*;
use __husky_root::__init_utils::*;

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_branch_in_loop1::f1",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    f1()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some f1),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_branch_in_loop1::f2",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    f2()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some f2),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_branch_in_loop1::f3",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    f3()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some f3),
    ),
];
