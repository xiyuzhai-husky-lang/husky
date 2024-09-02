use super::*;

impl<T> Frozen for Vec<T>
where
    T: Frozen,
{
    type Thawed = Vec<T::Thawed>;

    type Slush = Vec<T::Slush>;

    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
        todo!()
    }
}
