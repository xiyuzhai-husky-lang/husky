use super::*;

pub trait __RegularSnapshotDyn: std::fmt::Debug {}

impl<T> __RegularSnapshotDyn for T where T: __RegularSnapshot {}
