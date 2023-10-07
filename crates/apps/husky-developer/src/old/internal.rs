use husky_debugtime::Devtime;

use crate::HuskyDeveloperConfig;

pub struct HuskyDeveloperInternal {
    pub(crate) config: HuskyDeveloperConfig,
    pub(crate) devtime: Devtime,
    pub(crate) next_request_id: usize,
}
