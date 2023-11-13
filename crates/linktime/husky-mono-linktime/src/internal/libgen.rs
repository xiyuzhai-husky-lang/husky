use super::*;

pub(super) fn generate_library<Db: ?Sized>(_target_crate: CratePath, _db: &Db) -> MonoLibraryStorage {
    MonoLibraryStorage {}
}
