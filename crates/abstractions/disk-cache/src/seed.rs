use serde::{Deserialize, Serialize};

pub trait IsDiskCacheSeed:
    Copy + Serialize + for<'de> Deserialize<'de> + Eq + std::hash::Hash + Send + Sync
{
}

impl<T: Copy + Serialize + for<'de> Deserialize<'de> + Eq + std::hash::Hash + Send + Sync>
    IsDiskCacheSeed for T
{
}
