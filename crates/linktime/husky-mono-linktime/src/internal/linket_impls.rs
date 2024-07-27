use std::iter::zip;

use super::*;
use husky_linket::linket::{package_linkets, target_linkets};
use husky_linket_impl::LinketImpls;
use husky_manifest::HasManifest;

pub(crate) type LinketImplMap<LinketImpl> =
    fxhash::FxHashMap<Linket, (LinketVersionStamp, LinketImpl)>;

/// extract from library for efficient lookup
pub(super) fn generate_linket_impl_map<LinketImpl: IsLinketImpl>(
    target_path: LinktimeTargetPath,
    library: &MonoLinketsLibrary<LinketImpl>,
    db: &::salsa::Db,
) -> LinketImplMap<LinketImpl> {
    let mut linket_impl_map = LinketImplMap::default();
    let linket_impls = library.linket_impls();
    linket_impl_map.extend(
        zip(target_linkets(db, target_path), linket_impls.linket_impls())
            .map(|(&linket, &linket_impl)| (linket, (linket.version_stamp(db), linket_impl))),
    );
    linket_impl_map
}
