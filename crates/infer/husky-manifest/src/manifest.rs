mod dependency;

pub use self::dependency::*;

use crate::*;
use husky_manifest_ast::{HasPackageManifestAst, PackageManifestAst};
use husky_word::Word;

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageManifest {
    // intentially private
    dependencies: PackageDependenciesSection,
    // intentially private
    dev_dependencies: PackageDevDependenciesSection,
    pub errors: Vec<ManifestError>,
}

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageDependenciesSection {
    #[return_ref]
    pub data: Vec<PackageDependency>,
}

// is this necessary for keeping things as lazy as possible?
#[salsa::tracked(jar = ManifestJar)]
pub(crate) fn package_dependencies(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> VfsResult<PackageDependenciesSection> {
    Ok(package_path.package_manifest(db)?.dependencies(db))
}

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageDevDependenciesSection {
    #[return_ref]
    pub data: Vec<PackageDependency>,
}

// is this necessary for keeping things as lazy as possible?
#[salsa::tracked(jar = ManifestJar)]
pub(crate) fn package_dev_dependencies(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> VfsResult<PackageDevDependenciesSection> {
    Ok(package_path.package_manifest(db)?.dev_dependencies(db))
}

#[salsa::tracked(jar = ManifestJar)]
pub(crate) fn package_manifest(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> VfsResult<PackageManifest> {
    Ok(PackageManifest::from_ast(
        db,
        package_path.toolchain(db),
        package_path.manifest_ast(db)?,
    ))
}

impl PackageManifest {
    fn from_ast(
        db: &dyn ManifestDb,
        toolchain: Toolchain,
        manifest_ast: PackageManifestAst,
    ) -> Self {
        let mut errors = vec![];
        let dependencies_section = PackageDependenciesSection::new(
            db,
            manifest_ast
                .dependencies_section(db)
                .as_ref()
                .ok()
                .map(|s| s.as_ref())
                .flatten()
                .map(|dependencies_section_ast| {
                    dependencies_section_ast
                        .dependencies()
                        .iter()
                        .filter_map(|dependency_ast| {
                            PackageDependency::from_ast(db, toolchain, dependency_ast, &mut errors)
                        })
                        .collect()
                })
                .unwrap_or_default(),
        );
        let dev_dependencies_section = PackageDevDependenciesSection::new(
            db,
            manifest_ast
                .dependencies_section(db)
                .as_ref()
                .ok()
                .map(|s| s.as_ref())
                .flatten()
                .map(|ast| todo!())
                .unwrap_or_default(),
        );
        Self::new(db, dependencies_section, dev_dependencies_section, errors)
    }
}
