use super::*;

/// RoutineLinkage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecificRoutineLinkage {
    pub call: SpecificRoutineFp,
    pub nargs: u8,
    pub dev_src: StaticDevSource,
}

#[macro_export]
macro_rules! routine_linkage {
    ($fp: expr, $nargs: expr) => {{
        SpecificRoutineLinkage {
            call: vm::SpecificRoutineFp($fp),
            nargs: $nargs,
            dev_src: dev_utils::static_dev_src!(),
        }
    }};
}
