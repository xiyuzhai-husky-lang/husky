use husky_tracetime::HuskyTracetime;

use crate::HuskyDebuggerConfig;

pub struct HuskyDebuggerInternal {
    pub(crate) config: HuskyDebuggerConfig,
    pub(crate) tracetime: HuskyTracetime,
    pub(crate) next_request_id: usize,
}
