use super::*;
use vec_like::VecSet;

#[salsa::tracked]
pub struct PackageDependenciesSection {
    #[return_ref]
    pub data: Vec<PackageDependency>,
}

#[salsa::tracked(return_ref)]
pub(crate) fn package_dependencies(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResult<PackageDependenciesSection> {
    let cyclic_dependent_package_paths = cyclic_dependent_package_paths(db, package_path)?;
    if cyclic_dependent_package_paths.len() > 0 {
        Err(ManifestError::CyclicDependendencies {
            package_path,
            cyclic_dependent_package_paths,
        })?
    }
    package_dependencies_unchecked(db, package_path)
}

pub(crate) fn package_dependencies_unchecked(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResult<PackageDependenciesSection> {
    Ok(package_path.package_manifest(db)?.dependencies(db))
}

pub(crate) fn cyclic_dependent_package_paths(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResult<Vec<PackagePath>> {
    // skip the first because it's always equal to package_path
    Ok(full_dependent_package_paths(db, package_path)?[1..]
        .iter()
        .copied()
        .filter(
            |&dep_package_path| match full_dependent_package_paths(db, dep_package_path) {
                Ok(dep_dep_package_paths) => dep_dep_package_paths[1..]
                    .iter()
                    .find(|&&dep_dep_package_path| dep_dep_package_path == package_path)
                    .is_some(),
                Err(_) => false,
            },
        )
        .collect())
}

/// includes package_path itself
pub(crate) fn full_dependent_package_paths(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResultRef<&[PackagePath]> {
    full_dependent_package_paths_aux(db, package_path)
        .as_ref()
        .map(|s| s as &[_])
}

#[salsa::tracked(return_ref)]
pub(crate) fn full_dependent_package_paths_aux(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResult<VecSet<PackagePath>> {
    let mut package_paths: VecSet<PackagePath> = VecSet::new_one_elem_set(package_path);
    let mut first_unsearched = 0usize;
    while first_unsearched < package_paths.len() {
        let first_unsearched = std::mem::replace(&mut first_unsearched, package_paths.len());
        for i in first_unsearched..package_paths.len() {
            package_paths.extend(
                package_dependencies_unchecked(db, package_paths[i])?
                    .data(db)
                    .iter()
                    .map(|dep| dep.package_path()),
            );
        }
    }
    Ok(package_paths)
}
