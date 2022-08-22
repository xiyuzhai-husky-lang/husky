use crate::*;

pub struct ThickFp<F: for<'eval> ThinFp<'eval>> {
    needs_eval_context: bool,
    fp: F,
}

pub struct VirtualThickFp {
    needs_eval_context: bool,
    fp: *const (),
}
