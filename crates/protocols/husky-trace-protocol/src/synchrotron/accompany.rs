use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
pub struct AccompanyingTraceIdsExceptFollowed(AccompanyingTraceIds);

impl std::ops::Deref for AccompanyingTraceIdsExceptFollowed {
    type Target = [TraceId];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl AccompanyingTraceIdsExceptFollowed {
    pub(crate) fn new(
        followed_trace_id: Option<TraceId>,
        mut accompanying_trace_ids: AccompanyingTraceIds,
    ) -> Self {
        if let Some(followed_trace_id) = followed_trace_id {
            accompanying_trace_ids.remove(followed_trace_id)
        }
        Self(accompanying_trace_ids)
    }
}
