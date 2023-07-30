mod incubator;
mod incubator_dyn;
mod snapshot;
mod snapshot_dyn;
mod stand;
mod stand_dyn;

pub use self::incubator::*;
pub use self::incubator_dyn::*;
pub use self::snapshot::*;
pub use self::snapshot_dyn::*;
pub use self::stand::*;
pub use self::stand_dyn::*;

use crate::*;

pub trait __Regular: std::fmt::Debug {
    type __Stand: __RegularStand;
}

impl<T> __Regular for &mut T
where
    T: __Regular,
{
    type __Stand = __RegularValueStandRefMut<<T as __Regular>::__Stand>;
}

impl PartialEq for Box<dyn __RegularStandDyn> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
        // self.0 == other.0 && self.1 == other.1
    }
}

impl Clone for Box<dyn __RegularStandDyn> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Eq for Box<dyn __RegularStandDyn> {}

impl PartialEq for &'static dyn __RegularStandDyn {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for &'static dyn __RegularStandDyn {}
