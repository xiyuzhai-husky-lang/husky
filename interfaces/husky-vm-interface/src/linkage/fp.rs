mod field;
mod index;
mod method_elem;

use super::*;

#[derive(Clone, Copy)]
pub struct __LinkageFp {
    pub wrapper:
        for<'eval> unsafe fn(Option<&dyn __EvalContext<'eval>>, &mut [__Register]) -> __Register,
    pub opt_fp: Option<*const ()>,
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

// impl std::hash::Hash for __SpecificRoutineFp {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         (self.0 as usize).hash(state);
//     }
// }

#[derive(Clone, Copy)]
pub struct __ContextualSpecificRoutineFp(
    pub for<'temp, 'eval> fn(__ctx: &dyn __EvalContext<'eval>, &mut [__Register]) -> __Register,
);

impl std::fmt::Debug for __ContextualSpecificRoutineFp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("__ContextualSpecificRoutineFp(")?;
        (self.0 as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl std::hash::Hash for __ContextualSpecificRoutineFp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as usize).hash(state);
    }
}

impl PartialEq for __ContextualSpecificRoutineFp {
    fn eq(&self, other: &Self) -> bool {
        (self.0 as usize) == (other.0 as usize)
    }
}

impl Eq for __ContextualSpecificRoutineFp {}
