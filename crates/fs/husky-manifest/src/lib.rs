pub mod dependency;
pub mod error;
pub mod helpers;
pub mod jar;
pub mod manifest;
pub mod sections;

use self::dependency::*;
use self::error::*;
pub use self::manifest::*;
use self::sections::*;

use self::jar::ManifestJar as Jar;
use husky_corgi_config::HasCorgiConfig;
use husky_manifest_ast::{HasPackageManifestAstSheet, PackageManifestAstSheet};
use husky_vfs::*;

#[salsa::tracked]
pub struct PackageManifest {
    // intentially private
    dependencies: PackageDependenciesSection,
    // intentially private
    dev_dependencies: PackageDevDependenciesSection,
}

pub(crate) fn package_manifest(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResultRef<PackageManifest> {
    package_manifest_aux(db, package_path).as_ref().copied()
}

#[salsa::tracked(return_ref)]
pub(crate) fn package_manifest_aux(
    db: &::salsa::Db,
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
        db: &::salsa::Db,
        toolchain: Toolchain,
        registry_path: RegistryPath,
        manifest_ast: &PackageManifestAstSheet,
    ) -> Self {
        let dependencies_section = PackageDependenciesSection::new(
            db,
            manifest_ast
                .dependencies_section()
                .as_ref()
                .map(|s| s.as_ref().ok())
                .flatten()
                .map(|dependencies_section_ast| {
                    dependencies_section_ast
                        .deps()
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
                .dependencies_section()
                .as_ref()
                .map(|s| s.as_ref().ok())
                .flatten()
                .map(|dev_dependencies_section_ast| {
                    dev_dependencies_section_ast
                        .deps()
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
        Self::new(db, dependencies_section, dev_dependencies_section)
    }
}
