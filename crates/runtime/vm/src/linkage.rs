use crate::*;

#[derive(Clone, Copy)]
pub struct Linkage {
    pub call: for<'stack, 'eval> fn(
        &mut [StackValue<'stack, 'eval>],
    ) -> VMResult<StackValue<'stack, 'eval>>,
    pub nargs: u8,
}

impl std::fmt::Debug for Linkage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("RoutineFp(")?;
        (self.call as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl std::hash::Hash for Linkage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.call as usize).hash(state);
    }
}

impl PartialEq for Linkage {
    fn eq(&self, other: &Self) -> bool {
        (self.call as usize) == (other.call as usize)
    }
}

impl Eq for Linkage {}
