use crate::*;

pub trait IsRequest:
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
    > IsRequest for T
{
}

pub trait IsResponse:
    Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static
{
}

impl<T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static> IsResponse for T {}
