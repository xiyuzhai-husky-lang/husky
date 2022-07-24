use std::panic::catch_unwind;

use super::*;

/// RoutineLinkage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct __SpecificRoutineLinkage {
    pub fp: __SpecificRoutineFp,
    pub dev_src: __StaticDevSource,
    pub opt_raw_fp: Option<*const ()>,
}

unsafe impl Sync for __SpecificRoutineLinkage {}
unsafe impl Send for __SpecificRoutineLinkage {}

impl __SpecificRoutineLinkage {
    pub fn call<'temp, 'eval>(
        &self,
        opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
        mut arguments: Vec<__TempValue<'temp, 'eval>>,
    ) -> __EvalResult<__TempValue<'temp, 'eval>> {
        catch_unwind(move || self.fp.0(opt_ctx, &mut arguments)).map_err(|_| todo!())
    }
    pub fn eval<'temp, 'eval>(
        &self,
        opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
        mut arguments: Vec<__TempValue<'temp, 'eval>>,
    ) -> __VMResult<__Register> {
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
macro_rules! transfer_linkage {
    ($fp: expr, some $raw_fp: expr) => {{
        __Linkage::SpecificTransfer(__SpecificRoutineLinkage {
            fp: __SpecificRoutineFp($fp),
            dev_src: __static_dev_src!(),
            opt_raw_fp: Some($raw_fp as *const ()),
        })
    }};
    ($fp: expr, none) => {{
        __Linkage::SpecificTransfer(__SpecificRoutineLinkage {
            fp: __SpecificRoutineFp($fp),
            dev_src: __static_dev_src!(),
            opt_raw_fp: None,
        })
    }};
}
