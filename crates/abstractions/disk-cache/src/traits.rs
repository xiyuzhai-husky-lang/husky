use crate::*;

pub trait IsDiskCacheRequest:
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
    > IsDiskCacheRequest for T
{
}

pub trait IsDiskCacheResponse:
    Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static
{
}

impl<T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static> IsDiskCacheResponse
    for T
{
}
