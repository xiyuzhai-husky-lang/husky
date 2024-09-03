use super::*;

impl<T> Frozen for Option<T>
where
    T: Frozen,
{
    type Thawed = Option<T::Thawed>;

    type Slush = T::Slush;

    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
        // (None,self.as_ref().map(|t|t.revive()))
        match self {
            Some(slf) => {
                let (stand, revived) = slf.thaw();
                (stand, Some(revived))
            }
            None => (None, None),
        }
    }
}
