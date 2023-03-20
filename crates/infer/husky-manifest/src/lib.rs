#![feature(trait_upcasting)]
mod db;
mod dependency;
mod error;
mod has_manifest;
mod manifest;

pub use self::db::*;
pub use self::dependency::*;
pub use self::error::*;
pub use self::has_manifest::*;
pub use self::manifest::*;

use self::dependency::*;
use husky_vfs::*;
use salsa::DbWithJar;

#[salsa::jar(db = ManifestDb)]
pub struct ManifestJar(
    package_manifest_aux,
    PackageManifest,
    PackageDependenciesSection,
    package_dependencies_aux,
    PackageDevDependenciesSection,
    package_dev_dependencies,
);
