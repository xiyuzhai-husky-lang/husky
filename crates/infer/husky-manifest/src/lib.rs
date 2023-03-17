#![feature(trait_upcasting)]
mod db;
mod dependency;
mod error;
mod has_manifest;
mod manifest;

pub use self::db::*;
pub use self::error::*;
pub use self::has_manifest::*;
pub use self::manifest::*;

use self::dependency::*;
use husky_vfs::*;
use salsa::DbWithJar;

#[salsa::jar(db = ManifestDb)]
pub struct ManifestJar(manifest_dependencies, PackageManifest);
