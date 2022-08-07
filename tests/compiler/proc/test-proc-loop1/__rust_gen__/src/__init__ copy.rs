
use crate::*;
use __husky::init::*;

pub static LINKAGES : &[(__LinkageKind, &'static str, __Linkage)]= &[
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::for_loop1",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    for_loop1().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(for_loop1 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::for_loop2",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    for_loop2().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(for_loop2 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::for_loop3",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    for_loop3().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(for_loop3 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::for_loop4",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    for_loop4().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(for_loop4 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::forext_loop1",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    forext_loop1().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(forext_loop1 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::forext_loop2",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    forext_loop2().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(forext_loop2 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::forext_loop3",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    forext_loop3().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(forext_loop3 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::while_loop1",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    while_loop1().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(while_loop1 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::while_loop2",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    while_loop2().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(while_loop2 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::while_loop3",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    while_loop3().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(while_loop3 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::while_loop4",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    while_loop4().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(while_loop4 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::do_while_loop1",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    do_while_loop1().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(do_while_loop1 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::do_while_loop2",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    do_while_loop2().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(do_while_loop2 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::do_while_loop3",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    do_while_loop3().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(do_while_loop3 as *const ())
        }
    ),
    (
        __LinkageKind::Transfer,
        "test_proc_loop1::do_while_loop4",
        __Linkage {
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register],
                ) -> __Register {
                    do_while_loop4().__to_register()
                }
                __wrapper
            },
            opt_fp: Some(do_while_loop4 as *const ())
        }
    ),
];