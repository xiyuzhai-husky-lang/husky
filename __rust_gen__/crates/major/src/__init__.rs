
use crate::*;
use __husky_root::__init_utils::*;

pub static LINKAGES : &[(__StaticLinkageKey, __Linkage)]= &[

    (
        __StaticLinkageKey::TypeCall {
            ty: "major::A"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let x: i32 = __arguments[0].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    A::__call__(x)
                    ))
            }
            __wrapper
        }),

    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "major::A",
            field_ident: "x",
        },
        eager_field_linkage!(A, x)
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "major::B"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let x: i32 = __arguments[0].downcast_copy();
                let a: A = unsafe { __arb_ref(&__arguments[1]) }.downcast_move();
                __TempValue::OwnedEval(__OwnedValue::new(
                    B::__call__(x, a)
                    ))
            }
            __wrapper
        }),

    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "major::B",
            field_ident: "x",
        },
        eager_field_linkage!(B, x)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "major::B",
            field_ident: "a",
        },
        eager_field_linkage!(B, a)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "major::B",
            field_ident: "y",
        },
        eager_field_linkage!(B, y)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "major::take_copyable_eval_ref"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let x: &'eval i32 = __arguments[0].downcast_eval_ref();
                __TempValue::Copyable(
                    take_copyable_eval_ref(x)
                    .__take_copyable_dyn())
            }
            __wrapper
        }),

    ),
];