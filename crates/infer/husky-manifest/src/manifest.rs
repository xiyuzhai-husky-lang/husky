mod dependency;

pub use self::dependency::*;

use crate::*;
use husky_corgi_config::HasCorgiConfig;
use husky_manifest_ast::{HasPackageManifestAstSheet, PackageManifestAstSheet};
use husky_word::Word;

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageManifest {
    // intentially private
    dependencies: PackageDependenciesSection,
    // intentially private
    dev_dependencies: PackageDevDependenciesSection,
}

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageDependenciesSection {
    #[return_ref]
    pub data: Vec<PackageDependency>,
}

pub(crate) fn package_dependencies(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> ManifestResultRef<PackageDependenciesSection> {
    package_dependencies_aux(db, package_path).as_ref().copied()
}

// is this necessary for keeping things as lazy as possible?
#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn package_dependencies_aux(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> ManifestResult<PackageDependenciesSection> {
    Ok(package_path.package_manifest(db)?.dependencies(db))
}

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageDevDependenciesSection {
    #[return_ref]
    pub data: Vec<PackageDependency>,
}

// is this necessary for keeping things as lazy as possible?
#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn package_dev_dependencies(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> ManifestResult<PackageDevDependenciesSection> {
    Ok(package_path.package_manifest(db)?.dev_dependencies(db))
}

pub(crate) fn package_manifest(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> ManifestResultRef<PackageManifest> {
    package_manifest_aux(db, package_path).as_ref().copied()
}

#[salsa::tracked(jar = ManifestJar, return_ref)]
pub(crate) fn package_manifest_aux(
    db: &dyn ManifestDb,
    package_path: PackagePath,
) -> ManifestResult<PackageManifest> {
    Ok(PackageManifest::from_ast(
        db,
        package_path.toolchain(db),
        package_path.registry_path(db)?,
        package_path.manifest_ast_sheet(db)?,
    ))
}

impl PackageManifest {
    fn from_ast(
        db: &dyn ManifestDb,
        toolchain: Toolchain,
        registry_path: RegistryPath,
        manifest_ast: PackageManifestAstSheet,
    ) -> Self {
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
                        .map(|dependency_ast| {
                            PackageDependency::from_ast(
                                db,
                                toolchain,
                                registry_path,
                                dependency_ast,
                            )
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
        Self::new(db, dependencies_section, dev_dependencies_section)
    }
}
