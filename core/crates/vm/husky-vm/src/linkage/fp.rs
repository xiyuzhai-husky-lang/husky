mod field;
mod index;
mod method_elem;

use super::*;

#[derive(Clone, Copy)]
pub struct __SpecificRoutineFp(
    pub  for<'temp, 'eval> fn(
        Option<&dyn __EvalContext<'eval>>,
        &mut [__TempValue<'temp, 'eval>],
    ) -> __TempValue<'temp, 'eval>,
);

impl std::fmt::Debug for __SpecificRoutineFp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("__SpecificRoutineFp(")?;
        (self.0 as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl std::hash::Hash for __SpecificRoutineFp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as usize).hash(state);
    }
}

impl PartialEq for __SpecificRoutineFp {
    fn eq(&self, other: &Self) -> bool {
        (self.0 as usize) == (other.0 as usize)
    }
}

impl Eq for __SpecificRoutineFp {}

#[derive(Clone, Copy)]
pub struct __ContextualSpecificRoutineFp(
    pub  for<'temp, 'eval> fn(
        __ctx: &dyn __EvalContext<'eval>,
        &mut [__TempValue<'temp, 'eval>],
    ) -> __TempValue<'temp, 'eval>,
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
