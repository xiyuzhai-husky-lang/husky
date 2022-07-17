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
        opt_ctx: Option<&__EvalContext<'eval>>,
        mut arguments: Vec<__TempValue<'temp, 'eval>>,
    ) -> __EvalResult<__TempValue<'temp, 'eval>> {
        catch_unwind(move || self.fp.0(opt_ctx, &mut arguments)).map_err(|_| todo!())
    }
    pub fn eval<'temp, 'eval>(
        &self,
        opt_ctx: Option<&__EvalContext<'eval>>,
        mut arguments: Vec<__TempValue<'temp, 'eval>>,
    ) -> __EvalValueResult<'eval> {
        catch_unwind(move || self.fp.0(opt_ctx, &mut arguments).into_eval()).map_err(|e| {
            EvalError::Normal {
                message: format!(
                    "error: {e:?} when calling linkage with src = {}",
                    self.dev_src
                ),
            }
        })
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
