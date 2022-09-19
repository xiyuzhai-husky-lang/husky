use husky_tracetime::Tracetime;

use crate::HuskyDebuggerConfig;

pub struct HuskyDebuggerInternal {
    pub(crate) config: HuskyDebuggerConfig,
    pub(crate) tracetime: Tracetime,
    pub(crate) next_request_id: usize,
}
