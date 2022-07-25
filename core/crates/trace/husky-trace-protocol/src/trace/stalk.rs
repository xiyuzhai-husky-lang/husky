use husky_vm_interface::__VMResult;

use super::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceStalkData {
    pub extra_tokens: Vec<TraceTokenData>,
}

impl<'eval> From<__VMResult<__Register<'eval>>> for TraceStalkData {
    fn from(_: __VMResult<__Register<'eval>>) -> Self {
        todo!()
    }
}
