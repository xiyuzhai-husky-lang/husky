pub trait SnapshotClone: Sized {
    fn snapshot_clone(&self) -> Self;

    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot {
            t: self.snapshot_clone(),
        }
    }
}

pub struct Snapshot<T> {
    t: T,
}

impl<T> std::ops::Deref for Snapshot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T> Clone for Snapshot<T>
where
    T: SnapshotClone,
{
    fn clone(&self) -> Self {
        Self {
            t: self.t.snapshot_clone(),
        }
    }
}
