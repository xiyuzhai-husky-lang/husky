use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "loop1::for_loop1",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    for_loop1().to_register()
                }
                __wrapper
            },
            opt_fp: Some(for_loop1 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::for_loop2",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    for_loop2().to_register()
                }
                __wrapper
            },
            opt_fp: Some(for_loop2 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::for_loop3",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    for_loop3().to_register()
                }
                __wrapper
            },
            opt_fp: Some(for_loop3 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::for_loop4",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    for_loop4().to_register()
                }
                __wrapper
            },
            opt_fp: Some(for_loop4 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::forext_loop1",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    forext_loop1().to_register()
                }
                __wrapper
            },
            opt_fp: Some(forext_loop1 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::forext_loop2",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    forext_loop2().to_register()
                }
                __wrapper
            },
            opt_fp: Some(forext_loop2 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::forext_loop3",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    forext_loop3().to_register()
                }
                __wrapper
            },
            opt_fp: Some(forext_loop3 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::while_loop1",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    while_loop1().to_register()
                }
                __wrapper
            },
            opt_fp: Some(while_loop1 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::while_loop2",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    while_loop2().to_register()
                }
                __wrapper
            },
            opt_fp: Some(while_loop2 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::while_loop3",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    while_loop3().to_register()
                }
                __wrapper
            },
            opt_fp: Some(while_loop3 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::while_loop4",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    while_loop4().to_register()
                }
                __wrapper
            },
            opt_fp: Some(while_loop4 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::do_while_loop1",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    do_while_loop1().to_register()
                }
                __wrapper
            },
            opt_fp: Some(do_while_loop1 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::do_while_loop2",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    do_while_loop2().to_register()
                }
                __wrapper
            },
            opt_fp: Some(do_while_loop2 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::do_while_loop3",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    do_while_loop3().to_register()
                }
                __wrapper
            },
            opt_fp: Some(do_while_loop3 as *const ()),
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "loop1::do_while_loop4",
        },
        __Linkage::Transfer(__ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: {
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {
                    do_while_loop4().to_register()
                }
                __wrapper
            },
            opt_fp: Some(do_while_loop4 as *const ()),
        }),
    ),
];
