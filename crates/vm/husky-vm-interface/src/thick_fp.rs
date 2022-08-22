use crate::*;

pub struct ThickFp<F: BaseFp> {
    needs_eval_context: bool,
    fp: *const (),
}
