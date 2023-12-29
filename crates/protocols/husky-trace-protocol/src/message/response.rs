use super::*;
use crate::cache::TraceCache;

/// message sent from trace client to trace server
#[derive(Debug, Serialize, Deserialize)]
pub enum TraceResponse<TraceProtocol: IsTraceProtocol> {
    Init {
        cache: TraceCache<TraceProtocol>,
    },
    TakeCacheAction {
        cache_actions: smallvec::SmallVec<[TraceCacheAction<TraceProtocol>; 3]>,
    },
}
