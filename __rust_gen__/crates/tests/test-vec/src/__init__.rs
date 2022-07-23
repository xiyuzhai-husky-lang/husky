use crate::*;
use __husky::__init_utils::*;

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::Routine {
            routine: "test_vec::f",
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
        }, some f),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_vec::change_element",
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
        }, some change_element),
    ),
    (
        __StaticLinkageKey::TypeCall { ty: "[]i32" },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __variadics = 
                    __arguments[0..]
                        .iter_mut()
                        .map(|v|v.downcast_copy())
                        .collect();
                __TempValue::OwnedEval(__OwnedValue::new(
                    Vec::<i32>::__call__(__variadics)
                ))
            }
            __wrapper
        }, some Vec::<i32>::__call__),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::ilen",
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
        }, some Vec::<i32>::ilen),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::push",
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
        }, some Vec::<i32>::push),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::popx",
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
        }, some Vec::<i32>::popx),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::firstx",
        },
        method_elem_linkage!(Vec<i32>, firstx),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::lastx",
        },
        method_elem_linkage!(Vec<i32>, lastx),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::cyclic_slice",
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
        }, some Vec::<i32>::cyclic_slice),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]i32", "i32"],
        },
        index_linkage!(Vec<i32>),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::push",
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
        }, some Vec::<i32>::push),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "Vec<i32>::ilen",
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
        }, some Vec::<i32>::ilen),
    ),
];
