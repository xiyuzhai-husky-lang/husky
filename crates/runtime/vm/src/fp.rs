use crate::*;

#[derive(Clone, Copy)]
pub struct RoutineLinkage {
    pub call: for<'stack, 'eval> fn(
        &mut [StackValue<'stack, 'eval>],
    ) -> VMResult<StackValue<'stack, 'eval>>,
    pub nargs: u8,
}

impl std::fmt::Debug for RoutineLinkage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("RoutineFp(")?;
        (self.call as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl std::hash::Hash for RoutineLinkage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.call as usize).hash(state);
    }
}

impl PartialEq for RoutineLinkage {
    fn eq(&self, other: &Self) -> bool {
        (self.call as usize) == (other.call as usize)
    }
}

impl Eq for RoutineLinkage {}

#[derive(Clone, Copy)]
pub struct MembAccessFp {
    pub call: for<'stack, 'eval> fn(
        &mut StackValue<'stack, 'eval>,
    ) -> VMResult<StackValue<'stack, 'eval>>,
}

impl std::fmt::Debug for MembAccessFp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("RoutineFp(")?;
        (self.call as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl std::hash::Hash for MembAccessFp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.call as usize).hash(state);
    }
}

impl PartialEq for MembAccessFp {
    fn eq(&self, other: &Self) -> bool {
        (self.call as usize) == (other.call as usize)
    }
}

impl Eq for MembAccessFp {}

#[derive(Clone, Copy)]
pub struct ElemAccessFp {
    pub call: for<'stack, 'eval> fn(
        &mut StackValue<'stack, 'eval>,
        usize,
    ) -> VMResult<StackValue<'stack, 'eval>>,
}

impl std::fmt::Debug for ElemAccessFp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("RoutineFp(")?;
        (self.call as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl std::hash::Hash for ElemAccessFp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.call as usize).hash(state);
    }
}

impl PartialEq for ElemAccessFp {
    fn eq(&self, other: &Self) -> bool {
        (self.call as usize) == (other.call as usize)
    }
}

impl Eq for ElemAccessFp {}
