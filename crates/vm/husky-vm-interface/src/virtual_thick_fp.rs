use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VirtualThickFp {
    needs_eval_context: bool,
    fp: *const (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct __OptVirtualThickFp {
    needs_eval_context: bool,
    fp: *const (),
}

impl __OptVirtualThickFp {
    pub const fn none() -> Self {
        Self {
            needs_eval_context: false,
            fp: std::ptr::null(),
        }
    }

    #[cfg(feature = "thin_fp")]
    pub const fn some_base<'eval, F: ~const __BaseThinFp>(f: F) -> Self {
        Self {
            needs_eval_context: false,
            fp: f.__to_void_pointer(),
        }
    }

    #[cfg(feature = "thin_fp")]
    pub const fn some_ctx<'eval, F: ~const __CtxThinFp>(f: F) -> Self {
        Self {
            needs_eval_context: true,
            fp: f.__to_void_pointer(),
        }
    }
}
