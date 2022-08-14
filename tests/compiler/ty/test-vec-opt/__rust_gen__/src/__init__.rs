use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "test_vec_opt::try_vec_opt",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    __Register::new_box::<Vec<Option<i32>>>(
                        try_vec_opt(),
                        &__registration__::__VEC_OPTION_I_32_VTABLE,
                    )
                }
                __wrapper
            },
            opt_fp: Some(try_vec_opt as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "Vec<?i32>::ilen",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &Vec<Option<i32>> = __arguments[0]
                        .downcast_temp_ref(&__registration__::__VEC_OPTION_I_32_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<Option<i32>>::ilen as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall { ty: "[]?i32" },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __variadics = __arguments[0..]
                        .iter_mut()
                        .map(|v| v.downcast_opt_i32())
                        .collect();
                    __Register::new_box::<Vec<Option<i32>>>(
                        Vec::<Option<i32>>::__call__(__variadics),
                        &__registration__::__VEC_OPTION_I_32_VTABLE,
                    )
                }
                __wrapper
            },
            opt_fp: Some(Vec::<Option<i32>>::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]?i32", "i32"],
        },
        index_linkage!(
            Vec<Option<i32>>,
            __registration__::__VEC_OPTION_I_32_VTABLE,
            __registration__::__I32_VTABLE,
            box,
            mutable
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "Vec<?i32>::push",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &mut Vec<Option<i32>> = unsafe { __arb_ref(&__arguments[0]) }
                        .downcast_temp_mut(&__registration__::__VEC_OPTION_I_32_VTABLE);
                    let element: Option<i32> = unsafe { __arb_ref(&__arguments[1]) }
                        .downcast_move(&__registration__::__I32_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            opt_fp: Some(Vec::<Option<i32>>::push as *const ()),
        }),
    ),
];
