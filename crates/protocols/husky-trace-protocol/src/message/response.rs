use super::*;
use crate::cache::TraceCache;


/// message sent from trace client to trace server
#[derive(Debug, Serialize, Deserialize)]
pub enum TraceResponse<VisualComponent> {
    Init {
        cache: TraceCache<VisualComponent>,
    },
    TakeCacheAction {
        cache_actions: smallvec::SmallVec<[TraceCacheAction<VisualComponent>; 3]>,
    },
}
