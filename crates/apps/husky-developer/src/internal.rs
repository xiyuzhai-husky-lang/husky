use husky_debugtime::HuskyDevtime;

use crate::HuskyDebuggerConfig;

pub struct HuskyDebuggerInternal {
    pub(crate) config: HuskyDebuggerConfig,
    pub(crate) devtime: HuskyDevtime,
    pub(crate) next_request_id: usize,
}
