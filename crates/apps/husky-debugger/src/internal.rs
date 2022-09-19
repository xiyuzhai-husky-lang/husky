use husky_debugtime::Tracetime;

use crate::HuskyDebuggerConfig;

pub struct HuskyDebuggerInternal {
    pub(crate) config: HuskyDebuggerConfig,
    pub(crate) debugtime: Tracetime,
    pub(crate) next_request_id: usize,
}
