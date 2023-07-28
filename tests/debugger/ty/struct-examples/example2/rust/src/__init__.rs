use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __LinkageGroup)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __LinkageGroup)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "example2::f1",
        },
        __LinkageGroup::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    __Register::new_box(f1(), &__registration__::__A_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(f1 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "example2::A::get_x",
        },
        __LinkageGroup::Transfer(__ResolvedLinkage {
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
            route: "example2::A::get_x_plus_constant",
        },
        __LinkageGroup::Transfer(__ResolvedLinkage {
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
            route: "example2::A::get_x_squared",
        },
        __LinkageGroup::Transfer(__ResolvedLinkage {
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
            route: "example2::g1",
        },
        __LinkageGroup::Transfer(__ResolvedLinkage {
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
        __StaticLinkageKey::TypeCall { ty: "example2::A" },
        __LinkageGroup::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let x: i32 = __arguments[0].downcast_i32();
                    let y: i32 = todo!();
                    __Register::new_box(
                        A::__call__(x, /* keyword arguments */ y),
                        &__registration__::__A_VTABLE,
                    )
                }
                __wrapper
            },
            opt_fp: Some(A::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "example2::A",
            field_ident: "x",
        },
        eager_field_linkage!(
            A,
            __registration__::__A_VTABLE,
            __registration__::__I32_VTABLE,
            x,
            direct
        ),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "example2::A",
            field_ident: "y",
        },
        eager_field_linkage!(
            A,
            __registration__::__A_VTABLE,
            __registration__::__I32_VTABLE,
            y,
            direct
        ),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "example2::A",
            field_ident: "z",
        },
        eager_field_linkage!(
            A,
            __registration__::__A_VTABLE,
            __registration__::__I32_VTABLE,
            z,
            direct
        ),
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "example2::A::w",
        },
        lazy_field_linkage!(
            A,
            __registration__::__A_VTABLE,
            __registration__::__I32_VTABLE,
            w
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "example2::f3",
        },
        __LinkageGroup::Transfer(__ResolvedLinkage {
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
