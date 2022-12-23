use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::TypeCall {
            ty: "test_qualifier_to_binding_major::A",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let x: i32 = __arguments[0].downcast_i32();
                    __Register::new_box::<A>(A::__call__(x), &__registration__::__A_VTABLE)
                }
                __wrapper
            },
            some base A::__call__ as fn(i32) -> A
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "test_qualifier_to_binding_major::A",
            field_ident: "x",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            A,
            __registration__::__A_VTABLE,
            i32,
            __registration__::__I32_VTABLE,
            x
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "test_qualifier_to_binding_major::B",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let x: i32 = __arguments[0].downcast_i32();
                    let a: A = unsafe { __arb_ref(&__arguments[1]) }.downcast_move(&__registration__::__A_VTABLE);
                    __Register::new_box::<B>(B::__call__(x, a), &__registration__::__B_VTABLE)
                }
                __wrapper
            },
            some base B::__call__ as fn(i32, A) -> B
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "test_qualifier_to_binding_major::B",
            field_ident: "x",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            B,
            __registration__::__B_VTABLE,
            i32,
            __registration__::__I32_VTABLE,
            x
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "test_qualifier_to_binding_major::B",
            field_ident: "a",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            BoxNonCopyable,
            B,
            __registration__::__B_VTABLE,
            A,
            __registration__::__A_VTABLE,
            a
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "test_qualifier_to_binding_major::B",
            field_ident: "y",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            B,
            __registration__::__B_VTABLE,
            i32,
            __registration__::__I32_VTABLE,
            y
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_qualifier_to_binding_major::take_copyable_eval_ref",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let x: &'eval i32 = __arguments[0].downcast_eval_ref(&__registration__::__I32_VTABLE);
                    take_copyable_eval_ref(x).to_register()
                }
                __wrapper
            },
            some base take_copyable_eval_ref as fn(&'static i32) -> i32
        ),
    ),
];
