use super::*;

/// RoutineLinkage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct __SpecificRoutineLinkage {
    pub call: __SpecificRoutineFp,
    pub nargs: u8,
    pub dev_src: __StaticDevSource,
}

#[macro_export]
macro_rules! specific_transfer_linkage {
    ($fp: expr, $nargs: expr) => {{
        __Linkage::SpecificTransfer(__SpecificRoutineLinkage {
            call: __SpecificRoutineFp($fp),
            nargs: $nargs,
            dev_src: __static_dev_src!(),
        })
    }};
}
