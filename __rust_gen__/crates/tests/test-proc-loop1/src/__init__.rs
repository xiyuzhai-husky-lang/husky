use crate::*;
use __husky::__init_utils::*;

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::for_loop1",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    for_loop1()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some for_loop1),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::for_loop2",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    for_loop2()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some for_loop2),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::for_loop3",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    for_loop3()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some for_loop3),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::for_loop4",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    for_loop4()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some for_loop4),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::forext_loop1",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    forext_loop1()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some forext_loop1),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::forext_loop2",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    forext_loop2()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some forext_loop2),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::forext_loop3",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    forext_loop3()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some forext_loop3),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::while_loop1",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    while_loop1()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some while_loop1),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::while_loop2",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    while_loop2()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some while_loop2),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::while_loop3",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    while_loop3()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some while_loop3),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::while_loop4",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    while_loop4()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some while_loop4),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::do_while_loop1",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    do_while_loop1()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some do_while_loop1),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::do_while_loop2",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    do_while_loop2()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some do_while_loop2),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::do_while_loop3",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    do_while_loop3()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some do_while_loop3),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "test_proc_loop1::do_while_loop4",
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    do_while_loop4()
                .__take_copyable_dyn())
            }
            __wrapper
        }, some do_while_loop4),
    ),
];
