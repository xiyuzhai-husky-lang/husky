use husky_debugtime::Debugtime;

use crate::HuskyDebuggerConfig;

pub struct HuskyDebuggerInternal {
    pub(crate) config: HuskyDebuggerConfig,
    pub(crate) debugtime: Debugtime,
    pub(crate) next_request_id: usize,
}
