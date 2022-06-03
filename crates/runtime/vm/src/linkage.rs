use dev_utils::{DevSource, StaticDevSource};

use crate::*;

#[derive(Clone, Copy)]
pub struct Linkage {
    pub call: for<'temp, 'eval> fn(
        &mut [TempValue<'temp, 'eval>],
    ) -> VMRuntimeResult<TempValue<'temp, 'eval>>,
    pub nargs: u8,
    pub dev_src: &'static StaticDevSource,
}

#[macro_export]
macro_rules! linkage {
    ($fp: expr, $nargs: expr) => {{
        Linkage {
            call: $fp,
            nargs: $nargs,
            dev_src: &dev_utils::static_dev_src!(),
        }
    }};
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
