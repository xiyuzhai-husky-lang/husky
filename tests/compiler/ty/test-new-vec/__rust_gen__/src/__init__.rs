use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "Vec<i32>::ilen",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &Vec<i32> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_I_32_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            some base Vec::<i32>::ilen as fn(&'static Vec<i32>) -> i32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall { ty: "[]i32" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __variadics =
                        __arguments[0..]
                            .iter_mut()
                            .map(|v|v.downcast_i32())
                            .collect();
                    __Register::new_box::<Vec<i32>>(Vec::<i32>::__call__(__variadics), &__registration__::__VEC_I_32_VTABLE)
                }
                __wrapper
            },
            some base Vec::<i32>::__call__ as fn(Vec<i32>) -> Vec<i32>
        ),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]i32", "i32"],
        },
        index_linkage!(
            mutable,
            Intrinsic,
            Direct,
            Vec<i32>,
            __registration__::__VEC_I_32_VTABLE,
            i32,
            __registration__::__I32_VTABLE
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_new_vec::try_vec",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    try_vec().to_register()
                }
                __wrapper
            },
            some base try_vec as fn() -> i32
        ),
    ),
];
