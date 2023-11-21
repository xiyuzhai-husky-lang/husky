use super::*;

/// extract from library for safer type wrapping and more efficient lookup
pub(super) fn generate_map<LinkageImpl: IsLinkageImpl>(
    target_path: LinktimeTargetPath,
    _library: &MonoLinkageStorage,
    _db: &dyn LinkageDb,
) -> HashMap<Linkage, (LinkageVersionStamp, LinkageImpl)> {
    // todo!()
    // ad hoc
    Default::default()
}
