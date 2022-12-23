use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::for_loop1",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    for_loop1().to_register()
                }
                __wrapper
            },
            some base for_loop1 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::for_loop2",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    for_loop2().to_register()
                }
                __wrapper
            },
            some base for_loop2 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::for_loop3",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    for_loop3().to_register()
                }
                __wrapper
            },
            some base for_loop3 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::for_loop4",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    for_loop4().to_register()
                }
                __wrapper
            },
            some base for_loop4 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::forext_loop1",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    forext_loop1().to_register()
                }
                __wrapper
            },
            some base forext_loop1 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::forext_loop2",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    forext_loop2().to_register()
                }
                __wrapper
            },
            some base forext_loop2 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::forext_loop3",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    forext_loop3().to_register()
                }
                __wrapper
            },
            some base forext_loop3 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::while_loop1",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    while_loop1().to_register()
                }
                __wrapper
            },
            some base while_loop1 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::while_loop2",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    while_loop2().to_register()
                }
                __wrapper
            },
            some base while_loop2 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::while_loop3",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    while_loop3().to_register()
                }
                __wrapper
            },
            some base while_loop3 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::while_loop4",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    while_loop4().to_register()
                }
                __wrapper
            },
            some base while_loop4 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::do_while_loop1",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    do_while_loop1().to_register()
                }
                __wrapper
            },
            some base do_while_loop1 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::do_while_loop2",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    do_while_loop2().to_register()
                }
                __wrapper
            },
            some base do_while_loop2 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::do_while_loop3",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    do_while_loop3().to_register()
                }
                __wrapper
            },
            some base do_while_loop3 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_proc_loop1::do_while_loop4",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    do_while_loop4().to_register()
                }
                __wrapper
            },
            some base do_while_loop4 as fn() -> i32
        ),
    ),
];
