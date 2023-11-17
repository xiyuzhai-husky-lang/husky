use super::*;

/// extract from library for safer type wrapping and more efficient lookup
pub(super) fn generate_map<Linkage: IsLinkage>(
    target_path: LinktimeTargetPath,
    _library: &MonoLinkageStorage,
    _db: &dyn LinkagePathDb,
) -> HashMap<LinkagePath, (LinkageDeps, Linkage)> {
    // todo!()
    // ad hoc
    Default::default()
}
