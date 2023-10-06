use crate::*;
use husky_visual_protocol::mock::MockVisualProtocol;
#[cfg(feature = "mock")]
use husky_visual_protocol::IsVisualProtocol;

pub struct TraceClientCache<VisualProtocol: IsVisualProtocol> {
    cache: TraceCache<VisualProtocol>,
}

impl<VisualProtocol: IsVisualProtocol> TraceClientCache<VisualProtocol> {
    pub fn root_trace_ids(&self) -> TraceIdRange {
        self.cache.root_trace_ids()
    }
}

#[cfg(feature = "mock")]
impl TraceClientCache<MockVisualProtocol> {
    pub fn new_mock() -> Self {
        Self {
            cache: TraceCache::new_mock(),
        }
    }
}

impl<VisualProtocol: IsVisualProtocol> std::ops::Index<TraceIdRange>
    for TraceClientCache<VisualProtocol>
{
    type Output = [TraceCacheEntry];

    fn index(&self, trace_id_range: TraceIdRange) -> &Self::Output {
        &self.cache[trace_id_range]
    }
}
