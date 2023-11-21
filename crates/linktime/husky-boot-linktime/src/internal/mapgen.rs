use super::*;
use husky_linkage::version_stamp::LinkageVersionStamp;

/// extract from library for safer type wrapping and more efficient lookup
pub(super) fn generate_map<LinkageImpl: IsLinkageImpl>(
    _target_crate: CratePath,
    _library: &BootLibraryStorage,
    _db: &dyn LinkageDb,
) -> HashMap<Linkage, (LinkageVersionStamp, LinkageImpl)> {
    todo!()
}
