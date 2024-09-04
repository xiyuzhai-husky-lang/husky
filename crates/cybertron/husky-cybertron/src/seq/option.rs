use super::*;

impl<T> Seq<Option<T>>
where
    T: Copy + Any + Send + Sync,
{
    pub fn or(self, other: Self) -> Self {
        Option::or.apply(self, other)
    }
}
