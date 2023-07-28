use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __LinkageGroup)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __LinkageGroup)] = &[
    (
        __StaticLinkageKey::TypeCall { ty: "major::A" },
        __LinkageGroup::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let x: i32 = __arguments[0].downcast_i32();
                    __Register::new_box(A::__call__(x), &__registration__::__A_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(A::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "major::A",
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
        __StaticLinkageKey::TypeCall { ty: "major::B" },
        __LinkageGroup::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let x: i32 = __arguments[0].downcast_i32();
                    let a: A = unsafe { __arb_ref(&__arguments[1]) }
                        .downcast_move(&__registration__::__A_VTABLE);
                    __Register::new_box(B::__call__(x, a), &__registration__::__B_VTABLE)
                }
                __wrapper
            },
            opt_fp: Some(B::__call__ as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "major::B",
            field_ident: "x",
        },
        eager_field_linkage!(
            B,
            __registration__::__B_VTABLE,
            __registration__::__I32_VTABLE,
            x,
            direct
        ),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "major::B",
            field_ident: "a",
        },
        eager_field_linkage!(
            B,
            __registration__::__B_VTABLE,
            __registration__::__A_VTABLE,
            a,
            invalid
        ),
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "major::B",
            field_ident: "y",
        },
        eager_field_linkage!(
            B,
            __registration__::__B_VTABLE,
            __registration__::__I32_VTABLE,
            y,
            direct
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "major::take_copyable_eval_ref",
        },
        __LinkageGroup::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    let x: &'eval i32 =
                        __arguments[0].downcast_eval_ref(&__registration__::__I32_VTABLE);
                    take_copyable_eval_ref(x).to_register()
                }
                __wrapper
            },
            opt_fp: Some(take_copyable_eval_ref as *const ()),
        }),
    ),
];
