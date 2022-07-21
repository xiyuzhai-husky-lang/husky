
use crate::*;
use __husky_root::__init_utils::*;

pub static LINKAGES : &[(__StaticLinkageKey, __Linkage)]= &[

    (
        __StaticLinkageKey::TypeCall {
            ty: "example1::A"
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
            this_ty: "example1::A",
            field_ident: "x",
        },
        eager_mut_field_linkage!(A, x)
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "example1::A::y",
        },
        lazy_field_linkage!(A, y)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "example1::f1"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::OwnedEval(__OwnedValue::new(
                    f1()
                    ))
            }
            __wrapper
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "example1::f2"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::OwnedEval(__OwnedValue::new(
                    f2()
                    ))
            }
            __wrapper
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "example1::f3"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    f3()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "example1::A::get_x"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &A = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.get_x()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "example1::g1"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    g1()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),

    ),
];