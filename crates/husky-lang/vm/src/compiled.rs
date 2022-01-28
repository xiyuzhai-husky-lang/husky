use crate::*;

#[derive(Clone, Copy)]
pub struct Compiled {
    pub call: for<'stack, 'eval> fn(
        Vec<StackValue<'stack, 'eval>>,
    ) -> VMResult<StackValue<'stack, 'eval>>,
}

impl std::fmt::Debug for Compiled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("Compiled(")?;
        (self.call as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl std::hash::Hash for Compiled {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.call as usize).hash(state);
    }
}

impl PartialEq for Compiled {
    fn eq(&self, other: &Self) -> bool {
        (self.call as usize) == (other.call as usize)
    }
}

impl Eq for Compiled {}
