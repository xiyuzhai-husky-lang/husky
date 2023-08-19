use husky_debugtime::Devtime;

use crate::HuskyDebuggerConfig;

pub struct HuskyDebuggerInternal {
    pub(crate) config: HuskyDebuggerConfig,
    pub(crate) devtime: Devtime,
    pub(crate) next_request_id: usize,
}
