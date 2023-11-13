#![feature(trait_upcasting)]
mod db;
mod dependency;
mod error;
mod has_manifest;
mod sections;

pub use self::db::*;
pub use self::dependency::*;
pub use self::error::*;
pub use self::has_manifest::*;
pub use self::sections::*;


use husky_corgi_config::HasCorgiConfig;
use husky_manifest_ast::{HasPackageManifestAstSheet, PackageManifestAstSheet};

use husky_vfs::*;
use salsa::{DbWithJar};

#[salsa::jar(db = ManifestDb)]
pub struct ManifestJar(
    package_manifest_aux,
    PackageManifest,
    PackageDependenciesSection,
    package_dependencies_unchecked_aux,
    full_dependent_package_paths_aux,
    cyclic_dependent_package_paths_aux,
    PackageDevDependenciesSection,
    package_dev_dependencies_unchecked,
);

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageManifest {
    // intentially private
    dependencies: PackageDependenciesSection,
    // intentially private
    dev_dependencies: PackageDevDependenciesSection,
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
                .dependencies_section()
                .as_ref()
                .map(|s| s.as_ref().ok())
                .flatten()
                .map(|dev_dependencies_section_ast| {
                    dev_dependencies_section_ast
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
        Self::new(db, dependencies_section, dev_dependencies_section)
    }
}
