use ide::IdeDatabaseSnapshot;

/// An immutable snapshot of the world's state at a point in time.
pub(crate) struct ServerSnapshot {
    pub(crate) db: IdeDatabaseSnapshot,
}

impl std::panic::UnwindSafe for ServerSnapshot {}

impl ServerSnapshot {}
