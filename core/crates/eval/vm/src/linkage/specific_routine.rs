use super::*;

/// RoutineLinkage
#[derive(Clone, Copy)]
pub struct SpecificRoutineLinkage {
    pub call:
        for<'temp, 'eval> fn(&mut [TempValue<'temp, 'eval>]) -> EvalResult<TempValue<'temp, 'eval>>,
    pub nargs: u8,
    pub dev_src: &'static StaticDevSource,
}

#[macro_export]
macro_rules! routine_linkage {
    ($fp: expr, $nargs: expr) => {{
        SpecificRoutineLinkage {
            call: $fp,
            nargs: $nargs,
            dev_src: &dev_utils::static_dev_src!(),
        }
    }};
}

impl std::fmt::Debug for SpecificRoutineLinkage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("RoutineFp(")?;
        (self.call as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl std::hash::Hash for SpecificRoutineLinkage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.call as usize).hash(state);
    }
}

impl PartialEq for SpecificRoutineLinkage {
    fn eq(&self, other: &Self) -> bool {
        (self.call as usize) == (other.call as usize)
    }
}

impl Eq for SpecificRoutineLinkage {}
