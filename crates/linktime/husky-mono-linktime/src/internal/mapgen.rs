use super::*;

/// extract from library for safer type wrapping and more efficient lookup
pub(super) fn generate_map<Linkage: IsLinkage>(
    _target_crate: CratePath,
    _library: &MonoLinkageStorage,
    _db: &dyn LinkagePathDb,
) -> HashMap<LinkagePath, (LinkageDeps, Linkage)> {
    // todo!()
    // ad hoc
    Default::default()
}
