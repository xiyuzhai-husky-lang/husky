use super::*;

/// RoutineLinkage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecificRoutineLinkage {
    pub call: __SpecificRoutineFp,
    pub nargs: u8,
    pub dev_src: __StaticDevSource,
}

#[macro_export]
macro_rules! routine_linkage {
    ($fp: expr, $nargs: expr) => {{
        SpecificRoutineLinkage {
            call: vm::__SpecificRoutineFp($fp),
            nargs: $nargs,
            dev_src: __static_dev_src!(),
        }
    }};
}
