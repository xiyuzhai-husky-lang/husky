// impl<T> Frozen for &'static T
// where
//     T: Thawed + Send + Sync,
// {
//     type Thawed = Self;

//     type Slush = ();

//     fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
//         (None, *self)
//     }
// }
