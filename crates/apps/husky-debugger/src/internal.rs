use husky_debugtime::HuskyDebugtime;

use crate::HuskyDebuggerConfig;

pub struct HuskyDebuggerInternal {
    pub(crate) config: HuskyDebuggerConfig,
    pub(crate) devtime: HuskyDebugtime,
    pub(crate) next_request_id: usize,
}
