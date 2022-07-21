
use crate::*;
use __husky_root::__init_utils::*;

pub static LINKAGES : &[(__StaticLinkageKey, __Linkage)]= &[

    (
        __StaticLinkageKey::Routine {
            routine: "test_misc_horizontal_extend::horizontal_extend"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let a: u32 = __arguments[0].downcast_copy();
                let x: u32 = __arguments[1].downcast_copy();
                __TempValue::Copyable(
                    horizontal_extend(a, x)
                .__take_copyable_dyn())
            }
            __wrapper
        }, some horizontal_extend),    ),
];