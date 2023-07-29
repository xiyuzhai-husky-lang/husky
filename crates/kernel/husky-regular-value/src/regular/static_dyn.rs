use super::*;

pub trait __RegularStaticDyn: std::fmt::Debug + RefUnwindSafe + UnwindSafe {
    unsafe fn clone_into_arc_snapshot(&self) -> Arc<dyn __RegularSnapshotDyn>;
}

impl<T> __RegularStaticDyn for T
where
    T: __RegularStatic,
{
    unsafe fn clone_into_arc_snapshot(&self) -> Arc<dyn __RegularSnapshotDyn> {
        Arc::new(self.clone_into_snapshot())
    }
}
