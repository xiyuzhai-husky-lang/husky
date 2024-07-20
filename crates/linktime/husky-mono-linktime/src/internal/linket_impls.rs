use std::iter::zip;

use super::*;
use husky_linket::linket::package_linkets;
use husky_manifest::HasManifest;

pub(crate) type LinketImplMap<LinketImpl> =
    fxhash::FxHashMap<Linket, (LinketVersionStamp, LinketImpl)>;

/// extract from library for efficient lookup
pub(super) fn generate_linket_impls<LinketImpl: IsLinketImpl>(
    target_path: LinktimeTargetPath,
    library: &MonoLinketsLibrary,
    db: &::salsa::Db,
) -> LinketImplMap<LinketImpl> {
    let mut linket_impl_map = LinketImplMap::default();
    let linket_impls: Vec<LinketImpl> = library.cdylib.linket_impls();
    linket_impl_map.extend(
        zip(
            target_path
                .full_dependencies(db)
                .unwrap()
                .iter()
                .map(|&dep| package_linkets(db, dep))
                .flatten()
                .copied(),
            linket_impls,
        )
        .map(|(linket, linket_impl)| (linket, (linket.version_stamp(db), linket_impl))),
    );
    linket_impl_map
}
