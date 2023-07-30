use super::*;

pub trait __RegularSnapshot: std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type __Incubator: __RegularIncubator;

    fn clone_into_incubator(&self) -> Self::__Incubator;
}

#[derive(Debug)]
pub struct __RegularSnapshotValueRefMut<T>(pub(super) Box<T::__Snapshot>)
where
    T: __RegularStand;

impl<T> __RegularSnapshot for __RegularSnapshotValueRefMut<T>
where
    T: __RegularStand,
{
    type __Incubator = __RegularValueIncubatorRefMut<T>;

    fn clone_into_incubator(&self) -> Self::__Incubator {
        todo!()
    }
}
pub trait __RegularSnapshotDyn: std::fmt::Debug {
    fn clone_into_incubator_box_dyn(&self) -> Box<dyn __RegularIncubatorDyn>;
}

impl<T> __RegularSnapshotDyn for T
where
    T: __RegularSnapshot,
{
    fn clone_into_incubator_box_dyn(&self) -> Box<dyn __RegularIncubatorDyn> {
        todo!()
    }
}
