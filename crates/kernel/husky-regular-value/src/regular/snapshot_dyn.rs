use super::*;

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
