use crate::*;

pub trait IsLlmCacheRequest:
    Serialize + for<'de> Deserialize<'de> + Eq + std::hash::Hash + Clone + Send + Sync + 'static
{
}

impl<
        T: Serialize
            + for<'de> Deserialize<'de>
            + Eq
            + std::hash::Hash
            + Clone
            + Send
            + Sync
            + 'static,
    > IsLlmCacheRequest for T
{
}

pub trait IsLlmCacheResponse:
    Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static
{
}

impl<T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static> IsLlmCacheResponse
    for T
{
}
