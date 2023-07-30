use super::*;

pub trait __RegularStandDyn: std::fmt::Debug + std::any::Any + RefUnwindSafe + UnwindSafe {
    unsafe fn clone_into_arc_snapshot(&self) -> Arc<dyn __RegularSnapshotDyn>;
}

impl<T> __RegularStandDyn for T
where
    T: __RegularStand,
{
    unsafe fn clone_into_arc_snapshot(&self) -> Arc<dyn __RegularSnapshotDyn> {
        Arc::new(self.clone_into_snapshot())
    }
}
