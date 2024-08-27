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
    pub(crate) fn new(followed: Option<TraceId>, mut accompanyings: AccompanyingTraceIds) -> Self {
        if let Some(followed) = followed {
            accompanyings.remove(followed)
        }
        Self(accompanyings)
    }
}
