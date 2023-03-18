mod dependency;

pub use self::dependency::*;

use crate::*;
use husky_word::Word;

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageManifest {
    // intentially private
    dependencies: PackageDependencies,
    // intentially private
    dev_dependencies: PackageDevDependencies,
    pub errors: Vec<ManifestError>,
}

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageDependencies {
    #[return_ref]
    pub data: Vec<PackageDependency>,
}

// is this necessary for keeping things as lazy as possible?
#[salsa::tracked(jar = ManifestJar)]
pub(crate) fn package_dependencies(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> VfsResult<PackageDependencies> {
    Ok(package_path.package_manifest(db)?.dependencies(db))
}

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageDevDependencies {
    #[return_ref]
    pub data: Vec<PackageDependency>,
}

// is this necessary for keeping things as lazy as possible?
#[salsa::tracked(jar = ManifestJar)]
pub(crate) fn package_dev_dependencies(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> VfsResult<PackageDevDependencies> {
    Ok(package_path.package_manifest(db)?.dev_dependencies(db))
}

#[salsa::tracked(jar = ManifestJar)]
pub(crate) fn package_manifest(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> VfsResult<PackageManifest> {
    todo!()
}
