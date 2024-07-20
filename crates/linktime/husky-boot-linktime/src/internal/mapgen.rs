use super::*;
use husky_linket::version_stamp::LinketVersionStamp;

/// extract from library for safer type wrapping and more efficient lookup
pub(super) fn generate_map<LinketImpl: IsLinketImpl>(
    _target_crate: CratePath,
    _library: &BootLibraryStorage,
    _db: &::salsa::Db,
) -> HashMap<Linket, (LinketVersionStamp, LinketImpl)> {
    todo!()
}
