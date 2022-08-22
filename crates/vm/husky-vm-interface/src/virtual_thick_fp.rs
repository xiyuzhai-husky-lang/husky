use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VirtualThickFp {
    needs_eval_context: bool,
    fp: *const (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptVirtualThickFp {
    needs_eval_context: bool,
    fp: *const (),
}

impl OptVirtualThickFp {
    pub const fn none() -> Self {
        Self {
            needs_eval_context: false,
            fp: std::ptr::null(),
        }
    }

    #[cfg(feature = "thin_fp")]
    pub const fn some<'eval, F: ~const BaseFp>(f: F) -> Self {
        Self {
            needs_eval_context: false,
            fp: f.to_raw(),
        }
    }
}
