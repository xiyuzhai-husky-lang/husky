use super::*;
use vec_like::VecSet;

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageDependenciesSection {
    #[return_ref]
    pub data: Vec<ManifestDependency>,
}

pub(crate) fn package_dependencies(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> ManifestResultRef<&[ManifestDependency]> {
    // todo!(): check for cycles!
    package_dependencies_unchecked(db, package_path)
}

fn package_dependencies_unchecked(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> ManifestResultRef<&[ManifestDependency]> {
    package_dependencies_unchecked_aux(db, package_path)
        .as_ref()
        .map(|s| s.data(db) as &[_])
}

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn package_dependencies_unchecked_aux(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> ManifestResult<PackageDependenciesSection> {
    Ok(package_path.package_manifest(db)?.dependencies(db))
}

fn cyclic_dependent_package_paths(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> ManifestResultRef<&[PackagePath]> {
    cyclic_dependent_package_paths_aux(db, package_path)
        .as_ref()
        .map(|s| s as &[_])
}

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn cyclic_dependent_package_paths_aux(
    _db: &dyn ManifestDb,
    _package_path: PackagePath,
) -> ManifestResult<VecSet<PackagePath>> {
    todo!()
}

fn full_dependent_package_paths(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> ManifestResultRef<&[PackagePath]> {
    full_dependent_package_paths_aux(db, package_path)
        .as_ref()
        .map(|s| s as &[_])
}

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn full_dependent_package_paths_aux(
    _db: &dyn ManifestDb,
    _package_path: PackagePath,
) -> ManifestResult<VecSet<PackagePath>> {
    todo!()
}
