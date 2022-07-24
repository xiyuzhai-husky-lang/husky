use crate::*;

pub struct __Linkage {
    pub wrapper:
        for<'eval> unsafe fn(Option<&dyn __EvalContext<'eval>>, &mut [__Register]) -> __Register,
    pub opt_fp: Option<*const ()>,
}

unsafe impl Send for __Linkage {}
unsafe impl Sync for __Linkage {}

pub enum __LinkageKind {
    Transfer,
}
