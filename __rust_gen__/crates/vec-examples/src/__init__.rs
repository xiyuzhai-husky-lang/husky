
use crate::*;
use __husky_root::__init_utils::*;

pub static LINKAGES : &[(__StaticLinkageKey, __Linkage)]= &[

    (
        __StaticLinkageKey::Routine {
            routine: "vec_examples::f"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    f()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "[]i32"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __variadics = todo!();
                __TempValue::OwnedEval(__OwnedValue::new(
                    Vec::<i32>::__call__(__variadics)
                    ))
            }
            __wrapper
        }),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::ilen"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &Vec<i32> = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.ilen()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::push"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<i32> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                let element: i32 = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::Copyable(
                    __this.push(element)
                    .__take_copyable_dyn())
            }
            __wrapper
        }),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::popx"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<i32> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                __TempValue::Copyable(
                    __this.popx()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::firstx"
        },
        method_elem_linkage!(Vec<i32>, firstx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::lastx"
        },
        method_elem_linkage!(Vec<i32>, lastx)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::cyclic_slice"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &'eval Vec<i32> = __arguments[0].downcast_eval_ref();
                let start: i32 = __arguments[1].downcast_copy();
                let end: i32 = __arguments[2].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    __this.cyclic_slice(start, end)
                    ))
            }
            __wrapper
        }),

    ),
    (

        __StaticLinkageKey::Index {
            opd_tys: &["[]i32", "i32"],
        },
        index_linkage!(Vec<i32>)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::push"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &mut Vec<i32> = unsafe { __arb_ref(&__arguments[0]) }.downcast_mut();
                let element: i32 = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::Copyable(
                    __this.push(element)
                    .__take_copyable_dyn())
            }
            __wrapper
        }),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::ilen"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &Vec<i32> = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.ilen()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "vec_examples::change_element"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    change_element()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),
    ),
];