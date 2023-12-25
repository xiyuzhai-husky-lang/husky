use std::iter::zip;

use super::*;
use husky_linkage::linkage::package_linkages;
use husky_linkage_impl::AnyLinkageImpls;

pub(crate) type LinkageImplMap<LinkageImpl> =
    fxhash::FxHashMap<Linkage, (LinkageVersionStamp, LinkageImpl)>;

/// extract from library for efficient lookup
pub(super) fn generate_linkage_impls<LinkageImpl: IsLinkageImpl>(
    target_path: LinktimeTargetPath,
    libraries: &MonoLinkageLibraries,
    db: &::salsa::Db,
) -> LinkageImplMap<LinkageImpl> {
    let mut linkage_impls = LinkageImplMap::default();
    for &(package_path, ref cdylib) in libraries.cdylibs.iter() {
        let package_linkages = package_linkages(db, package_path);
        let package_linkage_impls: Vec<LinkageImpl> = cdylib.linkage_impls();
        debug_assert_eq!(package_linkage_impls.len(), package_linkages.len());
        linkage_impls.extend(
            zip(package_linkages, package_linkage_impls).map(|(&linkage, linkage_impl)| {
                (linkage, (linkage.version_stamp(db), linkage_impl))
            }),
        )
    }
    linkage_impls
}
