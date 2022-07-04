use std::panic::catch_unwind;

use super::*;

/// RoutineLinkage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct __SpecificRoutineLinkage {
    pub fp: __SpecificRoutineFp,
    pub nargs: u8,
    pub dev_src: __StaticDevSource,
}

impl __SpecificRoutineLinkage {
    pub fn call<'temp, 'eval>(
        &self,
        mut arguments: Vec<__TempValue<'temp, 'eval>>,
    ) -> __EvalResult<__TempValue<'temp, 'eval>> {
        catch_unwind(move || self.fp.0(&mut arguments)).map_err(|_| todo!())
    }
    pub fn eval<'temp, 'eval>(
        &self,
        mut arguments: Vec<__TempValue<'temp, 'eval>>,
    ) -> EvalValueResult<'eval> {
        catch_unwind(move || self.fp.0(&mut arguments))
            .map_err(|_| todo!())
            .map(|v| v.into_eval())
    }
}

#[macro_export]
macro_rules! specific_transfer_linkage {
    ($fp: expr, $nargs: expr) => {{
        __Linkage::SpecificTransfer(__SpecificRoutineLinkage {
            fp: __SpecificRoutineFp($fp),
            nargs: $nargs,
            dev_src: __static_dev_src!(),
        })
    }};
}
