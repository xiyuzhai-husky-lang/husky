use super::*;

#[salsa::tracked]
pub struct PackageDevDependenciesSection {
    #[return_ref]
    pub data: Vec<PackageDependency>,
}

#[salsa::tracked(return_ref)]
pub(crate) fn package_dev_dependencies_unchecked(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResult<PackageDevDependenciesSection> {
    Ok(package_path.manifest(db)?.dev_dependencies(db))
}
