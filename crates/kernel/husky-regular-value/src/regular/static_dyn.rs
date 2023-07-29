use super::*;

pub trait __RegularStaticDyn: std::fmt::Debug + RefUnwindSafe + UnwindSafe {
    fn clone_into_arc_snapshot(&self) -> Arc<dyn __RegularSnapshotDyn>;
}
