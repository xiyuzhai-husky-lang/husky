mod field;
mod index;
mod method_elem;

use super::*;

#[derive(Clone, Copy)]
pub struct __LinkageFp {
    pub wrapper: for<'eval> unsafe fn(
        Option<&dyn __EvalContext<'eval>>,
        &mut [__Register<'eval>],
    ) -> __Register<'eval>,
    pub opt_fp: Option<*const ()>,
}
impl __LinkageFp {
    pub fn eval<'eval>(
        self,
        opt_ctx: Option<&dyn __EvalContext<'eval>>,
        arguments: &mut [__Register],
    ) -> __VMResult<__Register<'eval>> {
        todo!()
    }
}

impl std::fmt::Debug for __LinkageFp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("__LinkageFp")
            .field("wrapper", &(self.wrapper as *const ()))
            .field("opt_fp", &self.opt_fp)
            .finish()
    }
}
impl PartialEq for __LinkageFp {
    fn eq(&self, other: &Self) -> bool {
        self.wrapper as usize == other.wrapper as usize && self.opt_fp == other.opt_fp
    }
}
impl Eq for __LinkageFp {}
unsafe impl Send for __LinkageFp {}
unsafe impl Sync for __LinkageFp {}
