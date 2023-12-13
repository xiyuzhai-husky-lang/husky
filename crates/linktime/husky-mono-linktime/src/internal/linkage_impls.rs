use super::*;

pub(crate) type LinkageImpls<LinkageImpl> =
    fxhash::FxHashMap<Linkage, (LinkageVersionStamp, LinkageImpl)>;

/// extract from library for safer type wrapping and more efficient lookup
pub(super) fn generate_linkage_impls<LinkageImpl: IsLinkageImpl>(
    target_path: LinktimeTargetPath,
    library: &MonoLinkageLibraries,
    _db: &::salsa::Db,
) -> LinkageImpls<LinkageImpl> {
    // todo!()
    // ad hoc
    Default::default()
}
