use std::iter::zip;

use super::*;
use husky_linket::linket::package_linkets;

pub(crate) type LinketImplMap<LinketImpl> =
    fxhash::FxHashMap<Linket, (LinketVersionStamp, LinketImpl)>;

/// extract from library for efficient lookup
pub(super) fn generate_linket_impls<LinketImpl: IsLinketImpl>(
    _target_path: LinktimeTargetPath,
    libraries: &MonoLinketLibraries,
    db: &::salsa::Db,
) -> LinketImplMap<LinketImpl> {
    let mut linket_impls = LinketImplMap::default();
    for &(package_path, ref cdylib) in libraries.cdylibs.iter() {
        let package_linkets = package_linkets(db, package_path);
        let package_linket_impls: Vec<LinketImpl> = cdylib.linket_impls();
        debug_assert_eq!(package_linket_impls.len(), package_linkets.len());
        linket_impls.extend(
            zip(package_linkets, package_linket_impls)
                .map(|(&linket, linket_impl)| (linket, (linket.version_stamp(db), linket_impl))),
        )
    }
    linket_impls
}
