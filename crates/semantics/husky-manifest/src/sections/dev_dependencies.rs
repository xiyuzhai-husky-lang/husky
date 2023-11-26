use super::*;

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageDevDependenciesSection {
    #[return_ref]
    pub data: Vec<ManifestDependency>,
}

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn package_dev_dependencies_unchecked(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResult<PackageDevDependenciesSection> {
    Ok(package_path.package_manifest(db)?.dev_dependencies(db))
}
