mod field;
mod index;
mod method_elem;

use super::*;
use std::panic::catch_unwind;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct __ResolvedLinkage {
    pub wrapper: for<'eval> unsafe fn(
        Option<&dyn __EvalContext<'eval>>,
        &mut [__Register<'eval>],
    ) -> __Register<'eval>,
    pub opt_thick_fp: OptVirtualThickFp,
    pub dev_src: __StaticDevSource,
}

#[cfg(feature = "extra")]
impl __ResolvedLinkage {
    // pub fn eval<'eval>(
    //     self,
    //     opt_ctx: Option<&dyn __EvalContext<'eval>>,
    //     mut arguments: Vec<__Register<'eval>>,
    // ) -> __VMResult<__Register<'eval>> {
    //     catch_unwind(move || unsafe { (self.wrapper)(opt_ctx, &mut arguments).into_eval() })
    //         .map_err(|e| __VMError {
    //             message: format!("error: {e:?} when calling linkage",),
    //             variant: __VMErrorVariant::Normal,
    //         })
    // }

    pub fn call_catch_unwind<'eval>(
        self,
        opt_ctx: Option<&dyn __EvalContext<'eval>>,
        mut arguments: Vec<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>> {
        catch_unwind(move || self.call(opt_ctx, &mut arguments)).map_err(|e| {
            if let Some(e) = e.downcast_ref::<String>() {
                __VMError {
                    message: format!("error: `{e}` when calling linkage",),
                    variant: __VMErrorVariant::Normal,
                }
            } else if let Some(e) = e.downcast_ref::<&str>() {
                __VMError {
                    message: format!("error: `{e}` when calling linkage",),
                    variant: __VMErrorVariant::Normal,
                }
            } else {
                todo!()
            }
        })
    }

    pub fn call<'eval>(
        self,
        opt_ctx: Option<&dyn __EvalContext<'eval>>,
        arguments: &mut [__Register<'eval>],
    ) -> __Register<'eval> {
        unsafe { (self.wrapper)(opt_ctx, arguments) }
    }
}

impl std::fmt::Debug for __ResolvedLinkage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("__ResolvedLinkage")
            .field("wrapper", &(self.wrapper as *const ()))
            .field("opt_fp", &self.opt_thick_fp)
            .finish()
    }
}
impl PartialEq for __ResolvedLinkage {
    fn eq(&self, other: &Self) -> bool {
        self.wrapper as usize == other.wrapper as usize && self.opt_thick_fp == other.opt_thick_fp
    }
}
impl Eq for __ResolvedLinkage {}
unsafe impl Send for __ResolvedLinkage {}
unsafe impl Sync for __ResolvedLinkage {}

#[cfg(feature = "linkage_macro")]
#[macro_export]
macro_rules! resolved_linkage {
    ($wrapper: expr, some $raw_fp: expr) => {{
        __ResolvedLinkage {
            wrapper: $wrapper,
            opt_thick_fp: OptVirtualThickFp::some($raw_fp),
            dev_src: static_dev_src!(),
        }
    }};

    ($wrapper: expr, none) => {{
        __ResolvedLinkage {
            wrapper: $wrapper,
            opt_thick_fp: OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};

    ($wrapper: expr) => {{
        __ResolvedLinkage {
            wrapper: $wrapper,
            opt_thick_fp: OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
}
