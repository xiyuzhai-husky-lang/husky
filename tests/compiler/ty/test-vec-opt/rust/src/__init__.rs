use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __LinkageGroup)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __LinkageGroup)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "test_vec_opt::try_vec_opt",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    __Register::new_box::<Vec<Option<i32>>>(try_vec_opt(), &__registration__::__VEC_OPTION_I_32_VTABLE)
                }
                __wrapper
            },
            some base try_vec_opt as fn() -> Vec<Option<i32>>
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "Vec<?i32>::ilen",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &Vec<Option<i32>> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_OPTION_I_32_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            some base Vec::<Option<i32>>::ilen as fn(&'static Vec<Option<i32>>) -> i32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall { ty: "[]?i32" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __variadics =
                        __arguments[0..]
                            .iter_mut()
                            .map(|v|v.downcast_opt_i32())
                            .collect();
                    __Register::new_box::<Vec<Option<i32>>>(Vec::<Option<i32>>::__call__(__variadics), &__registration__::__VEC_OPTION_I_32_VTABLE)
                }
                __wrapper
            },
            some base Vec::<Option<i32>>::__call__ as fn(Vec<Option<i32>>) -> Vec<Option<i32>>
        ),
    ),
    (
        __StaticLinkageKey::Index {
            opd_tys: &["[]?i32", "i32"],
        },
        index_linkage!(
            mutable,
            Optional,
            Direct,
            Vec<Option<i32>>,
            __registration__::__VEC_OPTION_I_32_VTABLE,
            i32,
            __registration__::__I32_VTABLE
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "Vec<?i32>::push",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &mut Vec<Option<i32>> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_OPTION_I_32_VTABLE);
                    let element: Option<i32> = __arguments[1].downcast_opt_i32();
                    __this.push(element).to_register()
                }
                __wrapper
            },
            some base Vec::<Option<i32>>::push as fn(&'static mut Vec<Option<i32>>, Option<i32>) -> ()
        ),
    ),
];
