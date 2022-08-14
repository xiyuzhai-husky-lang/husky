use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example1::f1",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    __Register::new_box::<A>(f1(), &__registration__::__A_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(f1 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example1::A::get_x",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &A =
                        __arguments[0].downcast_temp_ref(&__registration__::__A_VTABLE);
                    __this.get_x().to_register()
                }
                __wrapper
            },
            opt_fp: Some(A::get_x as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example1::A::get_x_plus_constant",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &A =
                        __arguments[0].downcast_temp_ref(&__registration__::__A_VTABLE);
                    __this.get_x_plus_constant().to_register()
                }
                __wrapper
            },
            opt_fp: Some(A::get_x_plus_constant as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example1::A::get_x_squared",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let __this: &A =
                        __arguments[0].downcast_temp_ref(&__registration__::__A_VTABLE);
                    __this.get_x_squared().to_register()
                }
                __wrapper
            },
            opt_fp: Some(A::get_x_squared as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example1::g1",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    g1().to_register()
                }
                __wrapper
            },
            opt_fp: Some(g1 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example1::f2",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    __Register::new_box::<A>(f2(), &__registration__::__A_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(f2 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "test_struct_example1::A",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let x: i32 = __arguments[0].downcast_i32();
                    __Register::new_box::<A>(A::__call__(x), &__registration__::__A_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(A::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "test_struct_example1::A",
            field_ident: "x",
        },
        eager_mut_field_linkage!(
            A,
            __registration__::__A_VTABLE,
            __registration__::__I32_VTABLE,
            x,
            direct
        ),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "test_struct_example1::A::y",
        },
        lazy_field_linkage!(
            A,
            __registration__::__A_VTABLE,
            __registration__::__I32_VTABLE,
            y
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example1::f3",
        },
        __Linkage::Transfer(__LinkageFp {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    f3().to_register()
                }
                __wrapper
            },
            opt_fp: Some(f3 as *const ()),
        }),
    ),
];
