
use crate::*;
use __husky_root::__init_utils::*;

pub static LINKAGES : &[(__StaticLinkageKey, __Linkage)]= &[

    (
        __StaticLinkageKey::Routine {
            routine: "test_arithmetic_major::add"
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
        }, some add),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_arithmetic_major::bitor_assign"
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
        }, some bitor_assign),    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_arithmetic_major::bitand_assign"
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
        }, some bitand_assign),    ),
];