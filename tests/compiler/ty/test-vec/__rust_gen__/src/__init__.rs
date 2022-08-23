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
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> { /*haha*/
                    f().to_register()
                }
                __wrapper
            },
            some f as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_vec::change_element",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> { /*haha*/
                    change_element().to_register()
                }
                __wrapper
            },
            some change_element as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_vec::test_pop_with",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> { /*haha*/
                    test_pop_with().to_register()
                }
                __wrapper
            },
            some test_pop_with as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "Vec<i32>::ilen",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> { /*haha*/
                    let __this: &Vec<i32> = __arguments[0].downcast_temp_ref(&__registration__::__VEC_I_32_VTABLE);
                    __this.ilen().to_register()
                }
                __wrapper
            },
            some Vec::<i32>::ilen as fn(&'static Vec<i32>) -> i32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall { ty: "[]i32" },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> { /*haha*/
                    let __variadics =
                        __arguments[0..]
                            .iter_mut()
                            .map(|v|v.downcast_i32())
                            .collect();
                    __Register::new_box::<Vec<i32>>(Vec::<i32>::__call__(__variadics), &__registration__::__VEC_I_32_VTABLE)
                }
                __wrapper
            },
            some Vec::<i32>::__call__ as fn(Vec<i32>) -> Vec<i32>
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
            route: "Vec<i32>::push",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> { /*haha*/
                    let __this: &mut Vec<i32> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_I_32_VTABLE);
                    let element: i32 = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__I32_VTABLE);
                    __this.push(element).to_register()
                }
                __wrapper
            },
            some Vec::<i32>::push as fn(&'static mut Vec<i32>, i32) -> ()
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_vec::score",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> { /*haha*/
                    let a: i32 = __arguments[0].downcast_i32();
                    score(a).to_register()
                }
                __wrapper
            },
            some score as fn(i32) -> Option<f32>
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "Vec<i32>::pop_with_largest_opt_f32",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> { /*haha*/
                    let __this: &mut Vec<i32> = unsafe { __arb_ref(&__arguments[0]) }.downcast_temp_mut(&__registration__::__VEC_I_32_VTABLE);
                    let f: fn(i32)->Option<f32> = std::mem::transmute(__arguments[1]
                        .downcast_temp_ref::<__VirtualFunction>(&__registration__::__VIRTUAL_FUNCTION_VTABLE)
                        .fp());
                    __this.pop_with_largest_opt_f32_copyable(f).to_register()
                }
                __wrapper
            },
            some Vec::<i32>::pop_with_largest_opt_f32_copyable as fn(&'static mut Vec<i32>, fn(i32)->Option<f32>) -> Option<i32>
        ),
    ),
];
