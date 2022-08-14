use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "test_vec::f",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    f().to_register()
                }
                __wrapper
            },
            opt_fp: Some(f as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_vec::change_element",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    change_element().to_register()
                }
                __wrapper
            },
            opt_fp: Some(change_element as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_vec::test_pop_with",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    test_pop_with().to_register()
                }
                __wrapper
            },
            opt_fp: Some(test_pop_with as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "Vec<i32>::ilen",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &Vec<i32> =
                        __arguments[0].downcast_temp_ref(&__registration__::__VEC_I_32_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<i32>::ilen as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall { ty: "[]i32" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __variadics = __arguments[0..]
                        .iter_mut()
                        .map(|v| v.downcast_i32())
                        .collect();
                    __Register::new_box(
                        Vec::<i32>::__call__(__variadics),
                        &__registration__::__VEC_I_32_VTABLE,
                    )
                }
                __wrapper
            },
            opt_fp: Some(Vec::<i32>::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]i32", "i32"],
        },
        index_linkage!(
            Vec<i32>,
            __registration__::__VEC_I_32_VTABLE,
            __registration__::__I32_VTABLE,
            direct,
            mutable
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "Vec<i32>::push",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &mut Vec<i32> = unsafe { __arb_ref(&__arguments[0]) }
                        .downcast_temp_mut(&__registration__::__VEC_I_32_VTABLE);
                    let element: i32 = unsafe { __arb_ref(&__arguments[1]) }
                        .downcast_move(&__registration__::__I32_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<i32>::push as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_vec::score",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let a: i32 = __arguments[0].downcast_i32();
                    __Register::new_opt_box(score(a), &__registration__::__F32_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(score as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "Vec<i32>::pop_with_largest_opt_f32",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &mut Vec<i32> = unsafe { __arb_ref(&__arguments[0]) }
                        .downcast_temp_mut(&__registration__::__VEC_I_32_VTABLE);
                    let f: fn(i32) -> Option<f32> = std::mem::transmute(
                        __arguments[1]
                            .downcast_temp_ref::<__VirtualFunction>(
                                &__registration__::__VIRTUAL_FUNCTION_VTABLE,
                            )
                            .fp(),
                    );
                    __Register::new_opt_box(
                        __this.pop_with_largest_opt_f32_copyable(f),
                        &__registration__::__I32_VTABLE,
                    )
                }
                __wrapper
            },
            opt_fp: Some(Vec::<i32>::pop_with_largest_opt_f32_copyable as *const ()),
        }),
    ),
];
