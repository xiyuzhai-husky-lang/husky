
use crate::*;
use __husky_root::__init_utils::*;

pub static LINKAGES : &[(__StaticLinkageKey, __Linkage)]= &[

    (
        __StaticLinkageKey::Routine {
            routine: "arithmetic::add"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let x: i32 = __arguments[0].downcast_copy();
                let y: i32 = __arguments[1].downcast_copy();
                __TempValue::Copyable(
                    add(x, y)
                    .__take_copyable_dyn())
            }
            __wrapper
        }),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "arithmetic::bitor_assign"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    bitor_assign()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "arithmetic::bitand_assign"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    bitand_assign()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),
    ),
];