mod field;
mod index;

use super::*;

#[derive(Clone, Copy)]
pub struct SpecificRoutineFp(
    pub for<'temp, 'eval> fn(&mut [TempValue<'temp, 'eval>]) -> EvalResult<TempValue<'temp, 'eval>>,
);

impl std::fmt::Debug for SpecificRoutineFp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("SpecificRoutineFp(")?;
        (self.0 as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl std::hash::Hash for SpecificRoutineFp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as usize).hash(state);
    }
}

impl PartialEq for SpecificRoutineFp {
    fn eq(&self, other: &Self) -> bool {
        (self.0 as usize) == (other.0 as usize)
    }
}

impl Eq for SpecificRoutineFp {}
